#!/usr/bin/env bash
# X6: can the workspace crates be type-checked for Apple targets from a non-Mac machine?
# Decides whether the CI check in METAL_DESIGN.md D2 needs a macOS runner or not.
#
#   tools/metal_experiments/x6_apple_check.sh [target ...]     (default: aarch64-apple-darwin aarch64-apple-ios)
#
# For each target and crate it runs `cargo check` and prints one line: pass / FAIL / n.a.
# `cargo check` needs the target's rust-std (rustup target add) but no Apple SDK as long as no crate
# in the graph has a build script that probes the SDK or a C/ObjC compile step; any such crate shows
# up as a FAIL with its build-script error, which is exactly the information X6 is after.
#
# Exit status: 0 only if every requested check passed.
set -uo pipefail
cd "$(dirname "$0")/../.."
TARGETS=("$@"); [ ${#TARGETS[@]} -eq 0 ] && TARGETS=(aarch64-apple-darwin aarch64-apple-ios)
CRATES=(rhapsody country-core tiny-glade)
export PATH="$HOME/.cargo/bin:$PATH"
status=0
for target in "${TARGETS[@]}"; do
  if ! rustup target list --installed 2>/dev/null | grep -qx "$target"; then
    echo "[$target] installing rust-std ..."
    if ! rustup target add "$target" >/tmp/x6_rustup_$target.log 2>&1; then
      echo "[$target] n.a.: rustup target add failed (see /tmp/x6_rustup_$target.log): $(tail -1 /tmp/x6_rustup_$target.log | cut -c1-160)"
      status=1; continue
    fi
  fi
  for crate in "${CRATES[@]}"; do
    log="/tmp/x6_${target}_${crate}.log"
    if cargo check -p "$crate" --target "$target" >"$log" 2>&1; then
      echo "[$target] $crate: pass"
    else
      status=1
      echo "[$target] $crate: FAIL  first error: $(grep -m1 -E '^error' "$log" | cut -c1-200)   (log: $log)"
    fi
  done
done
exit $status
