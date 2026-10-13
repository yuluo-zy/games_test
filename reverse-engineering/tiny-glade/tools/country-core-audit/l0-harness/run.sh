#!/usr/bin/env bash
# 把 harness 复制到临时目录，把 src 里的 @SRC@ 替换成真实 crate 的绝对路径，再离线跑测试。
# 用法: ./run.sh [cargo test 的额外参数]
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
real="${L0H_REAL:-$(cd "$here/../../../reconstruction/country-core/src" && pwd)}"
work="${L0H_WORK:-/tmp/l0h-work}"
rm -rf "$work" && mkdir -p "$work"
cp -r "$here"/. "$work"/
find "$work/src" -name '*.rs' -print0 | xargs -0 sed -i "s|@SRC@|$real|g"
export PATH="$HOME/.cargo/bin:$PATH"
cd "$work" && CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-/tmp/l0h-target}" cargo test --offline "$@"
