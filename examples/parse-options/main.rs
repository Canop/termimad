use termimad::{
    minimad,
};

static MD: &str = r#"
    This text has too many indentations,
    hard-wrapping breaking an *italic
    part*, some code, and some **bold
    sub-sentence** too.
    To have span continue across lines,
    parse markdown with `minimad::parse_text(
    src, TreeOptions::default().continue_spans());`
    To fix superfluous indentations, use
        let options = TreeOptions::default()
            .clean_indentations();
"#;

fn print_md(s: &str) {
    print_text(minimad::parse_text(s, minimad::Options::default()));
}

fn print_text(text: minimad::Text) {
    let skin = termimad::get_default_skin();
    let fmt_text = termimad::FmtText::from_text(skin, text, None);
    println!("{fmt_text}");
}

fn main() {
    println!();
    print_md("# Raw Text:");
    println!("{MD}");

    // Cleaning indentations remove the indentation levels which are
    // most often due to the text being defined in indented raw literals
    println!();
    print_md("# Parsed, with indentations cleaned:");
    let options = minimad::Options::default()
        .clean_indentations(true);
    let text = minimad::parse_text(MD, options);
    print_text(text);

    // Span continuation allow italic, bold, strikeout, code, to
    // continue after a newline.
    // (you may be more selective, see minimad::Options)
    println!();
    print_md("# With span continuation too:");
    let options = minimad::Options::default()
        .clean_indentations(true)
        .continue_spans(true);
    let text = minimad::parse_text(MD, options);
    print_text(text);


}
