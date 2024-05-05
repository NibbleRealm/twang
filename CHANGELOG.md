# Changelog
All notable changes to `twang` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://github.com/AldaronLau/semver).

## [0.10.0] - Unreleased
### Changed
 - Bump MSRV to 1.70.0

## [0.9.0] - 2022-10-23
### Changed
 - Bump MSRV to 1.60.0
 - Documentation improvements

## [0.8.0] - 2022-02-03
### Added
 - no-std support
 - `ops` module for auditory effects previously provided by `Signal`
 - `osc` module for basic oscillators previously provided by `Signal`
 - `Synth::stream()` (needed now that the `Stream` trait is gone)

### Changed
 - Update to fon version 0.6
 - Move `Pink` to `noise::Pink`
 - Move `White` to `noise::White`
 - Rename `Pink::noise()` to `Pink::step()`
 - Rename `White::noise()` to `Pink::step()`
 - Move `Room` to `ops::Room`
 - Rename `Room::gen()` to `Room::step()`
 - `Room::add()` now takes seconds instead of samples for time parameter

### Removed
 - `Signal` - `fon::Ch32` is now used instead
 - `Mix` trait, you can now use `fon::Frame::pan()` to mix instead
 - `Synth::params()`
 - `Fc` - no longer any global synthesis state

## [0.7.0] - 2021-01-17
### Changed
 - Update to fon version 0.5

## [0.6.0] - 2020-12-30
### Added
 - `Synth` now implements `fon::Stream`
 - `Synth::params()`

### Changed
 - `Synth::new` now takes an additional parameter for parameterizing synthesis,
   and rather than taking a closure takes a function.
 - Update `fon` to version 0.4
 - `Signal::to_mono()` now always return `Ch64` rather than being generic.

### Removed
 - `Synth::gen()`

## [0.5.0] - 2020-12-19
### Changed
 - Update `fon` to version 0.3

## [0.4.0] - 2020-11-11
### Added
 - Add `Room` struct for creating various types of reverb and echoes. 

### Changed
 - `Mix` trait can now be used with either borrowed or unborrowed data.
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
