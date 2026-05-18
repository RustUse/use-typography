#![forbid(unsafe_code)]
//! Primitive font-size conversions.
//!
//! These helpers keep size conversions explicit and validate the input sizes
//! used in the calculation.
//!
//! # Examples
//!
//! ```rust
//! use use_font_size::{FontSize, em_to_px, px_to_rem, rem_to_px};
//!
//! let font = FontSize::new(18.0).unwrap();
//!
//! assert!((font.rem(16.0).unwrap() - 1.125).abs() < 1.0e-12);
//! assert!((px_to_rem(18.0, 16.0).unwrap() - 1.125).abs() < 1.0e-12);
//! assert!((rem_to_px(1.125, 16.0).unwrap() - 18.0).abs() < 1.0e-12);
//! assert!((em_to_px(1.5, 12.0).unwrap() - 18.0).abs() < 1.0e-12);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontSize {
    px: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontSizeError {
    InvalidFontSize,
    InvalidRootSize,
    InvalidParentSize,
    InvalidRem,
    InvalidEm,
}

fn validate_positive(value: f64, error: FontSizeError) -> Result<f64, FontSizeError> {
    if !value.is_finite() || value <= 0.0 {
        Err(error)
    } else {
        Ok(value)
    }
}

impl FontSize {
    pub fn new(px: f64) -> Result<Self, FontSizeError> {
        Ok(Self {
            px: validate_positive(px, FontSizeError::InvalidFontSize)?,
        })
    }

    #[must_use]
    pub fn px(&self) -> f64 {
        self.px
    }

    pub fn rem(&self, root_px: f64) -> Result<f64, FontSizeError> {
        px_to_rem(self.px, root_px)
    }

    pub fn em(&self, parent_px: f64) -> Result<f64, FontSizeError> {
        px_to_em(self.px, parent_px)
    }
}

pub fn px_to_rem(px: f64, root_px: f64) -> Result<f64, FontSizeError> {
    Ok(validate_positive(px, FontSizeError::InvalidFontSize)?
        / validate_positive(root_px, FontSizeError::InvalidRootSize)?)
}

pub fn rem_to_px(rem: f64, root_px: f64) -> Result<f64, FontSizeError> {
    Ok(validate_positive(rem, FontSizeError::InvalidRem)?
        * validate_positive(root_px, FontSizeError::InvalidRootSize)?)
}

pub fn px_to_em(px: f64, parent_px: f64) -> Result<f64, FontSizeError> {
    Ok(validate_positive(px, FontSizeError::InvalidFontSize)?
        / validate_positive(parent_px, FontSizeError::InvalidParentSize)?)
}

pub fn em_to_px(em: f64, parent_px: f64) -> Result<f64, FontSizeError> {
    Ok(validate_positive(em, FontSizeError::InvalidEm)?
        * validate_positive(parent_px, FontSizeError::InvalidParentSize)?)
}

#[cfg(test)]
mod tests {
    use super::{FontSize, FontSizeError, em_to_px, px_to_em, px_to_rem, rem_to_px};

    #[test]
    fn converts_between_px_rem_and_em() {
        let font = FontSize::new(18.0).unwrap();

        assert_eq!(font.px(), 18.0);
        assert!((font.rem(16.0).unwrap() - 1.125).abs() < 1.0e-12);
        assert!((font.em(12.0).unwrap() - 1.5).abs() < 1.0e-12);
        assert!((px_to_rem(18.0, 16.0).unwrap() - 1.125).abs() < 1.0e-12);
        assert!((rem_to_px(1.125, 16.0).unwrap() - 18.0).abs() < 1.0e-12);
        assert!((px_to_em(18.0, 12.0).unwrap() - 1.5).abs() < 1.0e-12);
        assert!((em_to_px(1.5, 12.0).unwrap() - 18.0).abs() < 1.0e-12);
    }

    #[test]
    fn rejects_invalid_font_size_inputs() {
        assert_eq!(FontSize::new(0.0), Err(FontSizeError::InvalidFontSize));
        assert_eq!(px_to_rem(-1.0, 16.0), Err(FontSizeError::InvalidFontSize));
        assert_eq!(px_to_em(18.0, 0.0), Err(FontSizeError::InvalidParentSize));
        assert_eq!(rem_to_px(f64::NAN, 16.0), Err(FontSizeError::InvalidRem));
        assert_eq!(
            em_to_px(1.0, f64::INFINITY),
            Err(FontSizeError::InvalidParentSize)
        );
    }
}
