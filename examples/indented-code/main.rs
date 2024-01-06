use termimad::*;

static MD: &str = r#"
# Indented Code

To indent code (as demonstrated here) do this:

```rust
fn main() {
    let mut skin = MadSkin::default();
    skin.code_block.left_margin = 4;
    skin.print_text(MD);
}
```

Note that you can add some margin to other kinds of lines, not just code blocks.

"#;

fn main() {
    let mut skin = MadSkin::default();
    skin.code_block.left_margin = 4;
    skin.print_text(MD);
}
