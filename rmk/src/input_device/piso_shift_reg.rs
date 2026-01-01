//! Parallel-In Serial-Out (PISO) shift register async driver for 74HC165 or similar (pulsed parallel load pin, no SPI CS).
//!
//! Snapshot-based: call `update().await` to refresh, then read pin state from the cache.
//! Supports any number of daisy-chained shift registers by specifying the number of bytes to read from the shift register.
//! Contains a sub-struct implementing the `InputPin` trait for individual pins so they can
//! be used with other drivers (e.g., `QuadratureEncoder`).
//! Also offers a shared wrapper around `PisoShiftReg` using an async mutex.
//!
//! This is intended for async scenarios where:
//! - The shift register is shared between tasks and updated via `update().await`, and
//! - Other drivers need to own `InputPin`-style objects that read from the cached snapshot.
use embassy_sync::mutex::Mutex;
use embedded_hal::digital::{Error, ErrorKind, ErrorType, InputPin, OutputPin};
use embedded_hal_async::spi::SpiBus as AsyncSpiBus;

use core::convert::Infallible;
use core::fmt::Debug;

use crate::RawMutex;

/// Driver error: either SPI or GPIO (load pin) failed.
#[derive(Debug)]
pub enum PisoShiftRegError<SpiError, GpioError> {
    Spi(SpiError),
    Gpio(GpioError),
}

/// Parallel-In Serial-Out shift register driver.
///
/// `BITS`  - number of logical input pins exposed
/// `BYTES` - number of bytes read from the shift register (i.e., number of daisy-chained shift register ICs)
/// BITS must be less than or equal to BYTES * 8. Otherwise, a runtime assertion will fail.
/// Pin numbers are zero-indexed, LSB first, and increase from byte to byte, from closest IC in the chain to furthest.
pub struct PisoShiftReg<LoadPin, Spi, const BITS: usize, const BYTES: usize> {
    load_pin: LoadPin,
    spi: Spi,
    state: [u8; BYTES],
}

impl<LoadPin, Spi, const BITS: usize, const BYTES: usize> PisoShiftReg<LoadPin, Spi, BITS, BYTES>
where
    LoadPin: OutputPin,
    Spi: AsyncSpiBus,
{
    /// Create a new driver instance.
    /// Initializes the load pin high.
    /// Internal state is initialized to all zeros.
    /// SPI driver must be configured for MSB first bit order.
    pub fn new(mut load_pin: LoadPin, spi: Spi) -> Result<Self, PisoShiftRegError<Spi::Error, LoadPin::Error>> {
        assert!(BITS <= BYTES * 8);

        load_pin.set_high().map_err(PisoShiftRegError::Gpio)?;

        Ok(Self {
            load_pin,
            spi,
            state: [0; BYTES],
        })
    }

    /// Refresh the internal snapshot from the shift register.
    /// Pulses the load pin to latch inputs, then shifts data into internal state.
    /// The function is async and may yield while waiting for the SPI transfer to complete.
    pub async fn update(&mut self) -> Result<(), PisoShiftRegError<Spi::Error, LoadPin::Error>> {
        self.load_pin.set_low().map_err(PisoShiftRegError::Gpio)?;
        self.load_pin.set_high().map_err(PisoShiftRegError::Gpio)?;

        self.spi.read(&mut self.state).await.map_err(PisoShiftRegError::Spi)
    }

    /// Check if a specific pin is high in the internal snapshot.
    /// Asserts if the pin index is out of range.
    pub fn is_pin_high(&self, pin: usize) -> bool {
        assert!(pin < BITS, "pin index out of range");

        let byte_idx = pin / 8;
        let bit_idx = pin % 8;

        (self.state[byte_idx] & (1 << bit_idx)) != 0
    }

    /// Get a struct implementing the InputPin trait for a specific pin number.
    /// Asserts if the pin index is out of range.
    pub fn pin(&self, pin: usize) -> PisoShiftRegPin<'_, LoadPin, Spi, BITS, BYTES> {
        assert!(pin < BITS, "pin index out of range");

        PisoShiftRegPin { reg: self, idx: pin }
    }

    /// Get an iterator of structs implementing the InputPin trait for all pins.
    /// This borrows `&self`, so InputPin's cannot be held across calls to `update()`.
    /// Example usage:
    ///
    /// ```ignore
    /// loop {
    ///     piso.update().await?;
    ///     let mut pins = piso.all_pins();
    ///     // read pins here
    /// }
    /// ```
    pub fn all_pins(&self) -> impl Iterator<Item = PisoShiftRegPin<'_, LoadPin, Spi, BITS, BYTES>> {
        (0..BITS).map(move |i| PisoShiftRegPin { reg: self, idx: i })
    }

    /// Get a reference to the state array.
    pub fn state(&self) -> &[u8; BYTES] {
        &self.state
    }
}

/// InputPin-style wrapper around a single bit of the cached state.
#[derive(Copy, Clone)]
pub struct PisoShiftRegPin<'a, LoadPin, Spi, const BITS: usize, const BYTES: usize> {
    reg: &'a PisoShiftReg<LoadPin, Spi, BITS, BYTES>,
    idx: usize,
}

/// This reads from cached state and cannot fail.
impl<'a, LoadPin, Spi, const BITS: usize, const BYTES: usize> ErrorType
    for PisoShiftRegPin<'a, LoadPin, Spi, BITS, BYTES>
