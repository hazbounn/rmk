use core::sync::atomic::{AtomicBool, Ordering};

use embassy_sync::signal::Signal;
use embassy_time::{Duration, Instant, Timer};
use heapless::Vec;
use trouble_host::prelude::*;

use super::ble_server::{CCCD_TABLE_SIZE, Server};
use crate::RawMutex;

pub(crate) const MIDI_SERVICE_UUID: [u8; 16] = [
    0x00, 0xC7, 0xC4, 0x4E, 0xE3, 0x6C, 0x51, 0xA7, 0x33, 0x4B, 0xE8, 0xED, 0x5A, 0x0E, 0xB8, 0x03,
];
pub(crate) const MIDI_IO_UUID: [u8; 16] = [
    0xF3, 0x6B, 0x10, 0x9D, 0x66, 0xF2, 0xA9, 0xA1, 0x12, 0x41, 0x68, 0x38, 0xDB, 0xE5, 0x72, 0x77,
];

const MIDI_IO_MAX_LEN: usize = 20;
const MIDI_TEST_INTERVAL_MS: u64 = 750;

pub(crate) static MIDI_NOTIFY_STATE: AtomicBool = AtomicBool::new(false);
pub(crate) static MIDI_NOTIFY_SIGNAL: Signal<RawMutex, bool> = Signal::new();

#[gatt_service(uuid = "03b80e5a-ede8-4b33-a751-6ce34ec4c700")]
pub(crate) struct MidiService {
    #[characteristic(uuid = "7772e5db-3868-4112-a1a9-f2669d106bf3", notify, write, write_without_response)]
    pub(crate) midi_io: Vec<u8, MIDI_IO_MAX_LEN>,
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub(crate) enum MidiError {
    PacketTooLarge,
    Ble(Error),
}

impl From<Error> for MidiError {
    fn from(err: Error) -> Self {
        MidiError::Ble(err)
    }
}

pub(crate) struct BleMidiServer<'stack, 'server, 'conn, P: PacketPool> {
    midi_io: Characteristic<Vec<u8, MIDI_IO_MAX_LEN>>,
    conn: &'conn GattConnection<'stack, 'server, P>,
}

impl<'stack, 'server, 'conn, P: PacketPool> BleMidiServer<'stack, 'server, 'conn, P> {
    pub(crate) fn new(server: &Server<'_>, conn: &'conn GattConnection<'stack, 'server, P>) -> Self {
        Self {
            midi_io: server.midi_service.midi_io.clone(),
            conn,
        }
    }

    pub(crate) async fn send_midi(&self, bytes: &[u8]) -> Result<(), MidiError> {
        let mut payload: Vec<u8, MIDI_IO_MAX_LEN> = Vec::new();
        payload
            .extend_from_slice(bytes)
            .map_err(|_| MidiError::PacketTooLarge)?;
        self.midi_io.notify(self.conn, &payload).await.map_err(MidiError::Ble)?;
        Ok(())
    }
}

pub(crate) fn update_midi_notify_state(enabled: bool) {
    MIDI_NOTIFY_STATE.store(enabled, Ordering::Release);
    MIDI_NOTIFY_SIGNAL.signal(enabled);
}

pub(crate) fn sync_midi_notify_state(cccd_table: &CccdTable<CCCD_TABLE_SIZE>, cccd_handle: Option<u16>) {
    let Some(cccd_handle) = cccd_handle else {
        return;
    };
    let mut enabled = false;
    for (handle, value) in cccd_table.inner().iter() {
        if *handle == cccd_handle {
            enabled = value.should_notify();
            break;
        }
    }
    update_midi_notify_state(enabled);
}

async fn wait_for_notify_enabled() {
    if MIDI_NOTIFY_STATE.load(Ordering::Acquire) {
        return;
    }
    loop {
        if MIDI_NOTIFY_SIGNAL.wait().await {
            break;
        }
    }
}

fn build_ble_midi_packet(midi_message: &[u8]) -> Result<Vec<u8, MIDI_IO_MAX_LEN>, MidiError> {
    // 13-bit timestamp, 1ms ticks, wraps at 8192ms
    let ts = (Instant::now().as_millis() as u16) & 0x1FFF;

    // Header: 0b10xxxxxx, where xxxxxx = ts[12:7] (top 6 bits)
    let header = 0x80 | (((ts >> 7) as u8) & 0x3F);

    // Timestamp byte: 0b1xxxxxxx, where xxxxxxx = ts[6:0] (low 7 bits)
    let ts_low = 0x80 | ((ts as u8) & 0x7F);

    let mut packet: Vec<u8, MIDI_IO_MAX_LEN> = Vec::new();
    packet.push(header).map_err(|_| MidiError::PacketTooLarge)?;
    packet.push(ts_low).map_err(|_| MidiError::PacketTooLarge)?;
    packet
        .extend_from_slice(midi_message)
        .map_err(|_| MidiError::PacketTooLarge)?;
    Ok(packet)
}

pub(crate) async fn run_ble_midi_test<'stack, 'server, 'conn, P: PacketPool>(
    server: &Server<'_>,
    conn: &'conn GattConnection<'stack, 'server, P>,
) {
    let midi = BleMidiServer::new(server, conn);
    info!("[midi] connected");
    wait_for_notify_enabled().await;
    info!("[midi] notify enabled");

    const C1_NOTE_ON: [u8; 3] = [0x90, 0x24, 0x40];
    const C1_NOTE_OFF: [u8; 3] = [0x80, 0x24, 0x40];

    let mut note_on = false;
    loop {
        let MIDI_TEST_MESSAGE = if note_on {
            note_on = false;
            C1_NOTE_OFF
        } else {
            note_on = true;
            C1_NOTE_ON
        };
        let packet = match build_ble_midi_packet(&MIDI_TEST_MESSAGE) {
            Ok(packet) => packet,
            Err(e) => {
                warn!("[midi] packet build error: {:?}", e);
                Timer::after(Duration::from_millis(MIDI_TEST_INTERVAL_MS)).await;
                continue;
            }
        };

        match midi.send_midi(&packet).await {
            Ok(()) => info!("[midi] send ok"),
            Err(e) => warn!("[midi] send error: {:?}", e),
        }
        Timer::after(Duration::from_millis(MIDI_TEST_INTERVAL_MS)).await;
    }
}
