use crate::composite::FmtComposite;
use crate::line::FmtLine;
use minimad::{Composite, CompositeStyle};

/// build a composite which can be a new line after wrapping.
fn follow_up_composite<'s>(fc: &FmtComposite<'s>) -> FmtComposite<'s> {
    let style = match fc.composite.style {
        CompositeStyle::ListItem => CompositeStyle::Paragraph,
        _ => fc.composite.style,
    };
    let visible_length = match style {
        CompositeStyle::Quote => 2,
        _ => 0,
    };
    FmtComposite {
        composite: Composite {
            style,
            compounds: Vec::new(),
        },
        visible_length,
        spacing: fc.spacing,
    }
}

/// a place where a line break is OK
#[derive(Debug, Clone, Copy)]
struct BreakPoint {
    idx: usize,
    len: usize, // nb chars removed on break: 0 for start of compound, 1 for a space
}
impl BreakPoint {
    fn on_start(idx: usize) -> Self {
        Self { idx, len: 0 }
    }
    fn on_space(idx: usize) -> Self {
        Self { idx, len: 1 }
    }
}

/// cut the passed composite in several composites fitting the given *visible* width
/// (which might be bigger or smaller than the length of the underlying string).
/// width can't be less than 3.
pub fn hard_wrap_composite<'s>(
    src_composite: &FmtComposite<'s>,
    width: usize,
) -> Vec<FmtComposite<'s>> {
    assert!(width > 2);
    debug_assert!(src_composite.visible_length > width); // or we shouldn't be called
    let mut composites = Vec::new();
    let compounds = &src_composite.composite.compounds;
    // we try to optimize for a quite frequent case: two parts with nothing or just space in
    // between
    if compounds.len() == 2
        && compounds[0].char_length() < width
        && compounds[1].char_length() < width
    {
        // clean cut of 2
        composites.push(FmtComposite::from_compound(compounds[0].clone()));
        composites.push(FmtComposite::from_compound(compounds[1].clone()));
        return composites;
    }
    if compounds.len() == 3
        && compounds[0].char_length() < width
        && compounds[2].char_length() < width
        && compounds[1].src.chars().all(char::is_whitespace)
    {
        // clean cut of 3
        composites.push(FmtComposite::from_compound(compounds[0].clone()));
        composites.push(FmtComposite::from_compound(compounds[2].clone()));
        return composites;
    }
    let max_cut_back = width / 5;
    let mut dst_composite = FmtComposite {
        composite: Composite {
            style: src_composite.composite.style,
            compounds: Vec::new(),
        },
        visible_length: match src_composite.composite.style {
            CompositeStyle::ListItem => 2,
            CompositeStyle::Quote => 2,
            _ => 0,
        },
        spacing: src_composite.spacing,
    };
    let mut ll = match src_composite.composite.style {
        CompositeStyle::ListItem => 2, // space of the bullet
        CompositeStyle::Quote => 2,    // space of the quote mark
        _ => 0,
    };
    let mut ignored_cut_back: Option<(usize, BreakPoint)> = None;
    for sc in &src_composite.composite.compounds {
        ignored_cut_back = None;
        let s = sc.as_str();
        let cl = s.chars().count();
        if ll + cl <= width {
            // we add the compound as is to the current composite
            dst_composite.add_compound(sc.clone());
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
        let mut last_break_point = Some(BreakPoint::on_start(0));
        for (idx, char) in s.char_indices() {
            let char_len = char.len_utf8();
            ll += 1;
            if char.is_whitespace() {
                last_break_point = Some(BreakPoint::on_space(idx));
            }
            if ll >= width {
                // we must cut
                let mut cut = idx;
                let mut loss = 0;
                ignored_cut_back = None;
                if idx + char_len < s.len() {
                    if let Some(bp) = last_break_point {
                        if bp.idx + max_cut_back >= idx {
                            // the last break isn't too far, we'll cut there
                            cut = bp.idx;
                            if idx > bp.idx {
                                loss = bp.len;
                            }
                        } else {
                            ignored_cut_back = Some((idx, bp));
                        }
                    }
                }
                let s2 = sc.sub(c_start, cut);
                dst_composite.add_compound(s2);
                let new_dst_composite = follow_up_composite(&dst_composite);
                composites.push(dst_composite);
                dst_composite = new_dst_composite;
                last_break_point = None;
                c_start = cut + loss; // + char_len;
                ll = idx - cut - loss;
                if dst_composite.composite.is_quote() {
                    ll += 2;
                }
            }
        }
        if c_start < s.len() {
            let sc = sc.tail(c_start);
            ll = sc.as_str().chars().count();
            dst_composite.add_compound(sc);
        } else {
            ignored_cut_back = None;
        }
    }
    if dst_composite.visible_length > 0 {
        // now we try to see if we can move back the cut to the last break point
        if let Some((idx, bp)) = ignored_cut_back {
            let diff = idx - bp.idx;
            if diff + dst_composite.visible_length - bp.len < width {
                let last = composites
                    .last_mut()
                    .unwrap()
                    .composite
                    .compounds
                    .last_mut().unwrap();
                if last.as_str().len() > diff + bp.len {
                    let mut tail = last.cut_tail(diff + bp.len);
                    composites.last_mut().unwrap().visible_length -= diff + bp.len;
                    if bp.len > 0 {
                        tail = tail.tail_chars(bp.len);
                    }
                    dst_composite.composite.compounds.insert(0, tail);
                    dst_composite.visible_length += diff;
                }
            }
        }
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
        if let FmtLine::Normal(fc) = src_line {
            if fc.visible_length <= width {
                lines.push(FmtLine::Normal(fc));
            } else {
                for fc in hard_wrap_composite(&fc, width) {
                    lines.push(FmtLine::Normal(fc));
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

    use {
        crate::{
            displayable_line::DisplayableLine,
            skin::MadSkin,
            wrap::*,
        },
        minimad::*,
    };

    fn visible_fmt_line_length(skin: &MadSkin, line: &FmtLine<'_>) -> usize {
        match line {
            FmtLine::Normal(fc) => skin.visible_composite_length(&fc.composite),
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
            print!(
                "len:{: >4}  | {}",
                len,
                DisplayableLine {
                    skin: &skin,
                    line,
                    width: None,
                }
            );
            assert!(len <= width);
            assert!(len > 0);
        }
    }

    /// check line lenghts are what is expected
    fn check_line_lengths(skin: &MadSkin, src: &str, width: usize, lenghts: Vec<usize>) {
        println!("input text:\n{}", &src);
        let text = skin.text(src, Some(width));
        assert_eq!(text.lines.len(), lenghts.len(), "same number of lines");
        println!("wrapped text:\n{}", &text);
        for i in 0..lenghts.len() {
            assert_eq!(
                visible_fmt_line_length(skin, &text.lines[i]),
                lenghts[i],
                "expected length for line {} when wrapping at {}",
                i,
                width
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
        for width in 3..50 {
            check_no_overflow(skin, &src, width);
        }
        //check_line_lengths(skin, &src, 25, vec![19, 19, 7, 20, 13, 12, 24, 22]);
        check_line_lengths(skin, &src, 25, vec![19, 25, 20, 12, 12, 24, 22]);
    }

    #[test]
    fn check_space_removing() {
        let skin = crate::get_default_skin();
        let src = FmtComposite::from(Composite::from_inline("syntax coloring"), &skin);
        println!("input:\n{:?}", &src);
        let wrapped = hard_wrap_composite(&src, 8);
        println!("wrapped: {:?}", &wrapped);
        assert_eq!(wrapped.len(), 2);
    }

    fn first_compound(line: FmtLine) -> Option<Compound> {
        match line {
            FmtLine::Normal(mut fc) => fc.composite.compounds.drain(..).next(),
            _ => None,
        }
    }

    #[test]
    /// check the case of a wrapping occuring after a space and at the start of a compound
    /// see https://github.com/Canop/termimad/issues/17
    fn check_issue_17() {
        let skin = crate::get_default_skin();
        let src = "*Now I'll describe this example with more words than necessary, in order to be sure to demonstrate scrolling (and **wrapping**, too, thanks to long sentences).*";
        let text = skin.text(src, Some(120));
        assert_eq!(text.lines.len(), 2);
        assert_eq!(
            first_compound(text.lines.into_iter().nth(1).unwrap()),
            Some(Compound::raw_str("wrapping").bold().italic()),
        );
    }
}
