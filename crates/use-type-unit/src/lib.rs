#![forbid(unsafe_code)]
//! Primitive typographic unit helpers.
//!
//! These helpers convert between common typographic units using explicit root
//! and parent font sizes where required.
//!
//! # Examples
//!
//! ```rust
//! use use_type_unit::{TypeUnit, em_to_px, pt_to_px, px_to_pt, rem_to_px};
//!
//! assert!((pt_to_px(12.0).unwrap() - 15.999_996).abs() < 1.0e-12);
//! assert!((px_to_pt(16.0).unwrap() - 12.000_003_000_000_75).abs() < 1.0e-12);
//! assert!((rem_to_px(1.25, 16.0).unwrap() - 20.0).abs() < 1.0e-12);
//! assert!((em_to_px(1.5, 18.0).unwrap() - 27.0).abs() < 1.0e-12);
//! assert!((TypeUnit::Pt(12.0).to_px(16.0, 18.0).unwrap() - 15.999_996).abs() < 1.0e-12);
//! ```

const PT_IN_PX: f64 = 1.333_333;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeUnit {
    Px(f64),
    Rem(f64),
    Em(f64),
    Pt(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeUnitError {
    InvalidPx,
    InvalidRem,
    InvalidEm,
    InvalidPt,
    InvalidRootSize,
    InvalidParentSize,
}

fn validate_non_negative(value: f64, error: TypeUnitError) -> Result<f64, TypeUnitError> {
    if !value.is_finite() || value < 0.0 {
        Err(error)
    } else {
        Ok(value)
    }
}

fn validate_positive_reference(value: f64, error: TypeUnitError) -> Result<f64, TypeUnitError> {
    if !value.is_finite() || value <= 0.0 {
        Err(error)
    } else {
        Ok(value)
    }
}

impl TypeUnit {
    pub fn to_px(&self, root_px: f64, parent_px: f64) -> Result<f64, TypeUnitError> {
        match *self {
            Self::Px(px) => validate_non_negative(px, TypeUnitError::InvalidPx),
            Self::Rem(rem) => rem_to_px(rem, root_px),
            Self::Em(em) => em_to_px(em, parent_px),
            Self::Pt(pt) => pt_to_px(pt),
        }
    }
}

pub fn pt_to_px(pt: f64) -> Result<f64, TypeUnitError> {
    Ok(validate_non_negative(pt, TypeUnitError::InvalidPt)? * PT_IN_PX)
}

pub fn px_to_pt(px: f64) -> Result<f64, TypeUnitError> {
    Ok(validate_non_negative(px, TypeUnitError::InvalidPx)? / PT_IN_PX)
}

pub fn rem_to_px(rem: f64, root_px: f64) -> Result<f64, TypeUnitError> {
    Ok(validate_non_negative(rem, TypeUnitError::InvalidRem)?
        * validate_positive_reference(root_px, TypeUnitError::InvalidRootSize)?)
}

pub fn em_to_px(em: f64, parent_px: f64) -> Result<f64, TypeUnitError> {
    Ok(validate_non_negative(em, TypeUnitError::InvalidEm)?
        * validate_positive_reference(parent_px, TypeUnitError::InvalidParentSize)?)
}

#[cfg(test)]
mod tests {
    use super::{TypeUnit, TypeUnitError, em_to_px, pt_to_px, px_to_pt, rem_to_px};

    #[test]
    fn converts_between_typographic_units() {
        assert!((pt_to_px(12.0).unwrap() - 15.999_996).abs() < 1.0e-12);
        assert!((px_to_pt(16.0).unwrap() - 12.000_003_000_000_75).abs() < 1.0e-12);
        assert!((rem_to_px(1.25, 16.0).unwrap() - 20.0).abs() < 1.0e-12);
        assert!((em_to_px(1.5, 18.0).unwrap() - 27.0).abs() < 1.0e-12);
        assert!((TypeUnit::Rem(1.25).to_px(16.0, 18.0).unwrap() - 20.0).abs() < 1.0e-12);
        assert!((TypeUnit::Pt(12.0).to_px(16.0, 18.0).unwrap() - 15.999_996).abs() < 1.0e-12);
        assert_eq!(TypeUnit::Px(0.0).to_px(16.0, 18.0).unwrap(), 0.0);
    }

    #[test]
    fn rejects_invalid_unit_inputs() {
        assert_eq!(pt_to_px(-1.0), Err(TypeUnitError::InvalidPt));
        assert_eq!(px_to_pt(f64::NAN), Err(TypeUnitError::InvalidPx));
        assert_eq!(rem_to_px(1.0, 0.0), Err(TypeUnitError::InvalidRootSize));
        assert_eq!(
            em_to_px(1.0, f64::NEG_INFINITY),
            Err(TypeUnitError::InvalidParentSize)
        );
        assert_eq!(
            TypeUnit::Em(-0.5).to_px(16.0, 18.0),
            Err(TypeUnitError::InvalidEm)
        );
    }
}
