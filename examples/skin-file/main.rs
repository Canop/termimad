use {
    std::fs,
    termimad::{
        minimad::{
            OwningTemplateExpander,
            TextTemplate,
        },
        MadSkin,
        FmtText,
        terminal_size,
    },
};

static TEMPLATE: &str = r#"
----

# Skin File


## What this example demonstrates

* read a skin from the skin file in the example directory
* print this text with the skin just deserialized
* a simple template
* ~~animated style transitions~~

## How it works

The skin file is read into a string, then **deserialized** into a `MadSkin`:

```
let hjson = fs::read_to_string(file_path)?;
let skin: MadSkin = deser_hjson::from_str(&hjson)?;
```

*(of course it doesn't have to be Hjson, it can be JSON, TOML, or any serde compatible format of your choice)*

As we want to print the real skin file, we use a template (see the complete code), but using the skin could have been as simple as

```
skin.print_text(MD);
```

In your own program, you may want to not parse a `MadSkin` but some specific styling parts then build the skin(s) yourself.
In this case, use the various functions of the `parse` module: they allow parsing `Color`, `LineStyle`, `CompoundStyle`, `StyledChar`, etc. from strings using the same syntax than for a whole skin deserialization.

## Skin syntax

Here's the content of the skin file used to render this text:

    ${skin}

### Colors:

A color can be described as
* one of the standard ANSI color names: `red`, `yellow`, `magenta`, etc.
* a gray level, `gray(0)` (dark) to `gray(23)` (light)
* an ANSI color code, eg `ansi(123)`
* a rgb color, eg `rgb(255, 0, 50)`, `#fb0`, or `#cafe00`

### Inline styles:

Inline styles are "bold", "italic", "strikeout", "inline-code", and "ellipsis".

They're defined by a foreground color, a background color, and attributes, all those parts being optional.

The first encountered color is the foreground color. If you want no foreground color but a background one, use `none`, eg `bold none red`.

### Line styles

Line styles are "paragraph", "code-block", and "table".

They're defined like inline styles but accept an optional alignment (`left`, `right`, or `center`).

### Styled chars

Styled chars are "bullet", "quote", "scrollbar", and "horizontal-rule".

They're defined by a character (which must be one character wide and long), and foreground and background colors. All parts are optional.

### Headers:

Headers are line styles too.
They can he defined one per one:

    headers: [
        yellow bold center
        yellow underlined
        yellow
    ]

*(you don't have to define all 8 possible levels, others will stay default)*

It's also possible to change all default headers with a shortened syntax. The example below would set all headers to be in yellow and italic but keep all other properties as default:

    headers: yellow italic

### Summary: Skin entries

|:-:|:-:|:-:|
|**keys**|**type**|**comments**|
|:-:|:-:|:-|
|bold|inline|
|italic|inline|
|strikeout|inline|
|inline-code, inline_code|inline|
|paragraph|line|standard line
|code-block, code_block|line|
|table|line|fg and bg colors are for the border
|bullet|character|
|quote, quote-mark, quote_mark|character|
|scrollbar|character|
|horizontal-rule, horizontal_rule, rule|character|
|:-:|:-:|:-|

----
"#;


fn main() {
    // read the skin file in a string
    let hjson = fs::read_to_string("examples/skin-file/skin.hjson").unwrap();

    // deserialize the Hjson into a skin
    let skin: MadSkin = deser_hjson::from_str(&hjson).unwrap();

    // build the text with a template so that we can include
    // the real content of the skin file
    let mut expander = OwningTemplateExpander::new();
    expander.set_lines("skin", hjson);
    let template = TextTemplate::from(TEMPLATE);
    let text = expander.expand(&template);

    // render the text in the terminal
    let (width, _) = terminal_size();
    let fmt_text = FmtText::from_text(&skin, text, Some(width as usize));
    print!("{}", fmt_text);
}
