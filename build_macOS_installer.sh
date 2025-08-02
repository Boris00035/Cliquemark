#!/bin/sh

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