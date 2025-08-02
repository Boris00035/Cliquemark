### TODO:
1. Compressing .msi file, macOS installer is 3-4x smaller with the same libraries
2. GLib-GIO-WARNING **: 15:58:13.474: win32 session dbus binary not found ; happens on windows machines who do not have gtk installed

Commands for setting up the environments of gtk and libadwaita:
1. `meson devenv -C gtk/_build`
2. `meson devenv -C libadwaita/_build`

### Windows
create .msi installer inside msys64 mingw with (need some packages and environment variables): `cargo wix`

### MacOS
create .app installer on a mac with the included install script:
`./build_macOS_installer.sh`
