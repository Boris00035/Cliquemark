### TODO:
1. Add gschemas / icons in the MacOS build
2. fix layout issues (buttons not stretching to fit their text)
3. GLib-GIO-WARNING **: 15:58:13.474: win32 session dbus binary not found ; happens on windows machines who do not have gtk installed

Commands for setting up the environments of gtk and libadwaita:
1. `meson devenv -C gtk/_build`
2. `meson devenv -C libadwaita/_build`

### Windows
create .msi installer inside msys64 mingw with:

`cargo wix`

### MacOS
create .app installer on a mac with:
```
cargo bundle --release
dylibbundler -od -b -x target/release/bundle/osx/Cliquemark.app/Contents/MacOS/Cliquemark -d target/release/bundle/osx/Cliquemark.app/Contents/libs
```
