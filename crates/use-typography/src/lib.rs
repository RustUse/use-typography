#![forbid(unsafe_code)]
//! Thin facade for the `use-typography` workspace.
//!
//! The crate reexports the focused typography crates directly so consumers
//! can opt into one dependency while still using the smaller APIs.
//!
//! # Examples
//!
//! ```rust
//! use use_typography::*;
//!
//! let font = FontSize::new(18.0).unwrap();
//! let line_height = LineHeight::new(18.0, 27.0).unwrap();
//! let scale = modular_scale(18.0, ScaleRatio::MajorThird, -1, 1).unwrap();
//!
//! assert!((font.rem(16.0).unwrap() - 1.125).abs() < 1.0e-12);
//! assert!((line_height.ratio() - 1.5).abs() < 1.0e-12);
//! assert_eq!(scale.len(), 3);
//! ```

pub use use_font_size;
pub use use_font_size::{
    em_to_px as font_size_em_to_px, px_to_em, px_to_rem, rem_to_px as font_size_rem_to_px,
    FontSize, FontSizeError,
};
pub use use_line_height;
pub use use_line_height::{
    is_readable_line_height, line_height_px, line_height_ratio, LineHeight, LineHeightError,
};
pub use use_measure;
pub use use_measure::{
    characters_per_line, container_width_for_measure, is_readable_measure, Measure, MeasureError,
};
pub use use_modular_scale;
pub use use_modular_scale::{modular_scale, scale_down, scale_up, ModularScaleError, ScaleRatio};
pub use use_spacing_scale;
pub use use_spacing_scale::{spacing_step, spacing_steps, SpacingScale, SpacingScaleError};
pub use use_text_block;
pub use use_text_block::{estimated_line_count, estimated_text_height, TextBlock, TextBlockError};
pub use use_type_rhythm;
pub use use_type_rhythm::{baseline_grid, snap_to_baseline, TypeRhythm, TypeRhythmError};
pub use use_type_unit;
pub use use_type_unit::{em_to_px, pt_to_px, px_to_pt, rem_to_px, TypeUnit, TypeUnitError};

#[cfg(test)]
mod tests {
    use super::{
        baseline_grid, modular_scale, FontSize, LineHeight, ScaleRatio, TextBlock, TypeRhythm,
        TypeUnit,
    };

    #[test]
    fn facade_reexports_workspace_apis() {
        let font = FontSize::new(18.0).unwrap();
        let line_height = LineHeight::new(18.0, 27.0).unwrap();
        let scale = modular_scale(18.0, ScaleRatio::MajorThird, -1, 1).unwrap();
        let block = TextBlock::new(480.0, 18.0, 27.0, 120).unwrap();
        let rhythm = TypeRhythm::new(18.0, 27.0).unwrap();

        assert!((font.rem(16.0).unwrap() - 1.125).abs() < 1.0e-12);
        assert!((line_height.ratio() - 1.5).abs() < 1.0e-12);
        assert_eq!(scale.len(), 3);
        assert_eq!(block.estimated_line_count(8.0).unwrap(), 2);
        assert_eq!(
            baseline_grid(rhythm.baseline_unit(), 3).unwrap(),
            vec![27.0, 54.0, 81.0]
        );
        assert!((TypeUnit::Rem(1.5).to_px(16.0, 18.0).unwrap() - 24.0).abs() < 1.0e-12);
    }
}
