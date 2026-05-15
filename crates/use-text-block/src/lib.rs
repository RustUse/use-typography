#![forbid(unsafe_code)]
//! Primitive text-block estimation helpers.
//!
//! These helpers estimate text block layout using explicit widths, average
//! character widths, and line heights.
//!
//! # Examples
//!
//! ```rust
//! use use_text_block::{TextBlock, estimated_line_count, estimated_text_height};
//!
//! let block = TextBlock::new(480.0, 16.0, 24.0, 120).unwrap();
//!
//! assert_eq!(block.estimated_characters_per_line(8.0).unwrap(), 60.0);
//! assert_eq!(block.estimated_line_count(8.0).unwrap(), 2);
//! assert_eq!(block.estimated_height_px(8.0).unwrap(), 48.0);
//! assert_eq!(estimated_line_count(120, 60.0).unwrap(), 2);
//! assert_eq!(estimated_text_height(2, 24.0).unwrap(), 48.0);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextBlock {
    width_px: f64,
    font_size_px: f64,
    line_height_px: f64,
    character_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextBlockError {
    InvalidWidth,
    InvalidFontSize,
    InvalidLineHeight,
    InvalidCharacterWidth,
    InvalidCharactersPerLine,
}

fn validate_positive(value: f64, error: TextBlockError) -> Result<f64, TextBlockError> {
    if !value.is_finite() || value <= 0.0 {
        Err(error)
    } else {
        Ok(value)
    }
}

impl TextBlock {
    pub fn new(
        width_px: f64,
        font_size_px: f64,
        line_height_px: f64,
        character_count: usize,
    ) -> Result<Self, TextBlockError> {
        Ok(Self {
            width_px: validate_positive(width_px, TextBlockError::InvalidWidth)?,
            font_size_px: validate_positive(font_size_px, TextBlockError::InvalidFontSize)?,
            line_height_px: validate_positive(line_height_px, TextBlockError::InvalidLineHeight)?,
            character_count,
        })
    }

    pub fn estimated_characters_per_line(
        &self,
        average_character_width_px: f64,
    ) -> Result<f64, TextBlockError> {
        let _ = self.font_size_px;

        Ok(self.width_px
            / validate_positive(
                average_character_width_px,
                TextBlockError::InvalidCharacterWidth,
            )?)
    }

    pub fn estimated_line_count(
        &self,
        average_character_width_px: f64,
    ) -> Result<usize, TextBlockError> {
        estimated_line_count(
            self.character_count,
            self.estimated_characters_per_line(average_character_width_px)?,
        )
    }

    pub fn estimated_height_px(
        &self,
        average_character_width_px: f64,
    ) -> Result<f64, TextBlockError> {
        estimated_text_height(
            self.estimated_line_count(average_character_width_px)?,
            self.line_height_px,
        )
    }
}

pub fn estimated_line_count(
    character_count: usize,
    characters_per_line: f64,
) -> Result<usize, TextBlockError> {
    let characters_per_line = validate_positive(
        characters_per_line,
        TextBlockError::InvalidCharactersPerLine,
    )?;

    if character_count == 0 {
        Ok(0)
    } else {
        Ok(((character_count as f64) / characters_per_line).ceil() as usize)
    }
}

pub fn estimated_text_height(
    line_count: usize,
    line_height_px: f64,
) -> Result<f64, TextBlockError> {
    Ok((line_count as f64) * validate_positive(line_height_px, TextBlockError::InvalidLineHeight)?)
}

#[cfg(test)]
mod tests {
    use super::{TextBlock, TextBlockError, estimated_line_count, estimated_text_height};

    #[test]
    fn estimates_text_block_lines_and_height() {
        let block = TextBlock::new(480.0, 16.0, 24.0, 120).unwrap();

        assert_eq!(block.estimated_characters_per_line(8.0).unwrap(), 60.0);
        assert_eq!(block.estimated_line_count(8.0).unwrap(), 2);
        assert_eq!(block.estimated_height_px(8.0).unwrap(), 48.0);
        assert_eq!(estimated_line_count(125, 60.0).unwrap(), 3);
        assert_eq!(estimated_text_height(3, 24.0).unwrap(), 72.0);
    }

    #[test]
    fn supports_empty_text_blocks() {
        let block = TextBlock::new(480.0, 16.0, 24.0, 0).unwrap();

        assert_eq!(block.estimated_line_count(8.0).unwrap(), 0);
        assert_eq!(block.estimated_height_px(8.0).unwrap(), 0.0);
    }

    #[test]
    fn rejects_invalid_text_block_inputs() {
        assert_eq!(
            TextBlock::new(0.0, 16.0, 24.0, 10),
            Err(TextBlockError::InvalidWidth)
        );
        assert_eq!(
            estimated_line_count(10, 0.0),
            Err(TextBlockError::InvalidCharactersPerLine)
        );
        assert_eq!(
            estimated_text_height(2, f64::NAN),
            Err(TextBlockError::InvalidLineHeight)
        );
    }
}
