# 0.9.0 (March 1st 2025)

### Added

Updates to the Rand trait:
- Support for Strings (defaults to lengths betwen 1 and 64)
- Support for Vec<T> where T implments Rand (defaults to lengths betwen 1 and 64)

### Removed

- Default implmentations for Rand based on the rand::distribution::Distribution becuase it prevents String and Vec<T> implmentations
- Support for SIMD types (if you need this please open an issue)
- Support for  rand::distributions::DistMap (if you need this please open an issue)


