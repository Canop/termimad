
A simple tool to display static or dynamic Markdown snippets in the terminal, with custom styles.

Based on crossterm so works on most terminals (even on windows).

##  Examples

### With the default skin:

    termimad::println("**Some** *nested **style*** and `some(code)`");

Result:

![simple example](doc/default-skin-simple.png)

### With a custom skin:

    let mut skin = MadSkin::new();
    skin.bold = skin.bold.fg(Color::Yellow);
    skin.println("*Hey* **World!** Here's `some(code)`");
    skin.normal = skin.normal.bg(Color::Rgb{r:30, g:30, b:40}).fg(Color::Magenta);
    skin.italic.add_attr(Attribute::Underlined);
    println!("and now {}", skin.line("a little *too much* **style!** (and `some(code)` too)"));

Result:

![too much style](doc/too_much.png)

