//! Tests of hard wrapping
//!
//! The print which happens in case of failure isn't really well
//! formatted. A solution if a test fails is to do
//!      cargo test --test wrap -- --nocapture

use termimad::{
    minimad::{
        Composite,
        Compound,
    },
    wrap::hard_wrap_composite,
    DisplayableLine,
    FmtComposite,
    FmtLine,
    FmtText,
    MadSkin,
};

fn visible_fmt_line_length(skin: &MadSkin, line: &FmtLine<'_>) -> usize {
    match line {
        FmtLine::Normal(fc) => skin.visible_composite_length(fc.kind, &fc.compounds),
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
        let len = visible_fmt_line_length(skin, line);
        println!(
            "len:{: >4}  | {}",
            len,
            DisplayableLine {
                skin,
                line,
                width: None,
            }
        );
        assert!(len <= width);
        assert!(len > 0);
    }
}

/// check line lenghts are what is expected
#[allow(clippy::needless_range_loop)]
fn check_line_lengths(skin: &MadSkin, src: &str, width: usize, lenghts: Vec<usize>) {
    println!("====\ninput text:\n{}", src);
    let text = skin.text(src, Some(width));
    assert_eq!(text.lines.len(), lenghts.len(), "same number of lines");
    println!("====\nwrapped text:\n{}", text);
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
    let skin = termimad::get_default_skin();
    // build a text and check it
    let src = "This is a *long* line which needs to be **broken**.\n\
               And the text goes on with a list:\n\
               * short item\n\
               * a *somewhat longer item* (with a part in **bold**)";
    for width in 3..50 {
        check_no_overflow(skin, src, width);
    }
    check_line_lengths(skin, src, 25, vec![25, 19, 25, 7, 12, 25, 23]);
}

#[test]
fn check_space_removing() {
    let skin = termimad::get_default_skin();
    let src = FmtComposite::from(Composite::from_inline("syntax coloring"), skin);
    println!("input:\n{:?}", src);
    let wrapped = hard_wrap_composite(&src, 8, skin).unwrap();
    println!("wrapped: {:?}", wrapped);
    assert_eq!(wrapped.len(), 2);
}

fn first_compound(line: FmtLine) -> Option<Compound> {
    match line {
        FmtLine::Normal(mut fc) => fc.compounds.drain(..).next(),
        _ => None,
    }
}

#[test]
/// check the case of a wrapping occuring after a space and at the start of a compound
/// see https://github.com/Canop/termimad/issues/17
fn check_issue_17() {
    let skin = termimad::get_default_skin();
    let src = "*Now I'll describe this example with more words than necessary, in order to be sure to demonstrate scrolling (and **wrapping**, too, thanks to long sentences).*";
    let text = skin.text(src, Some(120));
    assert_eq!(text.lines.len(), 2);
    assert_eq!(
        first_compound(text.lines.into_iter().nth(1).unwrap()),
        Some(Compound::raw_str("wrapping").bold().italic()),
    );
}

#[test]
/// check that we're not wrapping outside of char boudaries
fn check_issue_23() {
    let md: &str = "ZA\u{360}\u{321}\u{34a}\u{35d}LGΌ IS\u{36e}\u{302}\u{489}\u{32f}\u{348}\u{355}\u{339}\u{318}\u{331} T</b>O\u{345}\u{347}\u{339}\u{33a}Ɲ\u{334}ȳ\u{333} TH\u{318}<b>E\u{344}\u{309}\u{356} \u{360}P\u{32f}\u{34d}\u{32d}O\u{31a}\u{200b}N\u{310}Y\u{321} H\u{368}\u{34a}\u{33d}\u{305}\u{33e}\u{30e}\u{321}\u{338}\u{32a}\u{32f}E\u{33e}\u{35b}\u{36a}\u{344}\u{300}\u{301}\u{327}\u{358}\u{32c}\u{329} \u{367}\u{33e}\u{36c}\u{327}\u{336}\u{328}\u{331}\u{339}\u{32d}\u{32f}C\u{36d}\u{30f}\u{365}\u{36e}\u{35f}\u{337}\u{319}\u{332}\u{31d}\u{356}O\u{36e}\u{34f}\u{32e}\u{32a}\u{31d}\u{34d}";
    let skin = MadSkin::default();
    for w in 40..60 {
        println!("wrapping on width {}", w);
        let _text = FmtText::from(&skin, md, Some(w));
    }
}
