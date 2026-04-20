#!/usr/bin/env bash
set -e

LIB_NAME="mimir_eqsat"
TARGET_DIR="target"
DIST_DIR="dist"
WINDBG_CONFIG=".cargo/windbg-config.toml"

echo "Creating dist/"
mkdir -p "$DIST_DIR"
mkdir -p "$DIST_DIR/include"
mkdir -p "$DIST_DIR/src"
mkdir -p "$DIST_DIR/lib/windows"
mkdir -p "$DIST_DIR/lib/windows/debug"
mkdir -p "$DIST_DIR/lib/windows/release"
mkdir -p "$DIST_DIR/lib/linux"
mkdir -p "$DIST_DIR/lib/linux/debug"
mkdir -p "$DIST_DIR/lib/linux/release"
mkdir -p "$DIST_DIR/lib/macos"
mkdir -p "$DIST_DIR/lib/macos/debug"
mkdir -p "$DIST_DIR/lib/macos/release"

if [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
  echo "Building windows debug"
  cargo build --config "$WINDBG_CONFIG"

  echo "Building windows release"
  cargo build --release

  echo "Copying Libraries"
  cp "$TARGET_DIR/debug/${LIB_NAME}.lib" "$DIST_DIR/lib/windows/debug/" || true
  cp "$TARGET_DIR/release/${LIB_NAME}.lib" "$DIST_DIR/lib/windows/release/" || true

elif [[ "$OSTYPE" == "darwin"* ]]; then
  echo "Building macos debug"
  cargo build

  echo "Building macos release"
  cargo build --release

  echo "Copying libraries"
  cp "$TARGET_DIR/debug/lib${LIB_NAME}.a" "$DIST_DIR/lib/macos/debug/" || true
  cp "$TARGET_DIR/release/lib${LIB_NAME}.a" "$DIST_DIR/lib/macos/release/" || true

else
  echo "Building linux debug"
  cargo build

  echo "Building linux release"
  cargo build --release

  echo "Copying libraries"
  cp "$TARGET_DIR/debug/lib${LIB_NAME}.a" "$DIST_DIR/lib/linux/debug/" || true
  cp "$TARGET_DIR/release/lib${LIB_NAME}.a" "$DIST_DIR/lib/linux/release/" || true
fi

BRIDGE_DIR=$(find "$TARGET_DIR/cxxbridge" -type d -path "*/src" | head -n 1)

if [ -z "$BRIDGE_DIR" ]; then
  echo "Could not find cxxbridge directory"
  exit 1
fi

HEADER=$(find "$BRIDGE_DIR" -name "*.h" | head -n 1)
SOURCE=$(find "$BRIDGE_DIR" -name "*.cc" | head -n 1)

echo "Copying bridge files"
cp "$HEADER" "$DIST_DIR/include/${LIB_NAME}.h"
cp "$SOURCE" "$DIST_DIR/src/${LIB_NAME}.cc"

echo "Done!"