where
    LoadPin: OutputPin,
    Spi: AsyncSpiBus,
{
    type Error = Infallible;
}

impl<'a, LoadPin, Spi, const BITS: usize, const BYTES: usize> InputPin
    for PisoShiftRegPin<'a, LoadPin, Spi, BITS, BYTES>
where
    LoadPin: OutputPin,
    Spi: AsyncSpiBus,
{
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.reg.is_pin_high(self.idx))
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.reg.is_pin_high(self.idx))
    }
}

/// Error returned by shared InputPin-style wrappers.
#[derive(Debug)]
pub enum SharedPisoPinError {
    /// The shift register is currently locked (e.g., SPI transfer in progress).
    Busy,
}

impl Error for SharedPisoPinError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

/// Shared Parallel-In Serial-Out shift register driver.
/// This driver wraps a `PisoShiftReg` instance in an async mutex to allow safe shared access across tasks.
/// See `PisoShiftReg` for details on the parameters.
pub struct SharedPisoShiftReg<LoadPin, Spi, const BITS: usize, const BYTES: usize>
where
    LoadPin: OutputPin,
    Spi: AsyncSpiBus,
{
    inner: Mutex<RawMutex, PisoShiftReg<LoadPin, Spi, BITS, BYTES>>,
}

impl<LoadPin, Spi, const BITS: usize, const BYTES: usize> SharedPisoShiftReg<LoadPin, Spi, BITS, BYTES>
where
    LoadPin: OutputPin,
    Spi: AsyncSpiBus,
{
    /// Create a new shared driver instance.
    /// See `PisoShiftReg` for details on the parameters.
    pub fn new(load_pin: LoadPin, spi: Spi) -> Result<Self, PisoShiftRegError<Spi::Error, LoadPin::Error>> {
        let reg = PisoShiftReg::new(load_pin, spi)?;
        Ok(Self { inner: Mutex::new(reg) })
    }

    /// Update the underlying driver's internal snapshot.
    ///
    /// Async locks the underlying driver.
    /// See `PisoShiftReg` for details on the parameters.
    pub async fn update(&self) -> Result<(), PisoShiftRegError<Spi::Error, LoadPin::Error>> {
        let mut reg = self.inner.lock().await;
        reg.update().await
    }

    /// Check if a specific pin is high in the underlying driver.
    ///
    /// Returns `SharedPisoPinError::Busy` if the mutex is currently locked and cannot be acquired
    /// without waiting.
    pub fn try_is_pin_high(&self, pin: usize) -> Result<bool, SharedPisoPinError> {
        match self.inner.try_lock() {
            Ok(reg) => Ok(reg.is_pin_high(pin)),
            Err(_err) => Err(SharedPisoPinError::Busy),
        }
    }

    /// Get a struct implementing the InputPin trait for a specific pin number.
    ///
    /// This does not borrow `&self` for the lifetime of the pin. Instead, the
    /// pin internally uses `try_lock()` on each read, so it can be *owned* by
    /// other drivers across calls to `SharedPisoShiftReg::update().await`.
    pub fn pin(&self, pin: usize) -> SharedPisoShiftRegPin<'_, LoadPin, Spi, BITS, BYTES> {
        assert!(pin < BITS, "pin index out of range");

        SharedPisoShiftRegPin {
            reg: &self.inner,
            idx: pin,
        }
    }

    /// Get an array of structs implementing the InputPin trait for all pins.
    ///
    /// Each pin internally uses `try_lock()` on read and can be owned independently
    /// of this `SharedPisoShiftReg` across await points.
    pub fn all_pins(&self) -> impl Iterator<Item = SharedPisoShiftRegPin<'_, LoadPin, Spi, BITS, BYTES>> {
        (0..BITS).map(move |i| SharedPisoShiftRegPin {
            reg: &self.inner,
            idx: i,
        })
    }
}

/// InputPin-style wrapper around a single bit of the cached state for a shared PISO driver.
///
/// This variant owns a reference to the async mutex and uses `try_lock()` on each read.
/// It is suitable for use with drivers that need to own `InputPin`s across calls
/// to `SharedPisoShiftReg::update().await` (e.g., quadrature encoder drivers).
/// Any failed lock attempts return `SharedPisoPinError::Busy`.
pub struct SharedPisoShiftRegPin<'a, LoadPin, Spi, const BITS: usize, const BYTES: usize>
where
    LoadPin: OutputPin,
    Spi: AsyncSpiBus,
{
    reg: &'a Mutex<RawMutex, PisoShiftReg<LoadPin, Spi, BITS, BYTES>>,
    idx: usize,
}

impl<'a, LoadPin, Spi, const BITS: usize, const BYTES: usize> ErrorType
    for SharedPisoShiftRegPin<'a, LoadPin, Spi, BITS, BYTES>
where
    LoadPin: OutputPin,
    Spi: AsyncSpiBus,
{
    type Error = SharedPisoPinError;
}

impl<'a, LoadPin, Spi, const BITS: usize, const BYTES: usize> InputPin
    for SharedPisoShiftRegPin<'a, LoadPin, Spi, BITS, BYTES>
where
    LoadPin: OutputPin,
    Spi: AsyncSpiBus,
{
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        match self.reg.try_lock() {
            Ok(reg) => Ok(reg.is_pin_high(self.idx)),
            Err(_err) => Err(SharedPisoPinError::Busy),
        }
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        self.is_high().map(|v| !v)
    }
}
