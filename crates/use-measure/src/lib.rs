#![forbid(unsafe_code)]
//! Primitive readable measure helpers.
//!
//! The helpers here focus on character-count estimates, not a full text
//! layout engine.
//!
//! # Examples
//!
//! ```rust
//! use use_measure::{Measure, characters_per_line, container_width_for_measure, is_readable_measure};
//!
//! let measure = Measure::new(66.0).unwrap();
//!
//! assert_eq!(measure.characters_per_line(), 66.0);
//! assert!(measure.is_readable());
//! assert_eq!(characters_per_line(528.0, 8.0).unwrap(), 66.0);
//! assert_eq!(container_width_for_measure(66.0, 8.0).unwrap(), 528.0);
//! assert!(is_readable_measure(66.0).unwrap());
//! ```

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measure {
    characters_per_line: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MeasureError {
    InvalidCharactersPerLine,
    InvalidContainerWidth,
    InvalidCharacterWidth,
}

fn validate_positive(value: f64, error: MeasureError) -> Result<f64, MeasureError> {
    if !value.is_finite() || value <= 0.0 {
        Err(error)
    } else {
        Ok(value)
    }
}

impl Measure {
    pub fn new(characters_per_line: f64) -> Result<Self, MeasureError> {
        Ok(Self {
            characters_per_line: validate_positive(
                characters_per_line,
                MeasureError::InvalidCharactersPerLine,
            )?,
        })
    }

    #[must_use]
    pub fn characters_per_line(&self) -> f64 {
        self.characters_per_line
    }

    #[must_use]
    pub fn is_readable(&self) -> bool {
        (45.0..=90.0).contains(&self.characters_per_line)
    }
}

pub fn characters_per_line(
    container_width_px: f64,
    average_character_width_px: f64,
) -> Result<f64, MeasureError> {
    Ok(
        validate_positive(container_width_px, MeasureError::InvalidContainerWidth)?
            / validate_positive(
                average_character_width_px,
                MeasureError::InvalidCharacterWidth,
            )?,
    )
}

pub fn container_width_for_measure(
    characters_per_line: f64,
    average_character_width_px: f64,
) -> Result<f64, MeasureError> {
    Ok(
        validate_positive(characters_per_line, MeasureError::InvalidCharactersPerLine)?
            * validate_positive(
                average_character_width_px,
                MeasureError::InvalidCharacterWidth,
            )?,
    )
}

pub fn is_readable_measure(characters_per_line: f64) -> Result<bool, MeasureError> {
    let characters_per_line =
        validate_positive(characters_per_line, MeasureError::InvalidCharactersPerLine)?;
    Ok((45.0..=90.0).contains(&characters_per_line))
}

#[cfg(test)]
mod tests {
    use super::{
        Measure, MeasureError, characters_per_line, container_width_for_measure,
        is_readable_measure,
    };

    #[test]
    fn computes_readable_measure_helpers() {
        let measure = Measure::new(66.0).unwrap();

        assert_eq!(measure.characters_per_line(), 66.0);
        assert!(measure.is_readable());
        assert_eq!(characters_per_line(528.0, 8.0).unwrap(), 66.0);
        assert_eq!(container_width_for_measure(66.0, 8.0).unwrap(), 528.0);
    }

    #[test]
    fn checks_readable_measure_thresholds() {
        assert!(is_readable_measure(45.0).unwrap());
        assert!(is_readable_measure(90.0).unwrap());
        assert!(!is_readable_measure(44.9).unwrap());
        assert!(!is_readable_measure(90.1).unwrap());
    }

    #[test]
    fn rejects_invalid_measure_inputs() {
        assert_eq!(
            Measure::new(0.0),
            Err(MeasureError::InvalidCharactersPerLine)
        );
        assert_eq!(
            characters_per_line(-1.0, 8.0),
            Err(MeasureError::InvalidContainerWidth)
        );
        assert_eq!(
            container_width_for_measure(66.0, 0.0),
            Err(MeasureError::InvalidCharacterWidth)
        );
    }
}
