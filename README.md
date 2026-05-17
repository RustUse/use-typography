# use-typography

Composable primitive typography utilities for Rust.

`use-typography` is part of RustUse, alongside sibling repositories such as
`use-math`, `use-stats`, `use-optimization`, `use-simulation`,
`use-control`, `use-signal`, `use-graph`, `use-materials`,
`use-accessibility`, `use-color`, `use-text`, `use-time`, and
`use-units`. It groups small, focused crates for font sizing, line-height
ratios, readable measure, modular scales, spacing systems, text block
estimation, typographic unit conversion, and vertical rhythm helpers.

The RustUse approach in this workspace stays intentionally narrow:

- crates stay small and independently useful
- APIs stay explicit, documented, tested, and composable
- implementations favor practical `f64`, `usize`, `String`, `&str`, and small enums or structs
- dependencies stay minimal so each crate is easy to audit and adopt

These crates provide typographic calculation helpers, not a full layout
engine. They do not depend on browser, DOM, font-rendering, or graphics
libraries.

## Workspace crates

- `use-typography`: thin facade crate that reexports the full typography workspace
- `use-font-size`: font-size and `px`/`rem`/`em` conversion helpers
- `use-line-height`: line-height ratio and readability helpers
- `use-typography-measure`: readable line-length helpers, exposed in code as `use_measure`
- `use-modular-scale`: modular typography scale helpers
- `use-spacing-scale`: spacing scale generation helpers
- `use-text-block`: text block line-count and height estimation helpers
- `use-type-unit`: `px`, `rem`, `em`, and `pt` conversion helpers
- `use-type-rhythm`: baseline grid and vertical rhythm helpers

## Facade crate

If you want one dependency for the whole workspace, use `use-typography`.
It reexports each focused crate and exposes the focused APIs directly so this
works:

```rust
use use_typography::*;

let font = FontSize::new(18.0).unwrap();
let line_height = LineHeight::new(18.0, 27.0).unwrap();
let scale = modular_scale(18.0, ScaleRatio::MajorThird, -1, 1).unwrap();

assert!((font.rem(16.0).unwrap() - 1.125).abs() < 1.0e-12);
assert!((line_height.ratio() - 1.5).abs() < 1.0e-12);
assert_eq!(scale.len(), 3);
```

## Status

This workspace is experimental while it remains below `0.3.0`. Expect the
public API to stay small and practical, but still evolve as the RustUse
typography surface becomes clearer.

## Development

Run the standard workspace checks from the repository root:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo doc --workspace --no-deps
```
