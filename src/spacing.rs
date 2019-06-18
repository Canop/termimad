use minimad::Alignment;

#[derive(Debug, Clone, Copy)]
pub struct Spacing {
    pub width: usize,
    pub align: Alignment,
}

impl Spacing {
    /// compute the number of chars to add left and write of inner_width
    /// to fill outer_width
    #[inline(always)]
    pub fn completions(align: Alignment, inner_width: usize, outer_width: usize) -> (usize, usize) {
        if inner_width >= outer_width {
            return (0, 0);
        }
        match align {
            Alignment::Left | Alignment::Unspecified => (0, outer_width - inner_width),
            Alignment::Center => {
                let lp = (outer_width - inner_width) / 2;
                (lp, outer_width - inner_width - lp)
            },
            Alignment::Right => (outer_width - inner_width, 0),
        }
    }
    #[inline(always)]
    pub fn optional_completions(align: Alignment, inner_width: usize, outer_width: Option<usize>) -> (usize, usize) {
        match outer_width {
            Some(outer_width) => Spacing::completions(align, inner_width, outer_width),
            None => (0, 0),
        }
    }
    #[inline(always)]
    pub fn completions_for(&self, inner_width: usize) -> (usize, usize) {
        Spacing::completions(self.align, inner_width, self.width)
    }
}


