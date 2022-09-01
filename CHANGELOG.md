# Changelog

<!-- next-header -->

## [Unreleased] - ReleaseDate
### Changed
- Loosen some generic trait bounds:
  - Don't require `T` to be `Copy`, only `Clone`.
  - Declare `const Q_MAX` as `u64` instead of `u32` as we use `u64` as the unsigned int type in other places.
  - Remove obsolete `'static` bound on `impl Linear for IntRange`.
- Split normalization and quantization into different traits. This gives more flexibility.

## [0.2.0] - 2022-04-11
### Changed
- Remove dependency `num_traits`. It failed silently for some cases. Generic types must instead implement `TryFrom<u64>`.

## [0.1.0] - 2022-04-11
### Added
- First implementation

<!-- next-url -->
[Unreleased]: https://github.com/vilcans/numquant/compare/numquant-v0.2.0...HEAD
[0.2.0]: https://github.com/vilcans/numquant/compare/v0.1.0...numquant-v0.2.0
[0.1.0]: https://github.com/vilcans/numquant/tag/v0.1.0
