use crossterm::{self, Color};

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

/// build a gray-level color, from 0:dark to 23:light.
pub fn gray(level: u8) -> Color {
    assert!(level<24, "invalid gray level (must be in 0..24)");
    Color::AnsiValue(0xE8 + level)
}

#[cfg(test)]
mod color_tests {

    use crate::color::*;

    /// check the color range is correctly checked and a meaningful
    /// error is raised
    #[test]
    #[should_panic(expected = "invalid gray level (must be in 0..24)")]
    fn check_gray_panic() {
        let _ = gray(24);
    }

}
