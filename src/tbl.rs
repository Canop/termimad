use crate::text::FormattedText;
use crate::wrap;
use minimad::{Composite, Line, TableRow};

/// tables are the sequences of lines whose line style is TableRow
/// A table is just the indices, without the text
/// This structure isn't public because the indices are invalid as
///  soon as rows are inserted.
struct Table {
    start: usize,
    height: usize, // number of lines
    nbcols: usize,  // number of columns
}

// an internal struct used during col resizing
#[derive(Debug)]
struct Col {
    idx: usize, // index of the col
    width: usize, // col internal width
    to_remove: usize, // what should be removed
}

// no width can go below 3
// This function should be called only when the goal is attainable
// and when there's reducion to be done
fn reduce_col_widths(widths: &mut Vec<usize>, goal: usize) {
    let sum = widths.iter().fold(0, |s, w| s + w);
    let mut excess = sum - goal;
    assert!(excess > 0);
    let mut cols: Vec<Col> = widths.iter().enumerate().map(|(idx, width)| {
        let to_remove = (width * excess / sum).min(width-3);
        excess -= to_remove;
        Col { idx, width:*width, to_remove}
    }).collect();
    cols.sort_by(|a, b| b.to_remove.cmp(&a.to_remove));
    for col in &mut cols {
        if col.to_remove < 3 {
            excess += col.to_remove;
            col.to_remove = 0;
        }
    }
    if excess > 0 {
        cols[0].to_remove += excess;
    }
    for c in cols {
        widths[c.idx] -= c.to_remove;
    }
}

impl Table {
    pub fn fix_columns(&self, text: &mut FormattedText, width: usize) {
        let mut nbcols = self.nbcols;
        assert!(width > 5);
        // let's first compute the initial widths of all columns
        // (not counting the widths of the borders)
        // We also add the missing cells
        let mut widths: Vec<usize> = vec![0; nbcols];
        for ir in self.start..self.start + self.height {
            let line = &mut text.text.lines[ir];
            if let Line::TableRow(TableRow{cells}) = line {
                for ic in 0..nbcols {
                    if cells.len() <= ic {
                        cells.push(Composite::new());
                    } else {
                        widths[ic] = widths[ic].max(cells[ic].char_length());
                    }
                }
            } else {
                println!("not a table row, should not happen"); // should we panic ?
            }
        }
        // let's find what we must do
        let mut add_padding = false;
        let widths_sum = widths.iter().fold(0, |s, w| s+w);
        // TODO propose a formatting without the external border ?
        if widths_sum + nbcols*3 + 1 <= width {
            // all is well, there's enough space for a 1 char padding
            add_padding = true;
        } else if widths_sum + nbcols + 1 <= width {
            // it fits if we don't put a padding in cells
        } else if nbcols*4 + 1 <= width {
            // we can keep all columns but we'll have to wrap them
            reduce_col_widths(&mut widths, width - nbcols - 1);
        } else {
            // crisis behavior: we remove the columns which don't fit
            nbcols = (width - 1) / 4;
            for ic in 0..nbcols {
                widths[ic] = 3;
            }
        }
        // Now we resize all cells
        //  and we insert new rows if necessary
        // We iterate in reverse order so that we can insert rows
        //  without recomputing row indices
        for ir in (self.start..self.start + self.height).rev() {
            let line = &mut text.text.lines[ir];
            if let Line::TableRow(TableRow{cells}) = line {
                let mut cells_to_add: Vec<Vec<Composite>> = Vec::new();
                cells.truncate(nbcols);
                for ic in 0..nbcols {
                    if cells.len()<=ic {
                        cells.push(Composite::new());
                        continue;
                    }
                    cells_to_add.push(Vec::new());
                    let cl = cells[ic].char_length();
                    let w = widths[ic];
                    if cl <= w {
                        // we add spaces around the content
                        let (lp, rp) = if add_padding {
                            (1, w - cl + 1)
                        } else {
                            (0, w - cl)
                        };
                        cells[ic].pad_left(lp);
                        cells[ic].pad_right(rp);
                    } else {
                        // we must wrap the cell over several lines
                        let mut composites = wrap::hard_wrap_composite(&cells[ic], w);
                        for inc in 0..composites.len() {
                            let winc = composites[inc].char_length();
                            composites[inc].pad_right(w - winc);
                        }
                        debug_assert!(composites.len()>1);
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
                    let mut new_cells: Vec<Composite> = Vec::new();
                    for ic in 0..nbcols {
                        new_cells.push(if cells_to_add[ic].len()>inl {
                            cells_to_add[ic].remove(inl)
                        } else {
                            let mut c = Composite::new();
                            c.pad_right(widths[ic]);
                            c
                        });
                    }
                    let new_line = Line::new_table_row(new_cells);
                    text.text.lines.insert(ir+1, new_line);
                }
            }
        }
    }
}

// find the positions of all tables
fn find_tables(text: &FormattedText) -> Vec<Table> {
    let mut tables: Vec<Table> = Vec::new();
    let mut current: Option<Table> = None;
    for (idx, line) in text.text.lines.iter().enumerate() {
        match line {
            Line::TableRow(TableRow{cells}) => {
                match current.as_mut() {
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
                }
            }
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

// modify the rows of all tables in order to ensure it fits the widths
// and all cells have the widths of their column.
// Some lines may be added to the table in the process, which means any
//  precedent indexing might be invalid.
pub fn fix_all_tables(text: &mut FormattedText, width: usize) {
    for tbl in find_tables(text).iter().rev() {
        tbl.fix_columns(text, width);
    }
}

