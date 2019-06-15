use minimad::{Line, TableAlignments};

use crate::skin::MadSkin;
use crate::tbl::{FmtTableRow, FmtTableRule, RelativePosition};
use crate::composite::FmtComposite;

/// A line in a text. This structure should normally not be
/// used outside of the lib.
pub enum FmtLine<'s> {
    Normal(FmtComposite<'s>),
    TableRow(FmtTableRow<'s>),
    TableRule(FmtTableRule),
}

impl<'s> FmtLine<'s> {
    /// build a fmtline from a minimad line.
    /// skin is passed because it might affect the visible size
    /// in the future
    pub fn from(mline: Line<'s>, skin: & MadSkin) -> FmtLine<'s> {
        match mline {
            Line::Normal(composite) => FmtLine::Normal(
                FmtComposite::from(composite, skin)
            ),
            Line::TableRow(table_row) => FmtLine::TableRow(
                FmtTableRow::from(table_row, skin)
            ),
            Line::TableAlignments(TableAlignments{cells}) => FmtLine::TableRule(
                FmtTableRule {
                    position: RelativePosition::Other,
                    widths: Vec::new(),
                    aligns: cells,
                }
            )
        }
    }
    pub fn visible_length(&self) -> usize {
        match self {
            FmtLine::Normal(composite) => composite.visible_length,
            FmtLine::TableRow(row) => row.cells.iter().fold(0, |s, c| s + c.visible_length),
            FmtLine::TableRule(rule) => 1 + rule.widths.iter().fold(0, |s, w| s + w + 1),
        }
    }
}


