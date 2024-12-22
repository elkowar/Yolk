# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.17](https://github.com/elkowar/yolk/compare/v0.0.16...v0.0.17) - 2024-12-22

### Added

- Allow for both .config and standard ~/Library/... dir on mac

## [0.0.16](https://github.com/elkowar/yolk/compare/v0.0.15...v0.0.16) - 2024-12-22

### Fixed

- yolk git --force-canonical flag being bad

## [0.0.15](https://github.com/elkowar/yolk/compare/v0.0.14...v0.0.15) - 2024-12-22

### Added

- Sync to canonical mode on git pull as well

## [0.0.14](https://github.com/elkowar/yolk/compare/v0.0.13...v0.0.14) - 2024-12-22

### Added

- Add support for importing files in yolk.rhai
- support multiline tags
- Add a few more comment symbols

### Fixed

- Yolk not removing dead symlinks when deploying in put mode
- Prevent yolk from comitting inconsistent state when syncing fails

## [0.0.13](https://github.com/elkowar/yolk/compare/v0.0.12...v0.0.13) - 2024-12-18

### Added

- [**breaking**] Add explicit deployment strategies, default to put
- add main_file config for smarter yolk edit command
- Add more flexible loglevel configuration

### Fixed

- Yolk not removing dead symlinks when deploying eggs

## [0.0.12](https://github.com/elkowar/yolk/compare/v0.0.11...v0.0.12) - 2024-12-16

### Added

- [**breaking**] Add --no-sync to yolk watch
- don't canonicalize templates when running yolk git push
- support globs in templates-declaration
- [**breaking**] Rename yolk.lua to yolk.luau

### Other

- Add link to docs to readme
- Update cargo dist, fix clippy warnings
- Update dependencies
- Fix autodocs being local path dependency
- Add test for default rhai file
- Fix is_deployed() not working
- Add TODO comment
- Cleanup
- Load yolk.rhai as module
- Generate documentation for rhai API via rhai-autodocs
- Add `yolk docs` command to generate documentation
- Fix clippy warnings
- Fix systeminfo getters
- Fix watch not properly reading templates
- Move back to rhai
- move build-setup.yaml out of workflows dir
- Various cleanups
- Start work on declarative egg deployment config
- Move back to global-artifacts-jobs for man

## [0.0.11](https://github.com/elkowar/yolk/compare/v0.0.10...v0.0.11) - 2024-12-13

### Added

- Implement `yolk watch` command
- add a few hex color utility functions

### Fixed

- Improve parser error message for missing end tag

### Other

- Try harder to make @druskus20 happy
- Slightly clean up parser code
- Improve error message for empty tag
- Update cargo dist to 0.26, try to use include and build-setup for man ([#9](https://github.com/elkowar/yolk/pull/9))
- Use different font for docs headings to make @druskus20 happy
- he animated now
- Try to fix theme
- Setup matching mdbook theme
- *(release)* build man page as part of release process

## [0.0.10](https://github.com/elkowar/yolk/compare/v0.0.9...v0.0.10) - 2024-12-09

### Added

- add yolk edit command
- Add to_json and from_json lua functions
- ensure template expressions are sandboxed
- add contains_key, contains_value, regex_captures functions

### Fixed

- join lines in parser where possible

### Other

- fix clippy warnings
- ensure that yolk_templates can deal with missing files
- Document inspect.lua library
- Add new utility functions to docs
- document yolk safeguard and how to clone
- simplify parser slightly
- enable tagging in release-plz
- Update references to replace function

## [0.0.9](https://github.com/elkowar/yolk/compare/v0.0.8...v0.0.9) - 2024-12-09

### Added

- [**breaking**] rename replace to replace_re (r -> rr)
- add replace_quoted, replace_value functions
- add replace_number tag function

### Fixed

- rename mktmpl to make-template
- show proper errors for yolk eval
- show source in errors in yolk.rhai
- parser not preserving newline after conditional end tag

### Other

- add more tests to lua functions
- disable dependency updates in release-plz config
