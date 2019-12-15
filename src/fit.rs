use minimad::{
    Compound,
};
use crate::{
    Alignment,
    FmtComposite,
    MadSkin,
};

pub static ELLIPSIS: &str = "…";


/// A fitter can shorten a composite to make it fit a target width
pub struct Fitter {

    // sum of kept head and kept tail, should be at least 1
    // If not fair, we keep one more at head.
    pub min_kept_after_ellision: usize,

    pub allow_ellision: bool,

    pub prefer_mid_token_ellision: bool,

    // if true we can aggressively remove content and whole
    // compounds
    pub allow_removals: bool,
}

impl Default for Fitter {
    fn default() -> Self {
        Self {
            min_kept_after_ellision: 4,
            allow_ellision: true,
            prefer_mid_token_ellision: true,
            allow_removals: true,
        }
    }
}

impl Fitter {

    pub fn fit(
        &self,
        fc: &mut FmtComposite<'_>,
        max_width: usize,
        align: Alignment,
        skin: &MadSkin
    ) {
        // all the widths and indices in this algorithms are of characters,
        // that is neither bytes nor graphemes
        if fc.visible_length <= max_width {
            if let Some(ref mut spacing) = fc.spacing {
                if spacing.width > max_width {
                    spacing.width = max_width;
                }
            }
            return;
        }
        let mut to_remove = fc.visible_length - max_width;
        if self.allow_ellision {
            // First strategy: ellision
            while to_remove > 0 {
                let mut compound_width = self.min_kept_after_ellision + 2;
                let mut longest: Option<usize> = None;
                for (idx, compound) in fc.composite.compounds.iter().enumerate() {
                    let len = compound.char_length();
                    if len >= compound_width {
                        longest = Some(idx);
                        compound_width = len;
                    }
                }
                if let Some(idx) = longest {
                    // to remove part of the compound, we make it 3 compounds:
                    // the head, the ellipsis, and the tail
                    let gain = to_remove.min(compound_width - self.min_kept_after_ellision - 1);
                    let left = compound_width - gain - 1;
                    let mut left_tail = left / 2;
                    let mut left_head = left - left_tail;
                    if self.prefer_mid_token_ellision && gain < compound_width + 1 {
                        // we search for a part of the compound big enough and without space
                        let mut token_start: Option<usize> = None; // start of non whitespace area
                        let mut biggest_token_width = gain + 1;
                        let mut biggest_token_start: Option<usize> = None;
                        for (char_idx, char) in fc.composite.compounds[idx].as_str().chars().enumerate() {
                            if char.is_whitespace() {
                                if let Some(ts) = token_start {
                                    let token_width = char_idx - ts;
                                    if token_width > biggest_token_width {
                                        biggest_token_start = Some(ts);
                                        biggest_token_width = token_width;
                                    }
                                    token_start = None;
                                }
                            } else {
                                if token_start.is_none() {
                                    token_start = Some(char_idx);
                                }
                            }
                        }
                        if let Some(token_start) = token_start {
                            let token_width = compound_width - 1 - token_start;
                            if token_width > biggest_token_width {
                                biggest_token_start = Some(token_start);
                                biggest_token_width = token_width;
                            }
                        }
                        if let Some(biggest_token_start) = biggest_token_start {
                            // there's a big enough token, we'll ellide in its middle
                            let r = biggest_token_width - gain - 1;
                            left_head = biggest_token_start + r / 2 + 1;
                            left_tail = left - left_head;
                        }
                    }
                    let head = fc.composite.compounds[idx].sub_chars(0, left_head);
                    let tail = fc.composite.compounds[idx].tail_chars(compound_width - left_tail);
                    fc.composite.compounds[idx]= head;
                    fc.composite.compounds.insert(idx+1, Compound::raw_str(ELLIPSIS));
                    fc.composite.compounds.insert(idx+2, tail);
                    to_remove -= gain;
                } else {
                    break; // there's no compound long enough to be ellided
                }
            }
        }
        if self.allow_removals && to_remove > 0 {
            fc.composite.remove_chars(to_remove, align);
        }

        // the visible width must be recomputed
        fc.recompute_width(&skin);
        if let Some(ref mut spacing) = fc.spacing {
            spacing.width = fc.visible_length;
        }
    }

}

/// Tests of fitting, that is cutting the composite at best to make it
///  fit a given width (if possible)
///
/// The print which happens in case of failure isn't really well
/// formatted. A solution if a test fails is to do
///      cargo test fit_tests -- --nocapture
#[cfg(test)]
mod fit_tests {

    use minimad::{
        Alignment,
        Composite,
    };
    use crate::{
        Fitter,
        FmtComposite,
    };

    fn check_fit(src: &str, target_width: usize) {
        let skin = crate::get_default_skin();
        let mut fc = FmtComposite::from(Composite::from_inline(src), &skin);
        let fitter = Fitter::default();
        fitter.fit(&mut fc, target_width, Alignment::Right, &skin);
        assert_eq!(fc.visible_length, target_width);
    }

    #[test]
    fn test_fit() {

        let sentence = "This beautiful sentence has **short** and **much much much longer** parts.";
        check_fit(sentence, 60);
        check_fit(sentence, 40);

        // the following test showcases a problem of termimad: we currently don't deal with the
        // fact that some chars take more space on screen even in monospaced font
        let five_issues = "一曰道，二曰天，三曰地，四曰將，五曰法。";
        check_fit(five_issues, 15);
        check_fit(five_issues, 8);

        let status = "ab *cd* `12345 123456789`";
        check_fit(status, 17);
        check_fit(status, 2);
    }

}
