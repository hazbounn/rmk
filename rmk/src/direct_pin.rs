use embassy_time::{Instant, Timer};
use embedded_hal;
use embedded_hal::digital::InputPin;
#[cfg(feature = "async_matrix")]
use {embassy_futures::select::select_slice, embedded_hal_async::digital::Wait, heapless::Vec};

use crate::MatrixTrait;
use crate::debounce::{DebounceState, DebouncerTrait};
use crate::event::{Event, KeyboardEvent};
use crate::input_device::InputDevice;
use crate::input_device::piso_shift_reg::SharedPisoShiftReg;
use crate::matrix::KeyState;

/// Hook called once per full scan before reading pins.
pub trait PreScanHook {
    async fn pre_scan(&mut self);
}

impl PreScanHook for () {
    async fn pre_scan(&mut self) {}
}

pub struct PisoPreScan<'a, LoadPin, Spi, const BITS: usize, const BYTES: usize>
where
    LoadPin: embedded_hal::digital::OutputPin,
    Spi: embedded_hal_async::spi::SpiBus,
{
    pub piso: &'a SharedPisoShiftReg<LoadPin, Spi, BITS, BYTES>,
}

impl<'a, LoadPin, Spi, const BITS: usize, const BYTES: usize> PreScanHook for PisoPreScan<'a, LoadPin, Spi, BITS, BYTES>
where
    LoadPin: embedded_hal::digital::OutputPin,
    Spi: embedded_hal_async::spi::SpiBus,
{
    async fn pre_scan(&mut self) {
        let _ = self.piso.update().await;
    }
}

/// DirectPinMartex only has input pins.
pub struct DirectPinMatrix<
    #[cfg(feature = "async_matrix")] In: Wait + InputPin,
    #[cfg(not(feature = "async_matrix"))] In: InputPin,
    D: DebouncerTrait<ROW, COL>,
    const ROW: usize,
    const COL: usize,
    const SIZE: usize,
    H: PreScanHook = (),
> {
    /// Input pins of the pcb matrix
    direct_pins: [[Option<In>; COL]; ROW],
    /// Debouncer
    debouncer: D,
    /// Key state matrix
    key_states: [[KeyState; COL]; ROW],
    /// Start scanning
    scan_start: Option<Instant>,
    /// Pin active level
    low_active: bool,
    /// Hook for refreshing inputs between full scans.
    pre_scan: H,
    /// Current scan pos: (out_idx, in_idx)
    scan_pos: (usize, usize),
}

impl<
    #[cfg(not(feature = "async_matrix"))] In: InputPin,
    #[cfg(feature = "async_matrix")] In: Wait + InputPin,
    D: DebouncerTrait<ROW, COL>,
    const ROW: usize,
    const COL: usize,
    const SIZE: usize,
    H: PreScanHook,
> DirectPinMatrix<In, D, ROW, COL, SIZE, H>
{
    /// Create a matrix from input pins with a pre-scan hook.
    pub fn new_with_hook(direct_pins: [[Option<In>; COL]; ROW], debouncer: D, low_active: bool, pre_scan: H) -> Self {
        DirectPinMatrix {
            direct_pins,
            debouncer,
            key_states: [[KeyState::new(); COL]; ROW],
            scan_start: None,
            low_active,
            pre_scan,
            scan_pos: (0, 0),
        }
    }
}

impl<
    #[cfg(not(feature = "async_matrix"))] In: InputPin,
    #[cfg(feature = "async_matrix")] In: Wait + InputPin,
    D: DebouncerTrait<ROW, COL>,
    const ROW: usize,
    const COL: usize,
    const SIZE: usize,
> DirectPinMatrix<In, D, ROW, COL, SIZE, ()>
{
    /// Create a matrix from input and output pins.
    pub fn new(direct_pins: [[Option<In>; COL]; ROW], debouncer: D, low_active: bool) -> Self {
        DirectPinMatrix {
            direct_pins,
            debouncer,
            key_states: [[KeyState::new(); COL]; ROW],
            scan_start: None,
            low_active,
            pre_scan: (),
            scan_pos: (0, 0),
        }
    }
}

