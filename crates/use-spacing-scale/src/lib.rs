#![forbid(unsafe_code)]
//! Primitive spacing scale helpers.
//!
//! These helpers generate predictable spacing values from a base size and
//! ratio.
//!
//! # Examples
//!
//! ```rust
//! use use_spacing_scale::{SpacingScale, spacing_step, spacing_steps};
//!
//! let scale = SpacingScale::new(8.0, 2.0).unwrap();
//!
//! assert_eq!(scale.step(-1), 4.0);
//! assert_eq!(scale.step(2), 32.0);
//! assert_eq!(spacing_step(8.0, 2.0, 1).unwrap(), 16.0);
//! assert_eq!(spacing_steps(8.0, 2.0, -1, 2).unwrap(), vec![4.0, 8.0, 16.0, 32.0]);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpacingScale {
    base_px: f64,
    ratio: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpacingScaleError {
    InvalidBaseSize,
    InvalidRatio,
    InvalidStepRange,
}

fn validate_base(base_px: f64) -> Result<f64, SpacingScaleError> {
    if !base_px.is_finite() || base_px <= 0.0 {
        Err(SpacingScaleError::InvalidBaseSize)
    } else {
        Ok(base_px)
    }
}

fn validate_ratio(ratio: f64) -> Result<f64, SpacingScaleError> {
    if !ratio.is_finite() || ratio <= 1.0 {
        Err(SpacingScaleError::InvalidRatio)
    } else {
        Ok(ratio)
    }
}

fn spacing_value(base_px: f64, ratio: f64, step: isize) -> f64 {
    base_px * ratio.powf(step as f64)
}

impl SpacingScale {
    pub fn new(base_px: f64, ratio: f64) -> Result<Self, SpacingScaleError> {
        Ok(Self {
            base_px: validate_base(base_px)?,
            ratio: validate_ratio(ratio)?,
        })
    }

    #[must_use]
    pub fn step(&self, step: isize) -> f64 {
        spacing_value(self.base_px, self.ratio, step)
    }

    #[must_use]
    pub fn steps(&self, min_step: isize, max_step: isize) -> Vec<f64> {
        if max_step < min_step {
            return Vec::new();
        }

        (min_step..=max_step).map(|step| self.step(step)).collect()
    }
}

pub fn spacing_step(base_px: f64, ratio: f64, step: isize) -> Result<f64, SpacingScaleError> {
    let scale = SpacingScale::new(base_px, ratio)?;
    Ok(scale.step(step))
}

pub fn spacing_steps(
    base_px: f64,
    ratio: f64,
    min_step: isize,
    max_step: isize,
) -> Result<Vec<f64>, SpacingScaleError> {
    if max_step < min_step {
        return Err(SpacingScaleError::InvalidStepRange);
    }

    let scale = SpacingScale::new(base_px, ratio)?;
    Ok(scale.steps(min_step, max_step))
}

#[cfg(test)]
mod tests {
    use super::{spacing_step, spacing_steps, SpacingScale, SpacingScaleError};

    #[test]
    fn generates_spacing_scale_values() {
        let scale = SpacingScale::new(8.0, 2.0).unwrap();

        assert_eq!(scale.step(-1), 4.0);
        assert_eq!(scale.step(0), 8.0);
        assert_eq!(scale.step(2), 32.0);
        assert_eq!(scale.steps(-1, 2), vec![4.0, 8.0, 16.0, 32.0]);
        assert_eq!(spacing_step(8.0, 2.0, 1).unwrap(), 16.0);
        assert_eq!(
            spacing_steps(8.0, 2.0, -1, 2).unwrap(),
            vec![4.0, 8.0, 16.0, 32.0]
        );
    }

    #[test]
    fn rejects_invalid_spacing_scale_inputs() {
        assert_eq!(
            SpacingScale::new(0.0, 2.0),
            Err(SpacingScaleError::InvalidBaseSize)
        );
        assert_eq!(
            SpacingScale::new(8.0, 1.0),
            Err(SpacingScaleError::InvalidRatio)
        );
        assert_eq!(
            spacing_steps(8.0, 2.0, 2, -1),
            Err(SpacingScaleError::InvalidStepRange)
        );
    }
}
