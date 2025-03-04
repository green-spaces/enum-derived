# 0.9.2 (March 3st 2025)

### Added

- Updated docs
- Added re-export of rand crate so that consumers do not have to import it in their cargo.toml

# 0.9.1 (March 1st 2025)

### Added

- Updated docs

# 0.9.0 (March 1st 2025)

### Added

Updates to the Rand trait:
- Support for Strings (defaults to lengths betwen 1 and 64)
- Support for Vec<T> where T implments Rand (defaults to lengths betwen 1 and 64)
- Support HashMap<K, V> where K and V implement Rand (defaults to size betwen 1 and 16)
- Support HashSet<K> where K implements Rand (defaults to size betwen 1 and 16)
- Added unsafe code for generating random arrays

### Removed

- Default implmentations for Rand based on the rand::distribution::Distribution becuase it prevents String and Vec<T> implmentations
- Support for SIMD types (if you need this please open an issue)
- Support for  rand::distributions::DistMap (if you need this please open an issue)
