
use {
    crate::{
        composite::*,
        line::FmtLine,
        skin::MadSkin,
        spacing::Spacing,
        fit::wrap,
    },
    minimad::{Alignment, TableRow},
    std::cmp,
};


/// Wrap a standard table row
#[derive(Debug)]
pub struct FmtTableRow<'s> {
    pub cells: Vec<FmtComposite<'s>>,
}

/// Top, Bottom, or other
#[derive(Debug)]
pub enum RelativePosition {
    Top,
    Other, // or unknown
    Bottom,
}

/// A separator or alignment rule in a table.
///
/// Represent this kind of lines in tables:
///  |----|:-:|--
#[derive(Debug)]
pub struct FmtTableRule {
    pub position: RelativePosition, // position relative to the table
    pub widths: Vec<usize>,
    pub aligns: Vec<Alignment>,
}

impl FmtTableRule {
    pub fn set_nbcols(&mut self, nbcols: usize) {
        self.widths.truncate(nbcols);
        self.aligns.truncate(nbcols);
        for ic in 0..nbcols {
            if ic >= self.widths.len() {
                self.widths.push(0);
            }
            if ic >= self.aligns.len() {
                self.aligns.push(Alignment::Unspecified);
            }
        }
    }
}

impl<'s> FmtTableRow<'s> {
    pub fn from(table_row: TableRow<'s>, skin: &MadSkin) -> FmtTableRow<'s> {
        let mut table_row = table_row;
        FmtTableRow {
            cells: table_row
                .cells
                .drain(..)
                .map(|composite| FmtComposite::from(composite, skin))
                .collect(),
        }
    }
}

/// Tables are the sequences of lines whose line style is TableRow.
///
/// A table is just the indices, without the text
/// This structure isn't public because the indices are invalid as
///  soon as rows are inserted. It only serves during the formatting
///  process.
struct Table {
    start: usize,
    height: usize, // number of lines
    nbcols: usize, // number of columns
}

// an internal struct used during col resizing
#[derive(Debug)]
struct Col {
    idx: usize,       // index of the col
    width: usize,     // col internal width
    to_remove: usize, // what should be removed
}

/// Determine suitable columns width from the current one and the
/// overall sum goal.
/// No width can go below 3.
/// This function should be called only when the goal is attainable
/// and when there's reduction to be done.
fn reduce_col_widths(widths: &mut Vec<usize>, goal: usize) {
    let sum: usize = widths.iter().sum();
    assert!(sum > goal);

    //- simple case 1 : there's only one col
    if widths.len()==1 {
        widths[0] = goal;
        return;
    }

    let mut cols: Vec<Col> = widths
        .iter()
        .enumerate()
        .map(|(idx, width)| {
            Col {
                idx,
                width: *width,
                to_remove: 0,
            }
        })
        .collect();
    cols.sort_by_key(|c| cmp::Reverse(c.width));

    let mut excess = sum - goal;

    // we do a first reduction, if possible, on columns wider
    // than 5
    let excess_of_wide_cols: usize = widths.iter()
        .filter(|&w| *w > 5)
        .map(|w| w - 5)
        .sum();
    if excess_of_wide_cols + goal > sum {
        for col in &mut cols {
            if col.width > 5 {
                let r = (sum-goal) * col.width / excess_of_wide_cols;
                let r = r.min(excess).min(col.width-5);
                excess -= r;
                col.to_remove += r;
            } else {
                break; // due to sort
            }
        }
    }

    if excess > 0 {
        for col in cols.iter_mut() {
            let w = col.width - col.to_remove;
            if w > 3 {
                let dr = (w * excess / sum).min(w - 3);
                col.to_remove += dr;
                excess -= dr;
            };
        }
    }

    cols.sort_by(|a, b| b.to_remove.cmp(&a.to_remove));

    //- general case, which could be improved
    for col in &mut cols {
        if col.to_remove < 3 {
            excess += col.to_remove;
            col.to_remove = 0;
        }
    }
    while excess > 0 {
        let mut nb_changed = 0;
        for col in &mut cols {
            if col.width - col.to_remove > 3 {
                col.to_remove += 1;
                excess -= 1;
                nb_changed += 1;
                if excess == 0 {
                    break;
                }
            }
        }
        if nb_changed == 0 {
            break;
        }
    }
    for c in cols {
        widths[c.idx] -= c.to_remove;
    }
}

impl Table {
    pub fn fix_columns(&mut self, lines: &mut Vec<FmtLine<'_>>, width: usize) {
        let mut nbcols = self.nbcols;
        if nbcols == 0 || width == 0 {
            return;
        }
        // let's first compute the initial widths of all columns
        // (not counting the widths of the borders)
        // We also add the missing cells
        let mut widths: Vec<usize> = vec![0; nbcols];
        for ir in self.start..self.start + self.height {
            let line = &mut lines[ir];
            if let FmtLine::TableRow(FmtTableRow { cells }) = line {
                for ic in 0..nbcols {
                    if cells.len() <= ic {
                        cells.push(FmtComposite::new());
                    } else {
                        widths[ic] = widths[ic].max(cells[ic].visible_length);
                    }
                }
            } else if let FmtLine::TableRule(rule) = line {
                rule.set_nbcols(nbcols);
            } else {
                println!("not a table row, should not happen"); // should we panic ?
            }
        }
        // let's find what we must do
        let widths_sum: usize = widths.iter().sum();
        let mut cols_removed = false;
        if widths_sum + nbcols < width {
            // it fits, all is well
        } else if nbcols * 4 < width {
            // we can keep all columns but we'll have to wrap them
            reduce_col_widths(&mut widths, width - nbcols - 1);
        } else {
            // crisis behavior: we remove the columns which don't fit
            nbcols = (width - 1) / 4;
            cols_removed = true;
            for ic in 0..nbcols {
                widths[ic] = 3;
            }
        }

        // Now we resize all cells and we insert new rows if necessary.
        // We iterate in reverse order so that we can insert rows
        //  without recomputing row indices.
        for ir in (self.start..self.start + self.height).rev() {
            let line = &mut lines[ir];
            if let FmtLine::TableRow(FmtTableRow { cells }) = line {
                let mut cells_to_add: Vec<Vec<FmtComposite<'_>>> = Vec::new();
                cells.truncate(nbcols);
                for ic in 0..nbcols {
                    if cells.len() <= ic {
                        //FIXME isn't this already done ?
                        cells.push(FmtComposite::new());
                        continue;
                    }
                    cells_to_add.push(Vec::new());
                    if cells[ic].visible_length > widths[ic] {
                        // we must wrap the cell over several lines
                        let mut composites = wrap::hard_wrap_composite(&cells[ic], widths[ic]);
                        // the first composite replaces the cell, while the other
                        // ones go to cells_to_add
                        let mut drain = composites.drain(..);
                        cells[ic] = drain.next().unwrap();
                        for c in drain {
                            cells_to_add[ic].push(c);
                        }
                    }
                }
                let nb_new_lines = cells_to_add.iter().fold(0, |m, cells| m.max(cells.len()));
                for inl in (0..nb_new_lines).rev() {
                    let mut new_cells: Vec<FmtComposite<'_>> = Vec::new();
                    for ic in 0..nbcols {
                        new_cells.push(if cells_to_add[ic].len() > inl {
                            cells_to_add[ic].remove(inl)
                        } else {
                            FmtComposite::new()
                        });
                    }
                    let new_line = FmtLine::TableRow(FmtTableRow { cells: new_cells });
                    lines.insert(ir + 1, new_line);
                    self.height += 1;
                }
            }
        }
        // Finally we iterate in normal order to specify alignment
        // (the alignments of a row are the ones of the last rule line)
        let mut current_aligns: Vec<Alignment> = vec![Alignment::Center; nbcols];
        for ir in self.start..self.start + self.height {
            let line = &mut lines[ir];
            match line {
                FmtLine::TableRow(FmtTableRow { cells }) => {
                    for ic in 0..nbcols {
                        cells[ic].spacing = Some(Spacing {
                            width: widths[ic],
                            align: current_aligns[ic],
                        });
                    }
                }
                FmtLine::TableRule(rule) => {
                    if cols_removed {
                        rule.set_nbcols(nbcols);
                    }
                    if ir == self.start {
                        rule.position = RelativePosition::Top;
                    } else if ir == self.start + self.height - 1 {
                        rule.position = RelativePosition::Bottom;
                    }
                    rule.widths[..nbcols].clone_from_slice(&widths[..nbcols]);
                    current_aligns[..nbcols].clone_from_slice(&rule.aligns[..nbcols]);
                }
                _ => {
                    panic!("It should be a table part");
                }
            }
        }
    }
}

/// find the positions of all tables
fn find_tables(lines: &[FmtLine<'_>]) -> Vec<Table> {
    let mut tables: Vec<Table> = Vec::new();
    let mut current: Option<Table> = None;
    for (idx, line) in lines.iter().enumerate() {
        match line {
            FmtLine::TableRule(FmtTableRule { aligns, .. }) => match current.as_mut() {
                Some(b) => {
                    b.height += 1;
                    b.nbcols = b.nbcols.max(aligns.len());
                }
                None => {
                    current = Some(Table {
                        start: idx,
                        height: 1,
                        nbcols: aligns.len(),
                    });
                }
            },
            FmtLine::TableRow(FmtTableRow { cells }) => match current.as_mut() {
                Some(b) => {
                    b.height += 1;
                    b.nbcols = b.nbcols.max(cells.len());
                }
                None => {
                    current = Some(Table {
                        start: idx,
                        height: 1,
                        nbcols: cells.len(),
                    });
                }
            },
            _ => {
                if let Some(c) = current.take() {
                    tables.push(c);
                }
            }
        }
    }
    if let Some(c) = current.take() {
        tables.push(c);
    }
    tables
}

/// Modify the rows of all tables in order to ensure it fits the widths
/// and all cells have the widths of their column.
///
/// Some lines may be added to the table in the process, which means any
///  precedent indexing might be invalid.
pub fn fix_all_tables(lines: &mut Vec<FmtLine<'_>>, width: usize) {
    for tbl in find_tables(lines).iter_mut().rev() {
        tbl.fix_columns(lines, width);
    }
}

#[cfg(test)]
mod col_reduction_tests {

    use super::*;

    #[test]
    fn test_col_reduction_1_col() {
        let mut widths = vec![500];
        reduce_col_widths(&mut widths, 100);
        assert_eq!(widths, &[100]);
    }
    #[test]
    fn test_col_reduction_bug_01() {
        // test for a bug giving a width of 0 to the first col
        let mut widths = vec![3, 1033, 4, 10, 20, 5];
        reduce_col_widths(&mut widths, 148);
        for &width in &widths {
            assert!(width > 2);
        }
    }
    #[test]
    fn test_col_reduction_bug_unpublished_01() {
        let widths = vec![ 3, 4, 11, 5, 15, 4, 9, 5, 4, 47 ];
        let sum: usize = widths.iter().sum();
        for goal in 30..sum {
            let mut widths = widths.clone();
            reduce_col_widths(&mut widths, goal);
            println!("widths after reduction: {:?}", &widths);
            for &width in &widths {
                assert!(width > 2);
            }
            let sum: usize = widths.iter().sum();
            dbg!(sum);
            assert!(sum<=goal);
        }
    }
}
