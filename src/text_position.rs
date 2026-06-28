pub use mirl_extensions::*;

use mirl_geometry_core::text_position::*;

#[must_use]
/// Get the line and offset from the given offset
pub fn try_line_and_column_from_offset<T: TryIntoPatch<usize> + Copy>(
    offset: T,
    text: &str,
) -> Option<(usize, usize)> {
    let mut line = 1;
    let mut col = 1;

    for (i, ch) in text.char_indices() {
        if i >= offset.try_into_value()? {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    Some((line, col))
}

#[must_use]
/// Get the offset from the given line and column
pub fn try_offset_from_line_and_column<T: TryIntoPatch<usize> + Copy>(
    line: T,
    col: T,
    text: &str,
) -> Option<usize> {
    let target_line = line.try_into_value()?;
    let target_col = col.try_into_value()?;

    let mut current_line = 1;
    let mut current_col = 1;

    for (i, ch) in text.char_indices() {
        if current_line == target_line && current_col == target_col {
            return Some(i);
        }

        if ch == '\n' {
            current_line += 1;
            current_col = 1;
        } else {
            current_col += 1;
        }
    }

    if current_line == target_line && current_col == target_col {
        return Some(text.len());
    }

    None
}
/// Ge the text position from just a string and an offset
pub const trait TextPositionTryFromOffset: Sized {
    #[must_use]
    /// Get the line and offset from the given offset
    fn try_from_offset<T: TryIntoPatch<usize> + Copy>(offset: T, text: &str) -> Option<Self>;
}

impl TextPositionTryFromOffset for TextPosition {
    /// Get the line and offset from the given offset
    fn try_from_offset<T: TryIntoPatch<usize> + Copy>(offset: T, text: &str) -> Option<Self> {
        let (line, col) = try_line_and_column_from_offset(offset, text)?;

        Some(Self::new(line, col))
    }
}
