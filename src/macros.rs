

/// print a markdown template, with other arguments taking `$0` to `$9` places in the template.
///
/// Example:
///
/// ```
/// use termimad::*;
///
/// let skin = MadSkin::default();
/// mad_print_inline!(
/// 	&skin,
/// 	"**$0 formula:** *$1*", // the markdown template, interpreted once
/// 	"Disk",  // fills $0
/// 	"2*π*r", // fills $1. Note that the stars don't mess the markdown
/// );
/// ```
#[macro_export]
macro_rules! mad_print_inline {
    ($skin: expr, $md: literal $(, $value: expr )* $(,)? ) => {
        $skin.print_composite(termimad::minimad::mad_inline!($md $(, $value)*));
    };
}

/// write a markdown template, with other arguments taking `$0` to `$9` places in the template.
///
/// Example:
///
/// ```
/// use termimad::*;
///
/// let skin = MadSkin::default();
/// mad_write_inline!(
/// 	&mut std::io::stdout(),
/// 	&skin,
/// 	"**$0 formula:** *$1*", // the markdown template, interpreted once
/// 	"Disk",  // fills $0
/// 	"2*π*r", // fills $1. Note that the stars don't mess the markdown
/// ).unwrap();
/// ```
#[macro_export]
macro_rules! mad_write_inline {
    ($w: expr, $skin: expr, $md: literal $(, $value: expr )* $(,)? ) => {{
        use std::io::Write;
        $skin.write_composite($w, termimad::minimad::mad_inline!($md $(, $value)*))
    }};
}



