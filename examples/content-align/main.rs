use termimad::*;

static MD: &str = r#"
----

# Centered Title

A medium long text.
It's bigger than the other parts but thinner than your terminal.
*I mean I hope it's thinner than your terminal*

    A right aligned thing

----

Note how all parts are aligned with the content width:
* title's centering is consistent with the text
* horizontal separators aren't wider than the text
* right aligned thing isn't stuck to the terminal's right side

This content align trick is useful for wide terminals
(especially when you know the content is thin)

----

"#;


fn main() {
    let mut skin = MadSkin::default();
    skin.code_block.align = Alignment::Right;

    let (width, _) = termimad::terminal_size();
    let terminal_width = width as usize;

    let mut text = FmtText::from(&skin, MD, Some(terminal_width));
    text.set_rendering_width(text.content_width());
    println!("{}", text);
}
