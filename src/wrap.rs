use minimad::{Composite, CompositeStyle, Line, Text};

fn follow_up_composite<'s>(composite: &Composite<'s>) -> Composite<'s> {
    Composite {
        style: match composite.style {
            CompositeStyle::ListItem => CompositeStyle::Paragraph,
            _ => composite.style,
        },
        compounds: Vec::new(),
    }
}

pub fn visible_composite_length(composite: &Composite) -> usize {
    (match composite.style {
        CompositeStyle::ListItem => 2, // space of the bullet
        _ => 0,
    }) + composite.char_length()
}

pub fn visible_line_length(line: &Line) -> usize {
    match line {
        Line::Normal( composite ) => visible_composite_length(composite),
        _ => 0, // FIXME implement
    }
}

/// cut the passed composite in several composites fitting the given *visible* width
/// (which might be bigger or smaller than the length of the underlying string).
/// width can't be less than 3.
pub fn hard_wrap_composite<'s>(src_composite: &Composite<'s>, width: usize) -> Vec<Composite<'s>> {
    assert!(width > 2);
    let mut composites = Vec::new();
    let max_cut_back = width / 5;
    let mut dst_composite = Composite {
        style: src_composite.style,
        compounds: Vec::new(),
    };
    let mut ll = match src_composite.style {
        CompositeStyle::ListItem => 2, // space of the bullet
        _ => 0,
    };
    for sc in &src_composite.compounds {
        let s = sc.as_str();
        let cl = s.chars().count();
        if ll + cl <= width {
            // we add the compound as is to the current composite
            dst_composite.compounds.push(sc.clone());
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
                    dst_composite.compounds.push(sc.sub(c_start, cut));
                }
                let new_dst_composite = follow_up_composite(&dst_composite);
                composites.push(dst_composite);
                dst_composite = new_dst_composite;
                c_start = cut;
                last_space = None;
                ll = idx - cut;
            }
        }
        if c_start<s.len() {
            let sc = sc.tail(c_start);
            ll = sc.as_str().chars().count(); // FAUX ? coupe trop tôt!
            dst_composite.compounds.push(sc);
        }
    }
    composites.push(dst_composite);
    composites
}

/// hard_wrap all lines to ensure the text fits the width
/// width can't be less than 3.
pub fn hard_wrap_text<'s>(text: &Text<'s>, width: usize) -> Text<'s> {
    assert!(width > 2);
    let mut lines = Vec::new();
    for src_line in &text.lines {
        if let Line::Normal( composite ) = src_line {
            for composite in hard_wrap_composite(composite, width) {
                lines.push(Line::Normal( composite ));
            }
        } else {
            lines.push((*src_line).clone());
        }
    }
    Text {
        lines
    }
}

/// Tests of hard wrapping
///
/// The print which happens in case of failure isn't really well
/// formatted. A solution if a test fails is to do
///      cargo test -- --nocapture
#[cfg(test)]
mod wrap_tests {

    use crate::displayable_line::DisplayableLine;
    use crate::skin::MadSkin;
    use crate::wrap::*;

    /// check that after hard wrap, no line is longer
    ///  that required
    /// check also that no line is empty (the source text
    ///  is assumed to have no empty line)
    fn check_no_overflow(src: &str, width: usize) {
        let src_text =  Text::from(&src);
        let text = hard_wrap_text(&src_text, width);
        println!("------- test wrapping with width: {}", width);
        let skin = MadSkin::new();
        for line in &text.lines {
            let len = visible_line_length(&line);
            println!("len:{: >4}  | {}", len, DisplayableLine {
                skin: &skin, line
            });
            assert!(len <= width);
            assert!(len > 0);
        }
    }

    /// check line lenghts are what is expected
    fn check_line_lengths(src: &str, width: usize, lenghts: Vec<usize>) {
        let src_text =  Text::from(&src);
        let text = hard_wrap_text(&src_text, width);
        assert_eq!(text.lines.len(), lenghts.len(), "same number of lines");
        for i in 0..lenghts.len() {
            assert_eq!(
                visible_line_length(&text.lines[i]),
                lenghts[i],
                "expected length for line {} when wrapping at {}", i, width
            );
        }
    }

    /// check many wrappings of a 4 lines text with 2 list items and
    /// some style
    #[test]
    fn check_hard_wrapping_simple_text() {
        // build a text and check it
        let src = "This is a *long* line which needs to be **broken**.\n\
            And the text goes on with a list:\n\
            * short item\n\
            * a *somewhat longer item* (with a part in **bold**)";
        println!("input text:\n{}", &src);
        for width in 3..50 {
            check_no_overflow(&src, width);
        }
        check_line_lengths(&src, 24, vec![19, 19, 7, 20, 13, 12, 24, 22]);
    }

}

