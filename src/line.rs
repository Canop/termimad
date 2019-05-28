use std::fmt;

use minimad::{Alignment, Line, TableAlignments};

use crate::skin::MadSkin;
use crate::tbl::{FmtTableRow, FmtTableRule, RelativePosition};
use crate::composite::FmtComposite;

pub enum FmtLine<'s> {
    Normal(FmtComposite<'s>),
    TableRow(FmtTableRow<'s>),
    TableRule(FmtTableRule),
}

impl<'s> FmtLine<'s> {
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
}


