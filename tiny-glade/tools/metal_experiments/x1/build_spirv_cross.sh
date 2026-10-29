#!/usr/bin/env bash
# Build the SPIRV-Cross CLI for X1. NOT run by the experiment tooling: it clones third-party
# source from GitHub, so a human decides when to run it.
#
# Usage: build_spirv_cross.sh [install_dir]     (default: $HOME/tools/spirv-cross)
#   SPIRV_CROSS_TAG=<tag>  override the pinned tag (results are only comparable per tag)
#   CXX=<compiler>         default g++ (clang++ on macOS)
#
# Plain compiler invocation, so neither cmake nor ninja is required.
set -euo pipefail
TAG="${SPIRV_CROSS_TAG:-vulkan-sdk-1.4.350.1}"
DEST="${1:-$HOME/tools/spirv-cross}"
CXX="${CXX:-g++}"
SRC="$(mktemp -d)/SPIRV-Cross"

git clone --depth 1 --branch "$TAG" https://github.com/KhronosGroup/SPIRV-Cross "$SRC"
mkdir -p "$DEST"
cd "$SRC"
"$CXX" -std=c++11 -O1 -I. \
  spirv_cfg.cpp spirv_cpp.cpp spirv_cross.cpp spirv_cross_c.cpp spirv_cross_parsed_ir.cpp \
  spirv_cross_util.cpp spirv_glsl.cpp spirv_hlsl.cpp spirv_msl.cpp spirv_parser.cpp \
  spirv_reflect.cpp main.cpp -o "$DEST/spirv-cross" -lpthread
git rev-parse HEAD > "$DEST/SPIRV-Cross.commit"
echo "$TAG" > "$DEST/SPIRV-Cross.tag"
echo "built $DEST/spirv-cross  tag=$TAG  commit=$(cat "$DEST/SPIRV-Cross.commit")"
