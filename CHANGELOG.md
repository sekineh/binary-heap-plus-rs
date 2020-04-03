# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

* Quickstart section in the doc

### Changed

* Bump MSRV (minimum supported rust version) to rust 1.31.1.
* [CI] Switched to Github actions.
* [CI] travis is removed because it was unreliable.
* [CI] appveyor is removed because it's slow.

## [0.2.0] - 2019-05-27

### Changed

* [COMPATIBILITY] Use `Compare` trait from `compare` crate instead of our own definition.
Most users should not be affected by this. TIP: External `Compare<T>` impls needs to be updated to use `Fn` instead of `FnMut`.
* [COMPATIBILITY] rename feature `serde1` to `serde` in order to comply with the guideline: 
https://rust-lang-nursery.github.io/api-guidelines/interoperability.html#c-serde
* Refactor ctor impl.

## [0.1.6] - 2019-05-21

### Added
* generic constructor `from_vec()` and `from_vec_cmp()`.

### Changed
* Refactor other ctor to call above methods.

## [0.1.5] - 2019-05-20

### Added
* `serde1` feature which adds Serialize/Deserialize

## [0.1.4]

### Fixed
* Merge #1) Do not require T: Ord when a custom comparator is provided

## [0.1.3] - 2018-05-14

* Add comprehensive CI based on `trust` CI template v0.1.2
* README.md tweaks.

## [0.1.2] - 2018-05-14

* Cargo.toml tweaks

## [0.1.1] - 2018-05-14

* Initial tag