
use crate::skin::MadSkin;
use minimad::{Alignment, Compound, Composite};

#[derive(Clone, Copy)]
pub struct Spacing {
    pub width: usize,
    pub align: Alignment,
}

/// wrap a Minimad Composite, which is a list of Compounds,
/// which are strings with an homogeneous style
pub struct FmtComposite<'s> {
    pub composite: Composite<'s>,
    pub visible_length: usize, // to avoid recomputing it again and again
    pub spacing: Option<Spacing>,
}

impl<'s> FmtComposite<'s> {
    pub fn from(composite: Composite<'s>, skin: &MadSkin) -> FmtComposite<'s> {
        FmtComposite {
            visible_length: skin.visible_composite_length(&composite),
            composite,
            spacing: None,
        }
    }
    // return the number of characters (usually spaces) to insert both
    // sides of the composite
    pub fn completions(&self) -> (usize, usize) {
        match &self.spacing {
            Some(spacing) => match spacing.align {
                Alignment::Left | Alignment::Unspecified => (0, spacing.width - self.visible_length),
                Alignment::Center => {
                    let lp = (spacing.width - self.visible_length) / 2;
                    (lp, spacing.width - lp)
                },
                Alignment::Right => (spacing.width - self.visible_length, 0),
            },
            None => (0, 0),
        }
    }
    pub fn add_compound(&mut self, compound: Compound<'s>) {
        self.visible_length += compound.char_length();
        self.composite.compounds.push(compound);
    }
}

