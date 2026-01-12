use heapless::Vec;
use rmk_types::action::MidiAction;

pub(crate) struct MidiTranslator {
    cc_table: [[u8; 128]; 16],
    note_on: [[bool; 128]; 16],
}

impl Default for MidiTranslator {
    fn default() -> Self {
        Self::new()
    }
}

impl MidiTranslator {
    pub const fn new() -> Self {
        Self {
            cc_table: [[0; 128]; 16],
            note_on: [[false; 128]; 16],
        }
    }

    pub fn action_to_bytes(&mut self, action: MidiAction, pressed: bool) -> Option<Vec<u8, 3>> {
        let message = self.translate_action(action, pressed)?;
        let mut buffer = [0u8; 3];
        let size = message.copy_to_slice(&mut buffer).ok()?;
        let mut out = Vec::<u8, 3>::new();
        out.extend_from_slice(&buffer[..size]).ok()?;
        Some(out)
    }

    fn translate_action(
        &mut self,
        action: MidiAction,
        pressed: bool,
    ) -> Option<wmidi::MidiMessage<'static>> {
        match action {
            MidiAction::Note {
                channel,
                note,
                velocity,
            } => {
                if pressed {
                    if self.is_note_on(channel, note) {
                        return None;
                    }
                    self.set_note_on(channel, note, true);
                    Some(wmidi::MidiMessage::NoteOn(channel, note, velocity))
                } else {
                    if !self.is_note_on(channel, note) {
                        return None;
                    }
                    self.set_note_on(channel, note, false);
                    let release = wmidi::U7::from_u8_lossy(64);
                    Some(wmidi::MidiMessage::NoteOff(channel, note, release))
                }
            }
            MidiAction::Cc {
                channel,
                cc,
                value,
            } => {
                let value = if pressed { value } else { wmidi::U7::from_u8_lossy(0) };
                Some(wmidi::MidiMessage::ControlChange(channel, cc, value))
            }
            MidiAction::CcStep { channel, cc, delta } => {
                if pressed {
                    let value = self.apply_cc_step(channel, cc, delta);
                    Some(wmidi::MidiMessage::ControlChange(channel, cc, value))
                } else {
                    None
                }
            }
        }
    }

    fn apply_cc_step(
        &mut self,
        channel: wmidi::Channel,
        cc: wmidi::ControlFunction,
        delta: i8,
    ) -> wmidi::ControlValue {
        let channel_index = channel.index() as usize;
        let cc_index = u8::from(cc) as usize;
        let current = self.cc_table[channel_index][cc_index] as i16;
        let next = (current + delta as i16).clamp(0, 127) as u8;
        self.cc_table[channel_index][cc_index] = next;
        wmidi::U7::from_u8_lossy(next)
    }

    fn is_note_on(&self, channel: wmidi::Channel, note: wmidi::Note) -> bool {
        let channel_index = channel.index() as usize;
        let note_index = u8::from(note) as usize;
        self.note_on[channel_index][note_index]
    }

    fn set_note_on(&mut self, channel: wmidi::Channel, note: wmidi::Note, on: bool) {
        let channel_index = channel.index() as usize;
        let note_index = u8::from(note) as usize;
        self.note_on[channel_index][note_index] = on;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_on_off_bytes() {
        let mut translator = MidiTranslator::new();
        let action = MidiAction::Note {
            channel: wmidi::Channel::Ch1,
            note: wmidi::Note::C4,
            velocity: wmidi::Velocity::try_from(100).unwrap(),
        };

        let bytes = translator.action_to_bytes(action, true).unwrap();
        assert_eq!(bytes.as_slice(), [0x90, 60, 100]);

        let bytes = translator.action_to_bytes(action, false).unwrap();
        assert_eq!(bytes.as_slice(), [0x80, 60, 64]);
    }

    #[test]
    fn test_cc_press_release_bytes() {
        let mut translator = MidiTranslator::new();
        let action = MidiAction::Cc {
            channel: wmidi::Channel::Ch2,
            cc: wmidi::ControlFunction::MODULATION_WHEEL,
            value: wmidi::ControlValue::try_from(127).unwrap(),
        };

        let bytes = translator.action_to_bytes(action, true).unwrap();
        assert_eq!(bytes.as_slice(), [0xB1, 1, 127]);

        let bytes = translator.action_to_bytes(action, false).unwrap();
        assert_eq!(bytes.as_slice(), [0xB1, 1, 0]);
    }

    #[test]
    fn test_cc_step_clamp_and_release() {
        let mut translator = MidiTranslator::new();
        let action = MidiAction::CcStep {
            channel: wmidi::Channel::Ch1,
            cc: wmidi::ControlFunction::MODULATION_WHEEL,
            delta: 120,
        };

        let bytes = translator.action_to_bytes(action, true).unwrap();
        assert_eq!(bytes.as_slice(), [0xB0, 1, 120]);

        let bytes = translator.action_to_bytes(action, true).unwrap();
        assert_eq!(bytes.as_slice(), [0xB0, 1, 127]);

        let bytes = translator.action_to_bytes(action, false);
        assert!(bytes.is_none());
    }

    #[test]
    fn test_note_repeat_filtered() {
        let mut translator = MidiTranslator::new();
        let action = MidiAction::Note {
            channel: wmidi::Channel::Ch1,
            note: wmidi::Note::C4,
            velocity: wmidi::Velocity::try_from(64).unwrap(),
        };

        let bytes = translator.action_to_bytes(action, true).unwrap();
        assert_eq!(bytes.as_slice(), [0x90, 60, 64]);

        let bytes = translator.action_to_bytes(action, true);
        assert!(bytes.is_none());

        let bytes = translator.action_to_bytes(action, false).unwrap();
        assert_eq!(bytes.as_slice(), [0x80, 60, 64]);

        let bytes = translator.action_to_bytes(action, false);
        assert!(bytes.is_none());
    }
}
