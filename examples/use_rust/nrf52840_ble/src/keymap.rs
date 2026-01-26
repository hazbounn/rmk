use rmk::types::action::{EncoderAction, KeyAction};
use rmk::{encoder, k, layer, mccd, mcci, mn, mo, to};
use wmidi;
use wmidi::Channel::Ch1;
use wmidi::ControlFunction as MidiCC;
use wmidi::Note::{A4, Ab4, Bb4, C4, C5, D4, D5, Db4, Db5, E4, Eb4, F4, G4, Gb4};
use wmidi::Velocity;
pub(crate) const COL: usize = 4;
pub(crate) const ROW: usize = 6;
pub(crate) const NUM_LAYER: usize = 6;
pub(crate) const NUM_ENCODER: usize = 8;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        // Sample layer
        layer!([
            [k!(A), k!(S), k!(D), k!(F)],
            [k!(Z), k!(X), k!(C), k!(V)],
            [k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4)],
            [k!(Q), k!(W), k!(E), k!(R)],
            [to!(0), to!(2), to!(4), mn!(Ch1, C4, Velocity::MAX)],
            [mo!(1), k!(Space), k!(Right), k!(N)]
        ]),
        // Sample Modifier layer
        layer!([
            [mn!(Ch1, Db4, Velocity::MAX), mn!(Ch1, D4, Velocity::MAX), mn!(Ch1, Eb4, Velocity::MAX), mn!(Ch1, E4, Velocity::MAX)],
            [mn!(Ch1, F4, Velocity::MAX), mn!(Ch1, Gb4, Velocity::MAX), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(Z), k!(Z), k!(Z), k!(Backspace)],
            [k!(No), k!(Kc0), k!(Left), k!(B)]
        ]),
        // Sequence Layer
        layer!([
            [k!(A), k!(S), k!(D), k!(F)],
            [k!(Z), k!(X), k!(C), k!(V)],
            [k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4)],
            [k!(Q), k!(W), k!(E), k!(R)],
            [to!(0), to!(2), to!(4), mn!(Ch1, C4, Velocity::MAX)],
            [mo!(3), k!(Space), k!(Right), k!(N)]
        ]),
        // Sequence Modifier Layer
        layer!([
            [mn!(Ch1, G4, Velocity::MAX), mn!(Ch1, Ab4, Velocity::MAX), k!(Equal), k!(Minus)],
            [mn!(Ch1, A4, Velocity::MAX), k!(Z), k!(X), k!(No)],
            [k!(Y), k!(U), k!(I), k!(O)],
            [k!(H), k!(J), k!(K), k!(L)],
            [k!(No), k!(No), k!(No), mn!(Ch1, C4, Velocity::MAX)],
            [k!(No), k!(Space), k!(Right), k!(N)]
        ]),
        // Perform layer
        layer!([
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(Y), k!(U), k!(I), k!(O)],
            [k!(H), k!(J), k!(K), k!(L)],
            [to!(0), to!(2), to!(4), mn!(Ch1, C4, Velocity::MAX)],
            [mo!(5), k!(Space), k!(Right), k!(N)]
        ]),
        // Perform Modifier layer
        layer!([
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No)],
            [k!(No), k!(No), k!(No), k!(No)],
            [to!(0), to!(2), to!(4), mn!(Ch1, C4, Velocity::MAX)],
            [k!(No), k!(Space), k!(Right), k!(N)]
        ]),
    ]
}

pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [
        [
            encoder!(
                mcci!(Ch1, MidiCC::MODULATION_WHEEL, 5),
                mccd!(Ch1, MidiCC::MODULATION_WHEEL, 5)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::BREATH_CONTROLLER, 5),
                mccd!(Ch1, MidiCC::BREATH_CONTROLLER, 5)
            ),
            encoder!(mcci!(Ch1, MidiCC::UNDEFINED_3, 5), mccd!(Ch1, MidiCC::UNDEFINED_3, 5)),
            encoder!(
                mcci!(Ch1, MidiCC::FOOT_CONTROLLER, 5),
                mccd!(Ch1, MidiCC::FOOT_CONTROLLER, 5)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::PORTAMENTO_TIME, 5),
                mccd!(Ch1, MidiCC::PORTAMENTO_TIME, 5)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::DATA_ENTRY_MSB, 5),
                mccd!(Ch1, MidiCC::DATA_ENTRY_MSB, 5)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::CHANNEL_VOLUME, 5),
                mccd!(Ch1, MidiCC::CHANNEL_VOLUME, 5)
            ),
            encoder!(mcci!(Ch1, MidiCC::BALANCE, 5), mccd!(Ch1, MidiCC::BALANCE, 5)),
        ],
        [
            encoder!(
                mcci!(Ch1, MidiCC::MODULATION_WHEEL, 5),
                mccd!(Ch1, MidiCC::MODULATION_WHEEL, 5)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::BREATH_CONTROLLER, 5),
                mccd!(Ch1, MidiCC::BREATH_CONTROLLER, 5)
            ),
            encoder!(mcci!(Ch1, MidiCC::UNDEFINED_3, 5), mccd!(Ch1, MidiCC::UNDEFINED_3, 5)),
            encoder!(
                mcci!(Ch1, MidiCC::FOOT_CONTROLLER, 5),
                mccd!(Ch1, MidiCC::FOOT_CONTROLLER, 5)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::PORTAMENTO_TIME, 5),
                mccd!(Ch1, MidiCC::PORTAMENTO_TIME, 5)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::DATA_ENTRY_MSB, 5),
                mccd!(Ch1, MidiCC::DATA_ENTRY_MSB, 5)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::CHANNEL_VOLUME, 5),
                mccd!(Ch1, MidiCC::CHANNEL_VOLUME, 5)
            ),
            encoder!(mcci!(Ch1, MidiCC::BALANCE, 5), mccd!(Ch1, MidiCC::BALANCE, 5)),
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
            encoder!(mcci!(Ch1, MidiCC::BALANCE, 5), mccd!(Ch1, MidiCC::BALANCE, 5)),
            encoder!(mcci!(Ch1, MidiCC::UNDEFINED_9, 5), mccd!(Ch1, MidiCC::UNDEFINED_9, 5)),
            encoder!(mcci!(Ch1, MidiCC::PAN, 5), mccd!(Ch1, MidiCC::PAN, 5)),
            encoder!(
                mcci!(Ch1, MidiCC::EXPRESSION_CONTROLLER, 5),
                mccd!(Ch1, MidiCC::EXPRESSION_CONTROLLER, 5)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::EFFECT_CONTROL_1, 5),
                mccd!(Ch1, MidiCC::EFFECT_CONTROL_1, 5)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::EFFECT_CONTROL_2, 5),
                mccd!(Ch1, MidiCC::EFFECT_CONTROL_2, 5)
            ),
            encoder!(mcci!(Ch1, MidiCC::UNDEFINED_14, 5), mccd!(Ch1, MidiCC::UNDEFINED_14, 5)),
            encoder!(mcci!(Ch1, MidiCC::UNDEFINED_15, 5), mccd!(Ch1, MidiCC::UNDEFINED_15, 5)),
        ],
        [
            encoder!(
                mcci!(Ch1, MidiCC::GENERAL_PURPOSE_CONTROLLER_1, 5),
                mccd!(Ch1, MidiCC::GENERAL_PURPOSE_CONTROLLER_1, 5)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::GENERAL_PURPOSE_CONTROLLER_2, 5),
                mccd!(Ch1, MidiCC::GENERAL_PURPOSE_CONTROLLER_2, 5)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::GENERAL_PURPOSE_CONTROLLER_3, 5),
                mccd!(Ch1, MidiCC::GENERAL_PURPOSE_CONTROLLER_3, 5)
            ),
            encoder!(
                mcci!(Ch1, MidiCC::GENERAL_PURPOSE_CONTROLLER_4, 5),
                mccd!(Ch1, MidiCC::GENERAL_PURPOSE_CONTROLLER_4, 5)
            ),
            encoder!(mcci!(Ch1, MidiCC::UNDEFINED_20, 5), mccd!(Ch1, MidiCC::UNDEFINED_20, 5)),
            encoder!(mcci!(Ch1, MidiCC::UNDEFINED_21, 5), mccd!(Ch1, MidiCC::UNDEFINED_21, 5)),
            encoder!(mcci!(Ch1, MidiCC::UNDEFINED_22, 5), mccd!(Ch1, MidiCC::UNDEFINED_22, 5)),
            encoder!(mcci!(Ch1, MidiCC::UNDEFINED_23, 5), mccd!(Ch1, MidiCC::UNDEFINED_23, 5)),
        ],
    ]
}
