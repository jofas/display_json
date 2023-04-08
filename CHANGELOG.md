# CHANGELOG

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.2.1]

### Added

* `FromStrAsJson` procedural macro


## [0.2.0]

### Added

* Documentation

### Changed

* Moved `display_json_derive` functionality into this crate, making
  `display_json_derive` obsolete


## [0.1.3]

### Fixed

* Types with generic type parameters did not compile


## [0.1.2]

### Changed

* `display_json_derive` now depends on `proc-macro2 ^1.0.24` instead of
  `^1.0.26`


## [0.1.1]

### Changed

* Changed the license field in `Cargo.toml` to `MIT`, instead of
  referencing the file
