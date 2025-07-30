Commands for setting up the environments of gtk and libadwaita:
1. `meson devenv -C gtk/_build`
2. `meson devenv -C libadwaita/_build`


TODO:
1. GLib-GIO-WARNING **: 15:58:13.474: win32 session dbus binary not found

create .msi installer inside msys64 mingw with:
`cargo wix`

create .app installer on a mac with:
```
cargo bundle --release

```