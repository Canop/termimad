
use crate::skin::MadSkin;
use crate::spacing::Spacing;

use minimad::{Compound, Composite};

/// wrap a Minimad Composite, which is a list of Compounds,
/// which are strings with an homogeneous style
#[derive(Debug, Clone)]
pub struct FmtComposite<'s> {
    pub composite: Composite<'s>,
    pub visible_length: usize, // to avoid recomputing it again and again
    pub spacing: Option<Spacing>,
}

impl<'s> FmtComposite<'s> {
    pub fn new() -> Self {
        FmtComposite {
            composite: Composite::new(),
            visible_length: 0,
            spacing: None,
        }
    }
    pub fn from(composite: Composite<'s>, skin: &MadSkin) -> Self {
        FmtComposite {
            visible_length: skin.visible_composite_length(&composite),
            composite,
            spacing: None,
        }
    }
    /// return the number of characters (usually spaces) to insert both
    /// sides of the composite
    #[inline(always)]
    pub fn completions(&self) -> (usize, usize) {
        match &self.spacing {
            Some(spacing) => spacing.completions_for(self.visible_length),
            None => (0, 0),
        }
    }
    /// add a compound and modifies `visible_length` accordingly
    #[inline(always)]
    pub fn add_compound(&mut self, compound: Compound<'s>) {
        self.visible_length += compound.char_length();
        self.composite.compounds.push(compound);
    }
}

