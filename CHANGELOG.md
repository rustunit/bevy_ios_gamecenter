# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

*note:* the crate release matches the commit tagged with the `ru-` prefix because we release the swift package after the crate due to its dependency on each other

### Added
* added method `delete_savegame` to delete cloud save games
* added method `fetch_signature` to allow server side verification

## [0.1.7] - 2024-05-06

*note:* the crate release matches the commit tagged with the `ru-` prefix because we release the swift package after the crate due to its dependency on each other

### Fixed
* linker errors in windows builds using `bevy_ios_gamecenter` crate