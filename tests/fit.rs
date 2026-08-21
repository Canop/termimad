//! Tests of fitting, that is cutting a composite at best to make it
//! fit a given width (if possible), and of the underlying str fitting.
//!
//! The print which happens in case of failure isn't really well
//! formatted. A solution if a test fails is to do
//!      cargo test --test fit -- --nocapture

use termimad::{
    minimad::{
        Alignment,
        Composite,
    },
    Fitter,
    FmtComposite,
    StrFit,
};

fn check_fit_align(src: &str, target_width: usize, align: Alignment) {
    dbg!((target_width, align));
    let skin = termimad::get_default_skin();
    let mut fc = FmtComposite::from(Composite::from_inline(src), skin);
    let fitter = Fitter::for_align(align);
    fitter.fit(&mut fc, target_width, skin);
    dbg!(&fc);
    assert!(fc.visible_length <= target_width); // can be smaller
}

fn check_fit(src: &str, target_width: usize) {
    check_fit_align(src, target_width, Alignment::Right);
    check_fit_align(src, target_width, Alignment::Left);
    check_fit_align(src, target_width, Alignment::Center);
    check_fit_align(src, target_width, Alignment::Unspecified);
}

#[test]
fn test_fit() {
    let sentence = "This sentence has **short** and **much longer** parts, and some Korean: *一曰道，二曰天*.";
    check_fit(sentence, 60);
    check_fit(sentence, 40);

    let five_issues = "一曰道，二曰天，三曰地，四曰將，五曰法。";
    check_fit(five_issues, 15);
    check_fit(five_issues, 8);

    let status = "ab *cd* `12345 123456789`";
    check_fit(status, 17);
    check_fit(status, 2);
}

#[test]
fn test_count_fitting() {
    assert_eq!(StrFit::count_fitting("test", 3), (3, 3));
    assert_eq!(StrFit::count_fitting("test", 5), (4, 4));
    let c12 = "Comunicações"; // normalized string (12 characters, 14 bytes)
    assert_eq!(c12.len(), 14);
    assert_eq!(c12.chars().count(), 12);
    assert_eq!(StrFit::count_fitting(c12, 12), (14, 12));
    assert_eq!(StrFit::count_fitting(c12, 10), (12, 10));
    assert_eq!(StrFit::count_fitting(c12, 11), (13, 11));
    let c14 = "Comunicações"; // unnormalized string (14 characters, 16 bytes)
    assert_eq!(c14.len(), 16);
    assert_eq!(c14.chars().count(), 14);
    assert_eq!(StrFit::count_fitting(c14, 12), (16, 12));
    let ja = "概要"; // each char takes 3 bytes and 2 columns
    assert_eq!(ja.len(), 6);
    assert_eq!(ja.chars().count(), 2);
    assert_eq!(StrFit::count_fitting(ja, 1), (0, 0));
    assert_eq!(StrFit::count_fitting(ja, 2), (3, 2));
    assert_eq!(StrFit::count_fitting(ja, 3), (3, 2));
    assert_eq!(StrFit::count_fitting(ja, 4), (6, 4));
    assert_eq!(StrFit::count_fitting(ja, 5), (6, 4));
}
