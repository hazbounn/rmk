use rmk::types::action::{EncoderAction, KeyAction};
use rmk::{a, encoder, k, layer, mccd, mcci, mn, mo};
use wmidi;
pub(crate) const COL: usize = 4;
pub(crate) const ROW: usize = 6;
pub(crate) const NUM_LAYER: usize = 1;
pub(crate) const NUM_ENCODER: usize = 8;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(A), k!(S), k!(D), k!(F)],
            [k!(Z), k!(X), k!(C), k!(V)],
            [k!(Kc1), k!(Kc2), k!(Kc3), k!(Kc4)],
            [k!(Q), k!(W), k!(E), k!(R)],
            [k!(Z), k!(X), k!(C), k!(V)],
            [mn!(wmidi::Channel::Ch1, wmidi::Note::C4, wmidi::Velocity::MAX), k!(W), k!(E), k!(R)]
        ]),
    ]
}

pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [[
        encoder!(
            mcci!(wmidi::Channel::Ch1, wmidi::ControlFunction::MODULATION_WHEEL, 5),
            mccd!(wmidi::Channel::Ch1, wmidi::ControlFunction::MODULATION_WHEEL, 5)
        ),
        encoder!(
            mcci!(wmidi::Channel::Ch1, wmidi::ControlFunction::BREATH_CONTROLLER, 5),
            mccd!(wmidi::Channel::Ch1, wmidi::ControlFunction::BREATH_CONTROLLER, 5)
        ),
        encoder!(
            mcci!(wmidi::Channel::Ch1, wmidi::ControlFunction::UNDEFINED_3, 5),
            mccd!(wmidi::Channel::Ch1, wmidi::ControlFunction::UNDEFINED_3, 5)
        ),
        encoder!(
            mcci!(wmidi::Channel::Ch1, wmidi::ControlFunction::FOOT_CONTROLLER, 5),
            mccd!(wmidi::Channel::Ch1, wmidi::ControlFunction::FOOT_CONTROLLER, 5)
        ),
        encoder!(
            mcci!(wmidi::Channel::Ch1, wmidi::ControlFunction::PORTAMENTO_TIME, 5),
            mccd!(wmidi::Channel::Ch1, wmidi::ControlFunction::PORTAMENTO_TIME, 5)
        ),
        encoder!(
            mcci!(wmidi::Channel::Ch1, wmidi::ControlFunction::DATA_ENTRY_MSB, 5),
            mccd!(wmidi::Channel::Ch1, wmidi::ControlFunction::DATA_ENTRY_MSB, 5)
        ),
        encoder!(
            mcci!(wmidi::Channel::Ch1, wmidi::ControlFunction::CHANNEL_VOLUME, 5),
            mccd!(wmidi::Channel::Ch1, wmidi::ControlFunction::CHANNEL_VOLUME, 5)
        ),
        encoder!(
            mcci!(wmidi::Channel::Ch1, wmidi::ControlFunction::BALANCE, 5),
            mccd!(wmidi::Channel::Ch1, wmidi::ControlFunction::BALANCE, 5)
        ),
    ]]
}
