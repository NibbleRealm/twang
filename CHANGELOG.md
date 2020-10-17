# Changelog
All notable changes to `twang` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://github.com/AldaronLau/semver).

## [0.4.0] - Unreleased
### Changed
 - `Mix` trait can now be used with borrowed or unborrowed data.
 - All methods on Signal that took `f64` now take `Into<Self>`
 - Rename `Sample.amp()` to `Sample.gain()`

## [0.3.0] - 2020-08-26
### Changed
 - Depend on `fon` for audio types.
 - Use simple PRNG for white noise instead of `rand` crate.
 - Replace `Wave` with `Synth` and `Fc` (frequency counter)
 - Replace `Sample` with `Signal` (which is slightly different than `fon`'s
   `Mono64`
 - Replace `SampleSlice` trait with `Mix` trait

### Removed
 - `prelude` module

## [0.2.0] - 2018-08-16
### Changed
 - Newtype'd everything.
 - Uses operator overloading now.

## [0.1.0] - 2018-08-13
### Added
 - Code
