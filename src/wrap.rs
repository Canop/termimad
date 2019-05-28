use crate::composite::FmtComposite;
use crate::line::FmtLine;
use minimad::{Composite, CompositeStyle, Line, Text};

fn follow_up_composite<'s>(fc: &FmtComposite<'s>) -> FmtComposite<'s> {
    FmtComposite {
        composite: Composite {
            style: match fc.composite.style {
                CompositeStyle::ListItem => CompositeStyle::Paragraph,
                _ => fc.composite.style,
            },
            compounds: Vec::new(),
        },
        visible_length: 0,
        spacing: fc.spacing,
    }
}

/// cut the passed composite in several composites fitting the given *visible* width
/// (which might be bigger or smaller than the length of the underlying string).
/// width can't be less than 3.
pub fn hard_wrap_composite<'s>(src_composite: &FmtComposite<'s>, width: usize) -> Vec<FmtComposite<'s>> {
    assert!(width > 2);
    let mut composites = Vec::new();
    let max_cut_back = width / 5;
    let mut dst_composite = FmtComposite {
        composite: Composite {
            style: src_composite.composite.style,
            compounds: Vec::new(),
        },
        visible_length: match src_composite.composite.style { // FIXME hack : should be computed using the skin
            CompositeStyle::ListItem => 2,
            _ => 0,
        },
        spacing: src_composite.spacing,
    };
    let mut ll = match src_composite.composite.style {
        CompositeStyle::ListItem => 2, // space of the bullet
        _ => 0,
    };
    for sc in &src_composite.composite.compounds {
        let s = sc.as_str();
        let cl = s.chars().count();
        if ll + cl <= width {
            // we add the compound as is to the current composite
            dst_composite.composite.compounds.push(sc.clone());
            ll += cl;
            continue;
        }
        if ll + 1 >= width {
            // we close the current composite
            let new_dst_composite = follow_up_composite(&dst_composite);
            composites.push(dst_composite);
            dst_composite = new_dst_composite;
            ll = 0;
        }
        let mut c_start = 0;
        let mut last_space: Option<usize> = Some(0);
        for (idx, char) in s.char_indices() {
            ll += 1;
            if char.is_whitespace() {
                last_space = Some(idx);
            }
            if ll == width {
                let mut cut = idx;
                if idx+1<s.len() {
                    if let Some(ls) = last_space {
                        if ls + max_cut_back >= idx {
                            cut = ls;
                        }
                    }
                }
                if cut > c_start {
                    dst_composite.add_compound(sc.sub(c_start, cut+1));
                }
                let new_dst_composite = follow_up_composite(&dst_composite);
                composites.push(dst_composite);
                dst_composite = new_dst_composite;
                c_start = cut+1;
                last_space = None;
                ll = idx - cut;
            }
        }
        if c_start<s.len() {
            let sc = sc.tail(c_start);
            ll = sc.as_str().chars().count();
            dst_composite.add_compound(sc);
        }
    }
    if dst_composite.visible_length > 0 {
        composites.push(dst_composite);
    }
    composites
}

/// hard_wrap all normal lines to ensure the text fits the width.
/// width can't be less than 3.
/// Doesn't touch table rows.
/// Consumes the passed array and return a new one (may contain
/// the original lines, avoiding cloning when possible)
pub fn hard_wrap_lines<'s>(src_lines: Vec<FmtLine<'s>>, width: usize) -> Vec<FmtLine<'s>> {
    assert!(width > 2);
    let mut src_lines = src_lines;
    let mut lines = Vec::new();
    for src_line in src_lines.drain(..) {
        if let FmtLine::Normal( fc ) = src_line {
            if fc.visible_length <= width {
                lines.push(FmtLine::Normal( fc ));
            } else {
                for fc in hard_wrap_composite(&fc, width) {
                    lines.push(FmtLine::Normal( fc ));
                }
            }
        } else {
            lines.push(src_line);
        }
    }
    lines
}

/// Tests of hard wrapping
///
/// The print which happens in case of failure isn't really well
/// formatted. A solution if a test fails is to do
///      cargo test -- --nocapture
#[cfg(test)]
mod wrap_tests {

    use crate::skin::MadSkin;
    use crate::displayable_line::DisplayableLine;
    use crate::wrap::*;

    fn visible_fmt_line_length(skin: &MadSkin, line: &FmtLine) -> usize {
        match line {
            FmtLine::Normal( fc ) => skin.visible_composite_length(&fc.composite),
            _ => 0, // FIXME implement
        }
    }

    /// check that after hard wrap, no line is longer
    ///  that required
    /// check also that no line is empty (the source text
    ///  is assumed to have no empty line)
    fn check_no_overflow(skin: &MadSkin, src: &str, width: usize) {
        let text = skin.text(src, Some(width));
        println!("------- test wrapping with width: {}", width);
        for line in &text.lines {
            let len = visible_fmt_line_length(skin, &line);
            print!("len:{: >4}  | {}", len, DisplayableLine {
                skin: &skin, line
            });
            assert!(len <= width);
            assert!(len > 0);
        }
    }

    /// check line lenghts are what is expected
    fn check_line_lengths(skin: &MadSkin, src: &str, width: usize, lenghts: Vec<usize>) {
        let text = skin.text(src, Some(width));
        assert_eq!(text.lines.len(), lenghts.len(), "same number of lines");
        for i in 0..lenghts.len() {
            assert_eq!(
                visible_fmt_line_length(skin, &text.lines[i]),
                lenghts[i],
                "expected length for line {} when wrapping at {}", i, width
            );
        }
    }

    /// check many wrappings of a 4 lines text with 2 list items and
    /// some style
    #[test]
    fn check_hard_wrapping_simple_text() {
        let skin = crate::get_default_skin();
        // build a text and check it
        let src = "This is a *long* line which needs to be **broken**.\n\
            And the text goes on with a list:\n\
            * short item\n\
            * a *somewhat longer item* (with a part in **bold**)";
        println!("input text:\n{}", &src);
        for width in 3..50 {
            check_no_overflow(skin, &src, width);
        }
        check_line_lengths(skin, &src, 24, vec![20, 24, 1, 21, 12, 12, 24, 22]);
    }

}

