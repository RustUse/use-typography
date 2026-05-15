#![forbid(unsafe_code)]
//! Primitive line-height helpers.
//!
//! These helpers expose explicit line-height calculations and a small readable
//! threshold check.
//!
//! # Examples
//!
//! ```rust
//! use use_line_height::{LineHeight, is_readable_line_height, line_height_px, line_height_ratio};
//!
//! let line_height = LineHeight::new(16.0, 24.0).unwrap();
//!
//! assert_eq!(line_height.px(), 24.0);
//! assert!((line_height.ratio() - 1.5).abs() < 1.0e-12);
//! assert!((line_height_px(16.0, 1.5).unwrap() - 24.0).abs() < 1.0e-12);
//! assert!((line_height_ratio(16.0, 24.0).unwrap() - 1.5).abs() < 1.0e-12);
//! assert!(is_readable_line_height(1.5).unwrap());
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineHeight {
    font_size_px: f64,
    line_height_px: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineHeightError {
    InvalidFontSize,
    InvalidLineHeight,
    InvalidRatio,
}

fn validate_positive(value: f64, error: LineHeightError) -> Result<f64, LineHeightError> {
    if !value.is_finite() || value <= 0.0 {
        Err(error)
    } else {
        Ok(value)
    }
}

impl LineHeight {
    pub fn new(font_size_px: f64, line_height_px: f64) -> Result<Self, LineHeightError> {
        Ok(Self {
            font_size_px: validate_positive(font_size_px, LineHeightError::InvalidFontSize)?,
            line_height_px: validate_positive(line_height_px, LineHeightError::InvalidLineHeight)?,
        })
    }

    #[must_use]
    pub fn ratio(&self) -> f64 {
        self.line_height_px / self.font_size_px
    }

    #[must_use]
    pub fn px(&self) -> f64 {
        self.line_height_px
    }
}

pub fn line_height_px(font_size_px: f64, ratio: f64) -> Result<f64, LineHeightError> {
    Ok(
        validate_positive(font_size_px, LineHeightError::InvalidFontSize)?
            * validate_positive(ratio, LineHeightError::InvalidRatio)?,
    )
}

pub fn line_height_ratio(font_size_px: f64, line_height_px: f64) -> Result<f64, LineHeightError> {
    Ok(
        validate_positive(line_height_px, LineHeightError::InvalidLineHeight)?
            / validate_positive(font_size_px, LineHeightError::InvalidFontSize)?,
    )
}

pub fn is_readable_line_height(ratio: f64) -> Result<bool, LineHeightError> {
    let ratio = validate_positive(ratio, LineHeightError::InvalidRatio)?;
    Ok((1.4..=1.8).contains(&ratio))
}

#[cfg(test)]
mod tests {
    use super::{
        is_readable_line_height, line_height_px, line_height_ratio, LineHeight, LineHeightError,
    };

    #[test]
    fn computes_line_height_ratios() {
        let line_height = LineHeight::new(16.0, 24.0).unwrap();

        assert_eq!(line_height.px(), 24.0);
        assert!((line_height.ratio() - 1.5).abs() < 1.0e-12);
        assert!((line_height_px(16.0, 1.5).unwrap() - 24.0).abs() < 1.0e-12);
        assert!((line_height_ratio(16.0, 24.0).unwrap() - 1.5).abs() < 1.0e-12);
    }

    #[test]
    fn checks_readable_line_height_thresholds() {
        assert!(is_readable_line_height(1.4).unwrap());
        assert!(is_readable_line_height(1.8).unwrap());
        assert!(!is_readable_line_height(1.3).unwrap());
        assert!(!is_readable_line_height(1.9).unwrap());
    }

    #[test]
    fn rejects_invalid_line_height_inputs() {
        assert_eq!(
            LineHeight::new(0.0, 24.0),
            Err(LineHeightError::InvalidFontSize)
        );
        assert_eq!(
            line_height_px(16.0, 0.0),
            Err(LineHeightError::InvalidRatio)
        );
        assert_eq!(
            line_height_ratio(16.0, f64::NAN),
            Err(LineHeightError::InvalidLineHeight)
        );
        assert_eq!(
            is_readable_line_height(f64::NEG_INFINITY),
            Err(LineHeightError::InvalidRatio)
        );
    }
}
