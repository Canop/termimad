use std::fmt::{self, Display};

use crossterm::{
    queue, Attribute, Color, ContentStyle, PrintStyledContent, SetBg, SetFg, StyledContent,
};

use crate::errors::Result;

/// A style which may be applied to a compound
#[derive(Default, Clone)]
pub struct CompoundStyle {
    pub object_style: ContentStyle, // a crossterm content style
}

impl From<ContentStyle> for CompoundStyle {
    fn from(object_style: ContentStyle) -> CompoundStyle {
        CompoundStyle { object_style }
    }
}

impl CompoundStyle {
    /// Apply an `StyledContent` to the passed displayable object.
    pub fn apply_to<D: Display>(&self, val: D) -> StyledContent<D>
    where
        D: Clone,
    {
        self.object_style.apply(val)
    }

    /// Get an new instance of `CompoundStyle`
    pub fn new(
        fg_color: Option<Color>,
        bg_color: Option<Color>,
        attrs: Vec<Attribute>,
    ) -> CompoundStyle {
        CompoundStyle {
            object_style: ContentStyle {
                fg_color,
                bg_color,
                attrs,
            },
        }
    }

    /// Get an new instance of `CompoundStyle`
    pub fn with_fgbg(fg: Color, bg: Color) -> CompoundStyle {
        CompoundStyle {
            object_style: ContentStyle::new().foreground(fg).background(bg),
        }
    }

    /// Get an new instance of `CompoundStyle`
    pub fn with_fg(fg: Color) -> CompoundStyle {
        CompoundStyle {
            object_style: ContentStyle::new().foreground(fg),
        }
    }

    /// Get an new instance of `CompoundStyle`
    pub fn with_bg(bg: Color) -> CompoundStyle {
        CompoundStyle {
            object_style: ContentStyle::new().background(bg),
        }
    }

    /// Get an new instance of `CompoundStyle`
    pub fn with_attr(attr: Attribute) -> CompoundStyle {
        let mut cp = CompoundStyle::default();
        cp.add_attr(attr);
        cp
    }

    /// Set the foreground color to the passed color.
    pub fn set_fg(&mut self, color: Color) {
        self.object_style.fg_color = Some(color);
    }

    /// Set the background color to the passed color.
    pub fn set_bg(&mut self, color: Color) {
        self.object_style.bg_color = Some(color);
    }

    /// Set the colors to the passed ones
    pub fn set_fgbg(&mut self, fg: Color, bg: Color) {
        self.object_style.fg_color = Some(fg);
        self.object_style.bg_color = Some(bg);
    }

    /// Add an `Attribute`. Like italic, underlined or bold.
    pub fn add_attr(&mut self, attr: Attribute) {
        self.object_style.attrs.push(attr);
    }

    /// Add the defined characteristics of `other` to self, overwriting
    ///  its own one when defined
    pub fn overwrite_with(&mut self, other: &CompoundStyle) {
        self.object_style.fg_color = other.object_style.fg_color.or(self.object_style.fg_color);
        self.object_style.bg_color = other.object_style.bg_color.or(self.object_style.bg_color);
        self.object_style.attrs.extend(&other.object_style.attrs);
    }

    #[inline(always)]
    pub fn get_fg(&self) -> Option<Color> {
        self.object_style.fg_color
    }

    #[inline(always)]
    pub fn get_bg(&self) -> Option<Color> {
        self.object_style.bg_color
    }

    /// Write a string several times with the line compound style
    ///
    /// Implementation Note: performances here are critical
    #[inline(always)]
    pub fn repeat_string(&self, f: &mut fmt::Formatter<'_>, s: &str, count: usize) -> fmt::Result {
        if count > 0 {
            write!(f, "{}", self.apply_to(s.repeat(count)))
        } else {
            Ok(())
        }
    }

    /// Write 0 or more spaces with the line's compound style
    #[inline(always)]
    pub fn repeat_space(&self, f: &mut fmt::Formatter<'_>, count: usize) -> fmt::Result {
        self.repeat_string(f, " ", count)
    }

    /// write the value with this style on the given
    /// writer
    pub fn queue<W, D>(&self, w: &mut W, val: D) -> Result<()>
    where
        D: Clone + Display,
        W: std::io::Write,
    {
        Ok(queue!(w, PrintStyledContent(self.apply_to(val)))?)
    }

    /// write the string with this style on the given
    /// writer
    pub fn queue_str<W>(&self, w: &mut W, s: &str) -> Result<()>
    where
        W: std::io::Write,
    {
        self.queue(w, s.to_string())
    }

    pub fn queue_fg<W>(&self, w: &mut W) -> Result<()>
    where
        W: std::io::Write,
    {
        Ok(if let Some(fg) = self.object_style.fg_color {
            queue!(w, SetFg(fg))?;
        })
    }

    pub fn queue_bg<W>(&self, w: &mut W) -> Result<()>
    where
        W: std::io::Write,
    {
        Ok(if let Some(bg) = self.object_style.bg_color {
            queue!(w, SetBg(bg))?;
        })
    }
}
