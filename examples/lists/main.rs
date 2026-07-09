use termimad::{
    crossterm::style::Color::*,
    *,
};

static MD: &str = r#"

# Support for lists

1. Unordered lists are supported
 - this is an unordered list item
 - and another one
 - they're created with a `*`, a `-`, or a `+` at the beginning of the line, followed by a space
2. Ordered lists are supported
 1. you can choose the color of the index (the number of the item)
 1. and the suffix (the dot or parenthesis, or other character)
 1. and the color of the suffix, of course
 1. indexes are recomputed to be consistent
3. And sublists too
 * I just demonstrated it above, so you knew it at this point
 * But maybe you didn't notice that the style of ordered list items is depth dependant ?
 * And that ordered and unordered lists can be mixed, and nested, in any way you want
  1. and you can go deeper than 2 levels, of course
  2. but remember that markdown doesn't allow the "2.1.2" kind of numbering, so it's often a bad idea to go deeper than 2 levels
 * By the way, and you should see that if your terminal is thin enough, default indentation logic is block based, which helps to keep the text readable, but you can also choose to have only the first line indented, which is more compact

"#;

fn main() {
    let mut skin = MadSkin::default();
    skin.set_headers_fg(rgb(255, 187, 0));
    skin.bold.set_fg(Yellow);
    skin.italic.set_fgbg(Magenta, rgb(30, 30, 40));
    skin.bullet = StyledChar::from_fg_char(Yellow, '⟡');
    skin.quote_mark.set_fg(Yellow);
    skin.ordered_item_styles[0] = OrderedItemStyle {
        index_style: CompoundStyle::with_fg(Yellow),
        index_suffix: StyledChar::from_fg_char(Yellow, ')'),
    };
    skin.print_text(MD);
}
