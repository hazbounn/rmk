use rmk::types::action::{EncoderAction, KeyAction};
use rmk::{a, encoder, k, layer, mn, mo};
use wmidi;
pub(crate) const COL: usize = 4;
pub(crate) const ROW: usize = 6;
pub(crate) const NUM_LAYER: usize = 1;
pub(crate) const NUM_ENCODER: usize = 0;

#[rustfmt::skip]
pub const fn get_default_keymap() -> [[[KeyAction; COL]; ROW]; NUM_LAYER] {
    [
        layer!([
            [k!(Q), k!(W), k!(E), k!(R)],
            [k!(Q), k!(W), k!(E), k!(R)],
            [k!(Q), k!(W), k!(E), k!(R)],
            [k!(Q), k!(W), k!(E), k!(R)],
            [k!(Q), k!(W), k!(E), k!(R)],
            [mn!(wmidi::Channel::Ch1, wmidi::Note::C4, wmidi::Velocity::MAX), k!(W), k!(E), k!(R)]
        ]),
    ]
}

pub const fn get_default_encoder_map() -> [[EncoderAction; NUM_ENCODER]; NUM_LAYER] {
    [[]]
}
