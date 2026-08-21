#[allow(unused_imports)]
use {
    crate::{
        minimad::*,
        *,
    },
    unicode_width::UnicodeWidthStr,
};

/// build a composite which can be a new line after wrapping.
fn follow_up_composite<'s>(fc: &FmtComposite<'s>, skin: &MadSkin) -> FmtComposite<'s> {
    let kind = match fc.kind {
        CompositeKind::ListItem(l) => CompositeKind::ListItemFollowUp(l),
        CompositeKind::OrderedListItem { level, index } => CompositeKind::OrderedListItemFollowUp { level, index },
        k => k,
    };
    let visible_length = match kind {
        CompositeKind::ListItemFollowUp(l)
            if skin.list_items_indentation_mode == ListItemsIndentationMode::Block =>
        {
            2 + l as usize
        }
        CompositeKind::OrderedListItemFollowUp { level, index }
            if skin.list_items_indentation_mode == ListItemsIndentationMode::Block =>
        {
            ordered_item_indent(level, index)
        }
        CompositeKind::Quote => 2,
        _ => 0,
    };
    FmtComposite {
        kind,
        compounds: Vec::new(),
        visible_length,
        spacing: fc.spacing,
    }
}

/// return the inherent widths related to the kind, the one of the first line (for
/// example with a bullet) and the ones for the next lines (for example with quotes)
#[must_use]
pub fn composite_kind_widths(
    composite_kind: CompositeKind,
    skin: &MadSkin,
) -> (usize, usize) {
    match composite_kind {
        CompositeKind::Paragraph => (0, 0),
        CompositeKind::Header(_) => (0, 0),
        CompositeKind::ListItem(depth) => {
            let indent = 2 + depth as usize;
            match skin.list_items_indentation_mode {
                ListItemsIndentationMode::FirstLineOnly => (indent, 0),
                ListItemsIndentationMode::Block => (indent, indent),
            }
        }
        CompositeKind::ListItemFollowUp(depth) => {
            let indent = 2 + depth as usize;
            match skin.list_items_indentation_mode {
                ListItemsIndentationMode::FirstLineOnly => (0, 0),
                ListItemsIndentationMode::Block => (indent, indent),
            }
        }
        CompositeKind::OrderedListItem { level, index } => {
            let indent = ordered_item_indent(level, index);
            match skin.list_items_indentation_mode {
                ListItemsIndentationMode::FirstLineOnly => (indent, 0),
                ListItemsIndentationMode::Block => (indent, indent),
            }
        }
        CompositeKind::OrderedListItemFollowUp { level, index } => {
            let indent = ordered_item_indent(level, index);
            match skin.list_items_indentation_mode {
                ListItemsIndentationMode::FirstLineOnly => (indent, 0),
                ListItemsIndentationMode::Block => (indent, indent),
            }
        }
        CompositeKind::Code => (0, 0),
        CompositeKind::Quote => (2, 2),
    }
}

/// cut the passed composite in several composites fitting the given *visible* width
/// (which might be bigger or smaller than the length of the underlying string).
/// width can't be less than 3.
pub fn hard_wrap_composite<'s, 'c>(
    src_composite: &'c FmtComposite<'s>,
    width: usize,
    skin: &MadSkin,
) -> Result<Vec<FmtComposite<'s>>, InsufficientWidthError> {
    if width < 3 {
        return Err(InsufficientWidthError {
            available_width: width,
        });
    }
    debug_assert!(src_composite.visible_length > width); // or we shouldn't be called
    let mut composites: Vec<FmtComposite<'s>> = Vec::new();
    let (first_width, other_widths) = composite_kind_widths(src_composite.kind, skin);
    let mut dst_composite = FmtComposite {
        kind: src_composite.kind,
        compounds: Vec::new(),
        visible_length: first_width,
        spacing: src_composite.spacing,
    };

    // Strategy 1:
    // we try to optimize for a quite frequent case: two parts with nothing or just space in
    // between
    let compounds = &src_composite.compounds;
    if (
        // clean cut of 2
        compounds.len() == 2
            && compounds[0].src.width() + first_width <= width
            && compounds[1].src.width() + other_widths <= width
    ) || (
        // clean cut of 3
        compounds.len() == 3
            && compounds[0].src.width() + first_width <= width
            && compounds[2].src.width() + other_widths <= width
            && compounds[1].src.chars().all(char::is_whitespace)
    ) {
        dst_composite.add_compound(compounds[0].clone());
        let mut new_dst_composite = follow_up_composite(&dst_composite, skin);
        composites.push(dst_composite);
        new_dst_composite.add_compound(compounds[compounds.len() - 1].clone());
        composites.push(new_dst_composite);
        return Ok(composites);
    }

    let mut tokens = tokenize(&src_composite.compounds, width - first_width);
    // Strategy 2:
    // we try to cut along tokens, using spaces to break
    for token in tokens.drain(..) {
        // TODO: does that really take first_width into account ?
        if dst_composite.visible_length + token.width > width {
            if !token.blank {
                // we skip blank composite at line change
                let mut repl_composite = follow_up_composite(&dst_composite, skin);
                std::mem::swap(&mut dst_composite, &mut repl_composite);
                composites.push(repl_composite);
                dst_composite.add_compound(token.to_compound());
            }
        } else {
            dst_composite.add_compound(token.to_compound());
        }
    }
    composites.push(dst_composite);
    Ok(composites)
}

/// hard_wrap all normal lines to ensure the text fits the width.
/// Doesn't touch table rows.
/// Consumes the passed array and return a new one (may contain
/// the original lines, avoiding cloning when possible).
/// Return an error if the width is less than 3.
pub fn hard_wrap_lines<'s>(
    src_lines: Vec<FmtLine<'s>>,
    width: usize,
    skin: &MadSkin,
) -> Result<Vec<FmtLine<'s>>, InsufficientWidthError> {
    let mut src_lines = src_lines;
    let mut lines = Vec::new();
    for src_line in src_lines.drain(..) {
        if let FmtLine::Normal(fc) = src_line {
            let (left_margin, right_margin) = skin.line_style(fc.kind).margins_in(Some(width));
            if fc.visible_length + left_margin + right_margin <= width {
                lines.push(FmtLine::Normal(fc));
            } else {
                for fc in hard_wrap_composite(&fc, width - left_margin - right_margin, skin)? {
                    lines.push(FmtLine::Normal(fc));
                }
            }
        } else {
            lines.push(src_line);
        }
    }
    Ok(lines)
}
