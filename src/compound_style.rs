use {
    crate::{errors::Result, styled_char::StyledChar},
    crossterm::{
        QueueableCommand,
        style::{
            Attribute,
            Attributes,
            Color,
            ContentStyle,
            PrintStyledContent,
            SetBackgroundColor,
            SetForegroundColor,
            StyledContent,
        },
        terminal::{Clear, ClearType},
    },
    std::fmt::{self, Display},
};

/// A style which may be applied to a compound
#[derive(Default, Clone, Debug)]
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
    pub fn apply_to<D>(&self, val: D) -> StyledContent<D>
    where
        D: Clone + Display,
    {
        self.object_style.clone().apply(val)
    }

    /// Get an new instance of `CompoundStyle`
    pub fn new(
        foreground_color: Option<Color>,
        background_color: Option<Color>,
        attributes: Attributes,
    ) -> CompoundStyle {
        CompoundStyle {
            object_style: ContentStyle {
                foreground_color,
                background_color,
                attributes,
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
        self.object_style.foreground_color = Some(color);
    }

    /// Set the background color to the passed color.
    pub fn set_bg(&mut self, color: Color) {
        self.object_style.background_color = Some(color);
    }

    /// Set the colors to the passed ones
    pub fn set_fgbg(&mut self, fg: Color, bg: Color) {
        self.object_style.foreground_color = Some(fg);
        self.object_style.background_color = Some(bg);
    }

    /// Add an `Attribute`. Like italic, underlined or bold.
    pub fn add_attr(&mut self, attr: Attribute) {
        self.object_style.attributes.set(attr);
    }

    /// Add the defined characteristics of `other` to self, overwriting
    ///  its own one when defined
    pub fn overwrite_with(&mut self, other: &CompoundStyle) {
        self.object_style.foreground_color = other
            .object_style
            .foreground_color
            .or(self.object_style.foreground_color);
        self.object_style.background_color = other
            .object_style
            .background_color
            .or(self.object_style.background_color);
        self.object_style
            .attributes
            .extend(other.object_style.attributes);
    }

    #[inline(always)]
    pub fn get_fg(&self) -> Option<Color> {
        self.object_style.foreground_color
    }

    #[inline(always)]
    pub fn get_bg(&self) -> Option<Color> {
        self.object_style.background_color
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
        w.queue(PrintStyledContent(self.apply_to(val)))?;
        Ok(())
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
        if let Some(fg) = self.object_style.foreground_color {
            w.queue(SetForegroundColor(fg))?;
        }
        Ok(())
    }

    pub fn queue_bg<W>(&self, w: &mut W) -> Result<()>
    where
        W: std::io::Write,
    {
        if let Some(bg) = self.object_style.background_color {
            w.queue(SetBackgroundColor(bg))?;
        }
        Ok(())
    }

    /// Clear with the compound_style's background.
    ///
    /// ```
    /// # use termimad::*;
    /// # use crossterm::terminal::ClearType;
    /// # let skin = MadSkin::default();
    /// let mut w = std::io::stderr();
    /// skin.paragraph.compound_style.clear(&mut w, ClearType::UntilNewLine).unwrap();
    /// ```
    pub fn clear<W>(&self, w: &mut W, clear_type: ClearType) -> Result<()>
    where
        W: std::io::Write,
    {
        self.queue_bg(w)?;
        w.queue(Clear(clear_type))?;
        Ok(())
    }

    pub fn style_char(&self, nude_char: char) -> StyledChar {
        StyledChar::new(self.clone(), nude_char)
    }
}
