use crate::action::action::Action;
use crate::common::color::ODColor;
use crate::common::palette::{ObjectPalette, Palette, PaletteColorIndex};

pub struct PaletteColorChangeAction {
    palette: ObjectPalette,
    idx: PaletteColorIndex,
    before_color: ODColor,
    after_color: ODColor,
}
