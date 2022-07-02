#!/usr/bin/env bash

BUILDER="gtoolkit-vm-builder"

if [[ ! -f "$BUILDER" ]]; then
  if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    curl -o "$BUILDER" -LsS https://github.com/feenkcom/gtoolkit-vm-builder/releases/latest/download/gtoolkit-vm-builder-x86_64-unknown-linux-gnu
  elif [[ "$OSTYPE" == "darwin"* ]]; then
    arch_name="$(uname -m)"
    is_m1=false
    if [ "${arch_name}" = "x86_64" ]; then
      if [ "$(sysctl -in sysctl.proc_translated)" = "1" ]; then
        is_m1=true
      fi
    elif [ "${arch_name}" = "arm64" ]; then
      is_m1=true
    fi

    if [[ "$is_m1" == true ]]; then
      curl -o "$BUILDER" -LsS https://github.com/feenkcom/gtoolkit-vm-builder/releases/latest/download/gtoolkit-vm-builder-aarch64-apple-darwin
    else
      curl -o "$BUILDER" -LsS https://github.com/feenkcom/gtoolkit-vm-builder/releases/latest/download/gtoolkit-vm-builder-x86_64-apple-darwin
    fi

  elif [[ "$OSTYPE" == "cygwin" || "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    echo "$OSTYPE is unsupported. Please submit an issue at https://github.com/feenkcom/gtoolkit/issues".
    exit 1
  else
    echo "$OSTYPE is unsupported. Please submit an issue at https://github.com/feenkcom/gtoolkit/issues".
    exit 1
  fi
  chmod +x "$BUILDER"
fi

"./$BUILDER" \
  --release \
  --app-name 'GlamorousToolkit' \
  --identifier 'com.gtoolkit' \
  --author "feenk gmbh <contact@feenk.com>" \
  --libraries-versions libraries.version \
  --icons icons/GlamorousToolkit.icns \
  --executables app cli \
  --libraries boxer cairo clipboard crypto freetype git gleam glutin process sdl2 skia winit pixels test-library
