# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.3.0] - 2024-12-02

### Added
* upgraded to bevy `0.15`
* new `resolve_conflicting_games` method (#8).
* new `ConflictingSaveGames` and `ResolvedConflicts` events.
* new `BevyIosGamecenter` SystemParam for convenient request response flow based on `Observer`

### Changed
* `init` is now explicitly `init_listeners` and is still called automatically (by default).
* `authenticate` was previously implictly done inside `init` this is now an explicit call and response.
* `get_player` is not implicitly called after successful `authenticate` anymore.
* new container type `IosGCSaveGames` now used in `IosGCSaveGamesResponse`.

## [0.2.0] - 2024-07-12

### Changed
* update to bevy `0.14`

## [0.1.8] - 2024-05-11

*note:* the crate release matches the commit tagged with the `ru-` prefix because we release the swift package after the crate due to its dependency on each other

### Added
* added method `delete_savegame` to delete cloud save games
* added method `fetch_signature` to allow server side verification

## [0.1.7] - 2024-05-06

*note:* the crate release matches the commit tagged with the `ru-` prefix because we release the swift package after the crate due to its dependency on each other

### Fixed
* linker errors in windows builds using `bevy_ios_gamecenter` crate