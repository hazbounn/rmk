use rmk::types::action::{EncoderAction, KeyAction};
use rmk::types::modifier::ModifierCombination;
use rmk::{encoder, k, layer, mccd, mcci, mn, mnto, mo, mto, wm};
use wmidi;
use wmidi::Channel::Ch1;
use wmidi::ControlFunction as MidiCC;
use wmidi::Note::{A4, Ab4, Bb4, C4, C5, D4, D5, Db4, Db5, E4, Eb4, F4, G4, Gb4};
use wmidi::Velocity;
pub(crate) const COL: usize = 4;
pub(crate) const ROW: usize = 6;
pub(crate) const NUM_LAYER: usize = 8;
pub(crate) const NUM_ENCODER: usize = 8;

const DEFAULT_CC_INCREMENT: i8 = 1i8;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        // Sample layer
        layer!([
            [k!(A), k!(S), k!(D), k!(F)],
            [k!(Z), k!(X), k!(C), k!(V)],
            [k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4)],
            [k!(Q), k!(W), k!(E), k!(R)],
            [mto!(0, 0), mto!(1, 2), mto!(2, 4), mn!(Ch1, C4, Velocity::MAX)],
            [mo!(1), k!(Space), k!(RightBracket), k!(N)]
        ]),
        // Sample Modifier layer
        layer!([
            [mn!(Ch1, Db4, Velocity::MAX), mn!(Ch1, D4, Velocity::MAX), mn!(Ch1, Eb4, Velocity::MAX), mn!(Ch1, E4, Velocity::MAX)],
            [mn!(Ch1, F4, Velocity::MAX), mn!(Ch1, Gb4, Velocity::MAX), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(Z), k!(Z), k!(Z), k!(Backspace)],
            [k!(No), k!(Kc0), k!(LeftBracket), k!(B)]
        ]),
        // Sequence Layer
        layer!([
            [k!(A), k!(S), k!(D), k!(F)],
            [k!(Z), k!(X), k!(C), k!(V)],
            [k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4)],
            [k!(Q), k!(W), k!(E), k!(R)],
            [mto!(0, 0), mto!(1, 2), mto!(2, 4), mn!(Ch1, C4, Velocity::MAX)],
            [mo!(3), k!(Space), k!(RightBracket), k!(N)]
        ]),
        // Sequence Modifier Layer
        layer!([
            [mn!(Ch1, G4, Velocity::MAX), mn!(Ch1, Ab4, Velocity::MAX), k!(Equal), k!(Minus)],
            [mnto!(Ch1, A4, Velocity::MAX, 6), k!(No), k!(No), k!(No)],
            [k!(Y), k!(U), k!(I), k!(O)],
            [k!(H), k!(J), k!(K), k!(L)],
            [k!(No), k!(No), k!(No), mn!(Ch1, C4, Velocity::MAX)],
            [k!(No), k!(Space), k!(LeftBracket), k!(N)]
        ]),
        // Perform layer
        layer!([
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(Y), k!(U), k!(I), k!(O)],
            [k!(H), k!(J), k!(K), k!(L)],
            [mto!(0, 0), mto!(1, 2), mto!(2, 4), mn!(Ch1, C4, Velocity::MAX)],
            [mo!(5), k!(Space), wm!(RightBracket, ModifierCombination::new_from(false, false, false, true, false)), k!(N)]
        ]),
        // Perform Modifier layer
        layer!([
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No)],
            [mto!(0, 0), mto!(1, 2), mto!(2, 4), mn!(Ch1, C4, Velocity::MAX)],
            [k!(No), k!(Space), wm!(LeftBracket, ModifierCombination::new_from(false, false, false, true, false)), k!(N)]
        ]),
        // keyboard mode layer
        layer!([
            [k!(D), k!(F), k!(T), k!(G)],
            [k!(A), k!(W), k!(S), k!(E)],
            [k!(K), k!(O), k!(L), k!(P)],
            [k!(Y), k!(H), k!(U), k!(J)],
            [k!(No), k!(No), k!(No), k!(No)],
            [mo!(7), k!(No), k!(No), k!(No)]
        ]),
        layer!([
            [k!(No), k!(No), k!(No), k!(No)],
            [mnto!(Ch1, A4, Velocity::MAX, 2), k!(Z), k!(X), k!(No)],
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No)]
        ]),
    ]
}

pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [
        [
            encoder!(
                mcci!(Ch1, MidiCC::MODULATION_WHEEL, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::MODULATION_WHEEL, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::BREATH_CONTROLLER, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::BREATH_CONTROLLER, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::UNDEFINED_3, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::UNDEFINED_3, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::FOOT_CONTROLLER, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::FOOT_CONTROLLER, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::PORTAMENTO_TIME, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::PORTAMENTO_TIME, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::DATA_ENTRY_MSB, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::DATA_ENTRY_MSB, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::CHANNEL_VOLUME, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::CHANNEL_VOLUME, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::BALANCE, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::BALANCE, DEFAULT_CC_INCREMENT)
            ),
        ],
        [
            encoder!(
                mcci!(Ch1, MidiCC::MODULATION_WHEEL, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::MODULATION_WHEEL, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::BREATH_CONTROLLER, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::BREATH_CONTROLLER, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::UNDEFINED_3, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::UNDEFINED_3, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::FOOT_CONTROLLER, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::FOOT_CONTROLLER, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::PORTAMENTO_TIME, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::PORTAMENTO_TIME, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::DATA_ENTRY_MSB, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::DATA_ENTRY_MSB, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::CHANNEL_VOLUME, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::CHANNEL_VOLUME, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::BALANCE, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::BALANCE, DEFAULT_CC_INCREMENT)
            ),
        ],
        // No-op Sequence layer
        [
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
        ],
        [
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
        ],
        [
            encoder!(
                mcci!(Ch1, MidiCC::UNDEFINED_24, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::UNDEFINED_24, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::UNDEFINED_9, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::UNDEFINED_9, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::PAN, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::PAN, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::EXPRESSION_CONTROLLER, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::EXPRESSION_CONTROLLER, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::EFFECT_CONTROL_1, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::EFFECT_CONTROL_1, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::EFFECT_CONTROL_2, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::EFFECT_CONTROL_2, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::UNDEFINED_14, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::UNDEFINED_14, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::UNDEFINED_15, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::UNDEFINED_15, DEFAULT_CC_INCREMENT)
            ),
        ],
        [
            encoder!(
                mcci!(Ch1, MidiCC::GENERAL_PURPOSE_CONTROLLER_1, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::GENERAL_PURPOSE_CONTROLLER_1, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::GENERAL_PURPOSE_CONTROLLER_2, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::GENERAL_PURPOSE_CONTROLLER_2, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::GENERAL_PURPOSE_CONTROLLER_3, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::GENERAL_PURPOSE_CONTROLLER_3, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::GENERAL_PURPOSE_CONTROLLER_4, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::GENERAL_PURPOSE_CONTROLLER_4, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::UNDEFINED_20, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::UNDEFINED_20, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::UNDEFINED_21, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::UNDEFINED_21, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::UNDEFINED_22, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::UNDEFINED_22, DEFAULT_CC_INCREMENT)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::UNDEFINED_23, DEFAULT_CC_INCREMENT),
                mccd!(Ch1, MidiCC::UNDEFINED_23, DEFAULT_CC_INCREMENT)
            ),
        ],
        // No-op keyboard layer
        [
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
        ],
        [
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
            encoder!(k!(No), k!(No)),
        ],
    ]
}