impl<
    #[cfg(not(feature = "async_matrix"))] In: InputPin,
    #[cfg(feature = "async_matrix")] In: Wait + InputPin,
    D: DebouncerTrait<ROW, COL>,
    H: PreScanHook,
    const ROW: usize,
    const COL: usize,
    const SIZE: usize,
> InputDevice for DirectPinMatrix<In, D, ROW, COL, SIZE, H>
{
    async fn read_event(&mut self) -> crate::event::Event {
        loop {
            let (row_idx_start, col_idx_start) = self.scan_pos;
            let fresh_scan = self.scan_pos == (0, 0);

            #[cfg(feature = "async_matrix")]
            self.wait_for_key().await;

            if fresh_scan {
                self.pre_scan.pre_scan().await;
            }

            // Scan matrix and send report
            for row_idx in row_idx_start..self.direct_pins.len() {
                let pins_row = self.direct_pins.get_mut(row_idx).unwrap();
                for col_idx in col_idx_start..pins_row.len() {
                    let direct_pin = pins_row.get_mut(col_idx).unwrap();
                    // for (col_idx, direct_pin) in pins_row.iter_mut().enumerate() {
                    if let Some(direct_pin) = direct_pin {
                        let pin_state = if self.low_active {
                            direct_pin.is_low().ok().unwrap_or_default()
                        } else {
                            direct_pin.is_high().ok().unwrap_or_default()
                        };

                        let debounce_state = self.debouncer.detect_change_with_debounce(
                            row_idx,
                            col_idx,
                            pin_state,
                            &self.key_states[row_idx][col_idx],
                        );

                        if let DebounceState::Debounced = debounce_state {
                            self.key_states[row_idx][col_idx].toggle_pressed();
                            let key_state = self.key_states[row_idx][col_idx];

                            self.scan_pos = (row_idx, col_idx);
                            return Event::Key(KeyboardEvent::key(row_idx as u8, col_idx as u8, key_state.pressed));
                        }

                        // If there's key still pressed, always refresh the self.scan_start
                        #[cfg(feature = "async_matrix")]
                        if self.key_states[row_idx][col_idx].pressed {
                            self.scan_start = Some(Instant::now());
                        }
                    }
                }
            }

            self.scan_pos = (0, 0);

            Timer::after_micros(100).await;
        }
    }
}

impl<
    #[cfg(not(feature = "async_matrix"))] In: InputPin,
    #[cfg(feature = "async_matrix")] In: Wait + InputPin,
    D: DebouncerTrait<ROW, COL>,
    const ROW: usize,
    const COL: usize,
    const SIZE: usize,
    H: PreScanHook,
> MatrixTrait<ROW, COL> for DirectPinMatrix<In, D, ROW, COL, SIZE, H>
{
    #[cfg(feature = "async_matrix")]
    async fn wait_for_key(&mut self) {
        use core::pin::pin;

        if let Some(start_time) = self.scan_start {
            // If no key press over 1ms, stop scanning and wait for interupt
            if start_time.elapsed().as_millis() <= 1 {
                return;
            } else {
                self.scan_start = None;
            }
        }
        Timer::after_micros(1).await;
        info!("Waiting for active level");

        if self.low_active {
            let mut futs: Vec<_, SIZE> = Vec::new();
            for direct_pins_row in self.direct_pins.iter_mut() {
                for direct_pin in direct_pins_row.iter_mut().flatten() {
                    let _ = futs.push(direct_pin.wait_for_low());
                }
            }
            let _ = select_slice(pin!(futs.as_mut_slice())).await;
        } else {
            let mut futs: Vec<_, SIZE> = Vec::new();
            for direct_pins_row in self.direct_pins.iter_mut() {
                for direct_pin in direct_pins_row.iter_mut().flatten() {
                    let _ = futs.push(direct_pin.wait_for_high());
                }
            }
            let _ = select_slice(pin!(futs.as_mut_slice())).await;
        }
        self.scan_start = Some(Instant::now());
    }
}
