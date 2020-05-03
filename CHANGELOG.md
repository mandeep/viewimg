# Changelog
All notable changes to this project will be documented in this file.


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
