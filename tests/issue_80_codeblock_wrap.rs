//! Regression test for https://github.com/Canop/termimad/issues/80
//!
//! When a code block contains a line wider than the width passed to
//! `skin.text`, every rendered row must still fit that width. Before the fix,
//! `justify_blocks` ran before `hard_wrap_lines`, so each code line's spacing
//! was squared to the widest *raw* line (e.g. 300) and the rendered rows
//! overflowed the requested width instead of being wrapped into it.

use termimad::MadSkin;

/// Visible width of a rendered line, ignoring ANSI escape sequences.
///
/// The test content is ASCII (`x` and spaces), so the number of visible
/// characters equals the terminal display width.
fn visible_width(line: &str) -> usize {
    let mut width = 0usize;
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Skip the escape sequence up to its terminating letter.
            for n in chars.by_ref() {
                if n.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            width += 1;
        }
    }
    width
}

/// A code block whose widest raw line is far wider than the requested width
/// must render with no row exceeding that width.
#[test]
fn code_block_wider_than_width_is_wrapped_not_padded() {
    let width = 80;
    let long = "x".repeat(300);
    let md = format!("before\n\n```text\nshort line\n{long}\n```\n\nafter\n");
    let skin = MadSkin::default();
    let text = skin.text(&md, Some(width));
    let rendered = format!("{text}");

    for (i, line) in rendered.lines().enumerate() {
        let vw = visible_width(line);
        assert!(
            vw <= width,
            "line {i} has visible width {vw} > requested {width}: {line:?}"
        );
    }
}

/// A code block that already fits the width must still be justified into a
/// rectangle: every code row is padded to the same (widest) width. This guards
/// against a fix that would break uniform justification for fitting blocks.
#[test]
fn small_code_block_stays_square() {
    let width = 80;
    let md = "```text\nab\nabcd\n```\n";
    let skin = MadSkin::default();
    let text = skin.text(md, Some(width));
    let rendered = format!("{text}");

    let row_widths: Vec<usize> = rendered
        .lines()
        .map(visible_width)
        .filter(|w| *w > 0)
        .collect();
    assert_eq!(
        row_widths.len(),
        2,
        "expected two code rows, got widths {row_widths:?}"
    );
    assert_eq!(
        row_widths[0], row_widths[1],
        "code block rows must be padded to an equal (square) width"
    );
    assert!(
        row_widths[0] <= width,
        "square width {} must not exceed requested {width}",
        row_widths[0]
    );
}
