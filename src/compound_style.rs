use std::fmt::{self, Display};
use crossterm::{Attribute, Color, ObjectStyle, StyledObject};


/// A style which may be applied to a compound
#[derive(Default, Clone)]
pub struct CompoundStyle {
    pub object_style: ObjectStyle, // a crossterm object style
}

impl From<ObjectStyle> for CompoundStyle {
    fn from(object_style: ObjectStyle) -> CompoundStyle {
        CompoundStyle {
            object_style
        }
    }
}

impl CompoundStyle {

    /// Apply an `StyledObject` to the passed displayable object.
    pub fn apply_to<D: Display>(&self, val: D) -> StyledObject<D> {
        self.object_style.apply_to(val)
    }

    /// Get an new instance of `CompoundStyle`
    pub fn new(fg_color: Option<Color>, bg_color: Option<Color>, attrs: Vec<Attribute>) -> CompoundStyle {
        CompoundStyle {
            object_style: ObjectStyle {
                fg_color,
                bg_color,
                attrs,
            }
        }
    }

    /// Get an new instance of `CompoundStyle`
    pub fn with_fgbg(fg: Color, bg: Color) -> CompoundStyle {
        CompoundStyle {
            object_style: ObjectStyle::new().fg(fg).bg(bg)
        }
    }

    /// Get an new instance of `CompoundStyle`
    pub fn with_fg(fg: Color) -> CompoundStyle {
        CompoundStyle {
            object_style: ObjectStyle::new().fg(fg)
        }
    }

    /// Get an new instance of `CompoundStyle`
    pub fn with_bg(bg: Color) -> CompoundStyle {
        CompoundStyle {
            object_style: ObjectStyle::new().bg(bg)
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

    /// Write a string several times with the line compound style
    ///
    /// Implementation Note: performances here are critical
    #[inline(always)]
    pub fn repeat_string(
        &self,
        f: &mut fmt::Formatter<'_>,
        s: &str,
        count: usize,
    ) -> fmt::Result {
        if count > 0 {
            write!(f, "{}", self.apply_to(s.repeat(count)))
        } else {
            Ok(())
        }
    }

    /// Write 0 or more spaces with the line's compound style
    #[inline(always)]
    pub fn repeat_space(
        &self,
        f: &mut fmt::Formatter<'_>,
        count: usize,
    ) -> fmt::Result {
        self.repeat_string(f, " ", count)
    }
}

