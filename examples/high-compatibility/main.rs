use {
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
# ${title}

## When to use it ?

* ${points}

## Terminal capabilities:

|:-:|:-:|
|**Capability**|**Necessary**|
|-:|:-:|
|ansi escape codes|${ansi-codes}|
|non ascii characters|${non-ascii}|
|-:|:-:|

## Skin initialization:

```
${code}
```

"#;


fn fun(
    title: &str,
    skin: MadSkin,
    when: &[&str],
    ansi_codes: bool,
    non_ascii: bool,
    code: &str,
) {
    let mut expander = OwningTemplateExpander::new();
    expander
        .set("title", title)
        .set_lines("points", when.join("\n"))
        .set_lines("code", code)
        .set("ansi-codes", if ansi_codes { "yes" } else { "no" })
        .set("non-ascii", if non_ascii { "yes" } else { "no" });
    let template = TextTemplate::from(TEMPLATE);
    let text = expander.expand(&template);
    let (width, _) = terminal_size();
    let fmt_text = FmtText::from_text(&skin, text, Some(width as usize));
    println!("{}", fmt_text);
}


fn main() {
    // default skin
    let skin = MadSkin::default();
    fun(
        "Default skin",
        skin,
        &["almost always"],
        true,
        true,
        "let skin = MadSkin::default();",
    );

    // skin without ANSI escape codes
    let skin = MadSkin::no_style();
    fun(
        "Without ANSI escape codes",
        skin,
        &["when your terminal is very old"],
        false,
        true,
        "let skin = MadSkin::no_style();"
    );

    // skin with only ascii chars
    let mut skin = MadSkin::default();
    skin.limit_to_ascii();
    fun(
        "Using only ASCII",
        skin,
        &["when your terminal only knows ASCII"],
        true,
        false,
        r#"
        let mut skin = MadSkin::default();
        skin.limit_to_ascii();
        "#
    );

    // skin with only ascii chars and no ANSI escape code
    let mut skin = MadSkin::no_style();
    skin.limit_to_ascii();
    fun(
        "Using only ASCII and no ANSI escape code",
        skin,
        &[
            "when your terminal is very very very old",
            "when your multiplexer is pigeon carrier based",
        ],
        false,
        false,
        r#"
        let mut skin = MadSkin::no_style();
        skin.limit_to_ascii();
        "#
    );

}
