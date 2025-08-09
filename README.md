<div align="center">
  <img src="https://github.com/Boris00035/Cliquemark/blob/fb3efdf2a7b3155c1a7a8f5aa10a4cfffd51418c/assets/my-app-icon.png"
    width=20%>
</div>


# Cliquemark
Basic app to apply a watermark onto an image.

Select a folder with the images you would like to apply the watermark to, choose your watermark and the settings, and a watermarked copy will be saved in a new folder.

## Install instructions:

Download the correct installer according to your platform and open it. ([Windows](https://github.com/Boris00035/Cliquemark/releases/download/v1.1.0/Cliquemark-0.1.0-x86_64.msi), [Mac](https://github.com/Boris00035/Cliquemark/releases/download/v1.1.0/Cliquemark-Installer.dmg)) On Windows follow the instructions given there, on MacOS just drag the app into the application folder.

Now you can open the app by searching in the searchbar!
<br/><br/>

## Build instructions:

### Windows
Building is always done inside a msys64 mingw environment. (you will need to download some packages and set some environment variables)

The build process is handled by the rust toolchain. (i.e. `cargo run --release`) 
To create the .msi installer, execute `cargo wix`. 

### MacOS
Install `gtk` and `libadwaita` with HomeBrew.
The build process is handled by the rust toolchain. (i.e. `cargo run --release`)
To create the .dmg installer execute the included install script (on MacOS): `./build_macOS_installer.sh`
<br/><br/>

### Potential features to add:
1. Changing opacity of watermark

### TODO:
1. Setting up github Actions to create a release
2. Compressing .msi file, macOS installer is 3-4x smaller with the same libraries
