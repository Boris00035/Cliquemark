Commands for setting up the environments of gtk and libadwaita:
1. `meson devenv -C gtk/_build`
2. `meson devenv -C libadwaita/_build`


TODO:

1. GLib-GIO-WARNING **: 15:58:13.474: win32 session dbus binary not found
2. de app icon laad niet
3. adjust the installer menus, remove unnecessary information

create .msi installer inside msys64 mingw with:
cargo wix