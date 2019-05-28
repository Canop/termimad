use std::fmt;

use crate::composite::FmtComposite;
use crate::skin::MadSkin;

pub struct FmtInline<'k, 's> {
    pub skin: &'k MadSkin,
    pub composite: FmtComposite<'s>,
}

impl fmt::Display for FmtInline<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.skin.write_fmt_composite(f, &self.composite)
    }
}
