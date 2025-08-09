# Cliquemark
Basic app to apply a watermark onto an image.

Select a folder with the images you would like to apply the watermark to, choose your watermark and the settings, and a watermarked copy will be saved in a new folder.

### Potential features to add:
1. Changing opacity of watermark

### TODO:
1. Setting up github Actions to create a release
2. Compressing .msi file, macOS installer is 3-4x smaller with the same libraries

## Build instructions:

### Windows
create .msi installer inside msys64 mingw with (need some packages and environment variables): `cargo wix`

### MacOS
create .dmg installer on a mac with the included install script:
`./build_macOS_installer.sh`
