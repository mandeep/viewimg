# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
- Add command line flag that disables scaling of images prior to displaying in window
- Correctly measure usable workspace across platforms rather than using arbitrary taskbar size

## [0.16.0] - 2026-07-15
- Replace deprecated image2 crate with image crate
- Remove unneeded dependencies for faster compile times
- Remove ability to resize images in render window
- Fix issue where images were not being correctly scaled to fit the display
- Show new window in the center of the display
- Print to stdout whenever an image is resized
- Fix redraws occurring in the window when a redraw is not needed

## [0.15.0] - 2026-07-15
- Add Q as a key to quit the application

## [0.14.0] - 2026-07-14
- Fix issue where images weren't being scaled properly on hi-DPI displays
- Remove window decorations. Now Esc and Left Mouse click perform actions

## [0.13.2] - 2026-07-14
- Fix issue where compensate function was not being called on EXR image buffers

## [0.13.1] - 2026-04-27
- Retrieve version from Cargo.toml so that CLI --version output is always up to date

## [0.13.0] - 2026-04-26
- Add window icon

## [0.12.0] - 2026-04-26
- Fix issue where minimizing window caused the program to crash

## [0.11.0] - 2026-04-21
- Exclude test images from published crate to reduce crate size
- Handle errors when pixel buffer is unable to render and resize


## [0.10.0] - 2024-04-24
- Updated pixels dependency
- Change screenshot to show test


## [0.9.0] - 2024-04-24
### Changed
- Updated exr dependency
- Add more test images


## [0.8.1] - 2022-11-15
### Changed
- Updated winit dependencies in order to fix a window crash (thanks @ctrlcctrlv)


## [0.8.0] - 2020-05-06
### Added
- Iterate over large exr buffers in parallel

### Fixed
- Fix contrast being too muddled after intensity correction


## [0.7.0] - 2020-05-04
### Changed
- OpenEXR values are intensity and gamma corrected prior to clamping on the [0, 255] interval


## [0.6.0] - 2020-05-03
### Changed
- OpenEXR values are now normalized between [0.0, 1.0] instead of clamped


## [0.5.1] - 2020-04-23
### Fixed
- Pinned OpenEXR library version to fix breakage


## [0.5.0] - 2020-03-27
### Added
- Use clap crate for command line argument parsing and usage menu


## [0.4.2] - 2020-03-20
### Fixed
- Errors due to opening unsupported files now display properly to stderr


## [0.4.1] - 2020-03-14
### Changed

- Changed the OpenEXR error message to be more specific


## [0.4.0] - 2020-03-14
### Added
- Add support for F16 and U32 OpenEXR images


## [0.3.0] - 2020-03-10
### Added
- Add support of OpenEXR files via the `exr` crate


## [0.2.4] - 2020-01-27
### Fixed
- Fixed the resizing of images where the image is larger than the display and the aspect ratio is greater than 1.0


## [0.2.3] - 2020-01-13
### Added
- Added a more detailed description in the README


## [0.2.2] - 2020-01-11
### Changed
- Window title shows the filename of the opened image
- Error messages have been edited to be more clear


## [0.2.1] - 2020-01-06
### Added
- Added this CHANGELOG to the project


## [0.2.0] - 2020-01-05
### Changed
- Calculate dimensions of window with regards to the image size and primary monitor resolution


## [0.1.2] - 2019-12-03
### Added
- Add Travis CI build configuration
- Add build, license, and version badges in README

### Changed
- Pin winit dependency version to 0.20.0-alpha5


## [0.1.1] - 2019-12-03
### Added
- Add README to Cargo.toml metadata
