![header](header.png "viewimg")
[![crates](https://img.shields.io/crates/v/viewimg?style=flat-square)](https://crates.io/crates/viewimg) [![license](https://img.shields.io/crates/l/viewimg?style=flat-square)](https://crates.io/crates/viewimg)

viewimg is an image viewer intended to be used with OpenEXR and Radiance HDR images, however
other popular image formats are supported as well. The goal of viewimg is to be a cross-platform
image viewer that quickly opens OpenEXR and Radiance HDR images with the purpose of instantly
seeing the image contents. There is no editing component to this application,
therefore tone mapping and other editing operators are not supported.

The typical use case of viewimg would be to view HDR images immediately after rendering.

Luminance correction is applied to both OpenEXR and Radiance HDR images. OpenEXR images
are corrected using the same compensation as [exrdisplay](https://lists.aswf.io/g/openexr-dev/messages?page=235&after=1078387999000000000&subsort=1) whereas Radiance HDR images are gamma corrected
using a gamma of 2.2.



Installation
============

viewimg can be installed via cargo with the following command in a terminal prompt:

    $  cargo install viewimg

Note: Vulkan must be installed on the target system in order for viewimg to work correctly.
For Debian users this can be done with the command `sudo apt install mesa-vulkan-drivers`.

Usage
=====

To use viewimg you can run `viewimg` in a terminal with the path to the
image file that you wish to open:

    $  viewimg tests/WideColorGamut.exr

To close the viewimg window, one can press the Esc key. Left Mouse Button click and hold
can be used to drag the window around the display.

![screenshot](screenshot.png "viewimg window screenshot")
