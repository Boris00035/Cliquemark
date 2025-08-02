### TODO:
1. Add gschemas / icons in the MacOS build
2. fix layout issues (buttons not stretching to fit their text)
3. icon is missing from .app file on macos
4. GLib-GIO-WARNING **: 15:58:13.474: win32 session dbus binary not found ; happens on windows machines who do not have gtk installed

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
cp -r /opt/homebrew/lib/gdk-pixbuf-2.0 target/release/bundle/osx/Cliquemark.app/Contents/libs

mkdir -p target/release/bundle/osx/Cliquemark.app/Contents/Resources/glib-2.0/schemas
cp schema_files/gschemas.compiled target/release/bundle/osx/Cliquemark.app/Contents/Resources/glib-2.0/schemas/gschemas.compiled

mkdir -p target/release/bundle/osx/Cliquemark.app/Contents/Resources/share/icons
cp -r /opt/homebrew/share/icons/Adwaita target/release/bundle/osx/Cliquemark.app/Contents/Resources/share/icons

create-dmg \
  --volname "Cliquemark Installer" \
  --volicon "assets/logo.icns" \
  --window-pos 200 120 \
  --window-size 800 400 \
  --icon-size 100 \
  --icon "Cliquemark.app" 200 190 \
  --hide-extension "Application.app" \
  --app-drop-link 600 185 \
  --skip-jenkins \
  "target/release/bundle/osx/Cliquemark-Installer.dmg" \
  "target/release/bundle/osx"
```
