# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Better window abstraction to easily write text.
- Debug functionality for applications.

## [0.3.0] - 2025-08-10

### Added

- Add new example using scenes.

### Changed

- BREAKING remove unnecessary as_any method for Asset.
- Simplified the README for now.

## [0.2.0] - 2025-08-04

### Added

- ISO Dates on the CHANGELOG releases
- Links to the diffs for each release version.

### Changed

- Export only currently relevant parts of the sdl2 module.
- Minor simplification of the example app.

## [0.1.0] - 2025-08-03

### Added

- API to create an SDL2 application with a classic update+render game loop. Behavior is defined by the user's implementation.
- Fast, convenient and thread safe logging to console and rolling files using `tracing`.
- Generic and memory efficient Asset manager.
- Example for basic usage of the API.

[unreleased]: https://github.com/SundeMarius/Neuton/compare/v0.2.0...HEAD
[0.3.0]: https://github.com/SundeMarius/Neuton/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/SundeMarius/Neuton/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/SundeMarius/Neuton/releases/tag/v0.1.0