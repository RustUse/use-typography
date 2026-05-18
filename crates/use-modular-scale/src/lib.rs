#![forbid(unsafe_code)]
//! Primitive modular typography scale helpers.
//!
//! These helpers keep common scale ratios explicit and deterministic.
//!
//! # Examples
//!
//! ```rust
//! use use_modular_scale::{ScaleRatio, modular_scale, scale_down, scale_up};
//!
//! let values = modular_scale(16.0, ScaleRatio::MajorThird, -1, 1).unwrap();
//!
//! assert!((scale_up(16.0, ScaleRatio::MajorThird, 1).unwrap() - 20.0).abs() < 1.0e-12);
//! assert!((scale_down(16.0, ScaleRatio::MajorThird, 1).unwrap() - 12.8).abs() < 1.0e-12);
//! assert_eq!(values.len(), 3);
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScaleRatio {
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    AugmentedFourth,
    PerfectFifth,
    GoldenRatio,
    Custom(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModularScaleError {
    InvalidBaseSize,
    InvalidRatio,
    InvalidStepRange,
}

fn validate_positive_base(base_px: f64) -> Result<f64, ModularScaleError> {
    if !base_px.is_finite() || base_px <= 0.0 {
        Err(ModularScaleError::InvalidBaseSize)
    } else {
        Ok(base_px)
    }
}

fn validate_ratio(value: f64) -> Result<f64, ModularScaleError> {
    if !value.is_finite() || value <= 1.0 {
        Err(ModularScaleError::InvalidRatio)
    } else {
        Ok(value)
    }
}

fn scale_for_step(base_px: f64, ratio: f64, step: isize) -> f64 {
    base_px * ratio.powf(step as f64)
}

impl ScaleRatio {
    pub fn value(&self) -> Result<f64, ModularScaleError> {
        match self {
            Self::MinorSecond => Ok(1.067),
            Self::MajorSecond => Ok(1.125),
            Self::MinorThird => Ok(1.2),
            Self::MajorThird => Ok(1.25),
            Self::PerfectFourth => Ok(1.333),
            Self::AugmentedFourth => Ok(1.414),
            Self::PerfectFifth => Ok(1.5),
            Self::GoldenRatio => Ok(1.618_033_988_75),
            Self::Custom(value) => validate_ratio(*value),
        }
    }
}

pub fn scale_up(base_px: f64, ratio: ScaleRatio, steps: usize) -> Result<f64, ModularScaleError> {
    let base_px = validate_positive_base(base_px)?;
    let ratio = validate_ratio(ratio.value()?)?;
    Ok(scale_for_step(base_px, ratio, steps as isize))
}

pub fn scale_down(base_px: f64, ratio: ScaleRatio, steps: usize) -> Result<f64, ModularScaleError> {
    let base_px = validate_positive_base(base_px)?;
    let ratio = validate_ratio(ratio.value()?)?;
    Ok(scale_for_step(base_px, ratio, -(steps as isize)))
}

pub fn modular_scale(
    base_px: f64,
    ratio: ScaleRatio,
    min_step: isize,
    max_step: isize,
) -> Result<Vec<f64>, ModularScaleError> {
    if max_step < min_step {
        return Err(ModularScaleError::InvalidStepRange);
    }

    let base_px = validate_positive_base(base_px)?;
    let ratio = validate_ratio(ratio.value()?)?;

    Ok((min_step..=max_step)
        .map(|step| scale_for_step(base_px, ratio, step))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::{ModularScaleError, ScaleRatio, modular_scale, scale_down, scale_up};

    #[test]
    fn generates_modular_scale_values() {
        let values = modular_scale(16.0, ScaleRatio::MajorThird, -1, 1).unwrap();

        assert_eq!(values.len(), 3);
        assert!((values[0] - 12.8).abs() < 1.0e-12);
        assert!((values[1] - 16.0).abs() < 1.0e-12);
        assert!((values[2] - 20.0).abs() < 1.0e-12);
        assert!((scale_up(16.0, ScaleRatio::MajorThird, 2).unwrap() - 25.0).abs() < 1.0e-12);
        assert!((scale_down(16.0, ScaleRatio::MajorThird, 1).unwrap() - 12.8).abs() < 1.0e-12);
    }

    #[test]
    fn rejects_invalid_scale_inputs() {
        assert_eq!(
            scale_up(0.0, ScaleRatio::MajorThird, 1),
            Err(ModularScaleError::InvalidBaseSize)
        );
        assert_eq!(
            scale_down(16.0, ScaleRatio::Custom(1.0), 1),
            Err(ModularScaleError::InvalidRatio)
        );
        assert_eq!(
            modular_scale(16.0, ScaleRatio::MajorThird, 2, 1),
            Err(ModularScaleError::InvalidStepRange)
        );
    }
}
