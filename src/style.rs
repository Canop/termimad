/// elementary bricks of a skin

use std::fmt::Display;
use crossterm::{self, Attribute, Color, ObjectStyle, StyledObject};
use minimad::Alignment;

#[macro_export]
macro_rules! rgb {
    (
        $r:expr, $g:expr, $b:expr
    ) => {
        crossterm::Color::Rgb {
            r: $r,
            g: $g,
            b: $b,
        }
    }
}

/// A style which may be applied to a compound
/// Right now it's just a wrapper around a crossterm ObjectStyle
#[derive(Default, Clone)]
pub struct CompoundStyle {
    pub object_style: ObjectStyle, // a crossterm object style
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

    /// add the defined characteristics of `other` to self, overwriting
    ///  its own one when defined
    pub fn overwrite_with(&mut self, other: &CompoundStyle) {
        self.object_style.fg_color = other.object_style.fg_color.or(self.object_style.fg_color);
        self.object_style.bg_color = other.object_style.bg_color.or(self.object_style.bg_color);
        self.object_style.attrs.extend(&other.object_style.attrs);
    }

}

#[derive(Default)]
pub struct LineStyle {
    pub compound_style: CompoundStyle,
    pub align: Alignment,
    // add a bool to tell whether the background covers the whole line ?
    //      or is it the case as soon as align isn't unspecified ?
    // add a padding: usize ?
}

impl LineStyle {

    /// Set the foreground color to the passed color.
    #[inline(always)]
    pub fn set_fg(&mut self, color: Color) {
        self.compound_style.set_fg(color);
    }

    /// Set the background color to the passed color.
    #[inline(always)]
    pub fn set_bg(&mut self, color: Color) {
        self.compound_style.set_bg(color);
    }

    /// Set the colors to the passed ones
    pub fn set_fgbg(&mut self, fg: Color, bg: Color) {
        self.compound_style.set_fgbg(fg, bg);
    }

    /// Add an `Attribute`. Like italic, underlined or bold.
    #[inline(always)]
    pub fn add_attr(&mut self, attr: Attribute) {
        self.compound_style.add_attr(attr);
    }
}

/// The scrollbar style is defined by two styled chars, one
///  for the track, and one for the thumb.
/// For the default styling only the fg color is defined
///  and the char is ▐ but everything can be changed
pub struct ScrollBarStyle {
    pub track: StyledObject<char>,
    pub thumb: StyledObject<char>,
}

impl ScrollBarStyle {
    pub fn new() -> ScrollBarStyle {
        let char = '▐';
        ScrollBarStyle {
            track: ObjectStyle::new().fg(Color::Rgb{r:35, g:35, b:35}).apply_to(char),
            thumb: ObjectStyle::new().fg(Color::Rgb{r:140, g:140, b:140}).apply_to(char),
        }
    }
    pub fn set_thumb_fg(&mut self, c: Color) {
        let os = ObjectStyle::new().fg(c);
        self.thumb = os.apply_to(self.thumb.content);
    }
    pub fn set_track_fg(&mut self, c: Color) {
        let os = ObjectStyle::new().fg(c);
        self.track = os.apply_to(self.track.content);
    }
}
