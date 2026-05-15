#![forbid(unsafe_code)]
//! Primitive vertical rhythm helpers.
//!
//! These helpers expose a small baseline grid model for deterministic layout
//! calculations.
//!
//! # Examples
//!
//! ```rust
//! use use_type_rhythm::{TypeRhythm, baseline_grid, snap_to_baseline};
//!
//! let rhythm = TypeRhythm::new(16.0, 24.0).unwrap();
//!
//! assert_eq!(rhythm.baseline_unit(), 24.0);
//! assert_eq!(rhythm.snap_to_baseline(37.0).unwrap(), 48.0);
//! assert_eq!(rhythm.lines_for_height(72.0).unwrap(), 3.0);
//! assert_eq!(baseline_grid(24.0, 3).unwrap(), vec![24.0, 48.0, 72.0]);
//! assert_eq!(snap_to_baseline(37.0, 24.0).unwrap(), 48.0);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypeRhythm {
    base_font_size_px: f64,
    base_line_height_px: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeRhythmError {
    InvalidFontSize,
    InvalidLineHeight,
    InvalidValue,
}

fn validate_positive(value: f64, error: TypeRhythmError) -> Result<f64, TypeRhythmError> {
    if !value.is_finite() || value <= 0.0 {
        Err(error)
    } else {
        Ok(value)
    }
}

fn validate_non_negative(value: f64, error: TypeRhythmError) -> Result<f64, TypeRhythmError> {
    if !value.is_finite() || value < 0.0 {
        Err(error)
    } else {
        Ok(value)
    }
}

impl TypeRhythm {
    pub fn new(base_font_size_px: f64, base_line_height_px: f64) -> Result<Self, TypeRhythmError> {
        Ok(Self {
            base_font_size_px: validate_positive(
                base_font_size_px,
                TypeRhythmError::InvalidFontSize,
            )?,
            base_line_height_px: validate_positive(
                base_line_height_px,
                TypeRhythmError::InvalidLineHeight,
            )?,
        })
    }

    #[must_use]
    pub fn baseline_unit(&self) -> f64 {
        let _ = self.base_font_size_px;
        self.base_line_height_px
    }

    pub fn snap_to_baseline(&self, value_px: f64) -> Result<f64, TypeRhythmError> {
        snap_to_baseline(value_px, self.base_line_height_px)
    }

    pub fn lines_for_height(&self, height_px: f64) -> Result<f64, TypeRhythmError> {
        Ok(
            validate_non_negative(height_px, TypeRhythmError::InvalidValue)?
                / self.base_line_height_px,
        )
    }
}

pub fn baseline_grid(base_line_height_px: f64, lines: usize) -> Result<Vec<f64>, TypeRhythmError> {
    let base_line_height_px =
        validate_positive(base_line_height_px, TypeRhythmError::InvalidLineHeight)?;

    Ok((1..=lines)
        .map(|line| (line as f64) * base_line_height_px)
        .collect())
}

pub fn snap_to_baseline(value_px: f64, base_line_height_px: f64) -> Result<f64, TypeRhythmError> {
    let value_px = validate_non_negative(value_px, TypeRhythmError::InvalidValue)?;
    let base_line_height_px =
        validate_positive(base_line_height_px, TypeRhythmError::InvalidLineHeight)?;

    Ok((value_px / base_line_height_px).round() * base_line_height_px)
}

#[cfg(test)]
mod tests {
    use super::{TypeRhythm, TypeRhythmError, baseline_grid, snap_to_baseline};

    #[test]
    fn generates_baseline_grid_values() {
        let rhythm = TypeRhythm::new(16.0, 24.0).unwrap();

        assert_eq!(rhythm.baseline_unit(), 24.0);
        assert_eq!(rhythm.snap_to_baseline(37.0).unwrap(), 48.0);
        assert_eq!(rhythm.lines_for_height(72.0).unwrap(), 3.0);
        assert_eq!(baseline_grid(24.0, 3).unwrap(), vec![24.0, 48.0, 72.0]);
        assert_eq!(snap_to_baseline(37.0, 24.0).unwrap(), 48.0);
    }

    #[test]
    fn rejects_invalid_rhythm_inputs() {
        assert_eq!(
            TypeRhythm::new(0.0, 24.0),
            Err(TypeRhythmError::InvalidFontSize)
        );
        assert_eq!(
            baseline_grid(0.0, 3),
            Err(TypeRhythmError::InvalidLineHeight)
        );
        assert_eq!(
            snap_to_baseline(-1.0, 24.0),
            Err(TypeRhythmError::InvalidValue)
        );
        assert_eq!(
            TypeRhythm::new(16.0, 24.0)
                .unwrap()
                .lines_for_height(f64::NAN),
            Err(TypeRhythmError::InvalidValue)
        );
    }
}
