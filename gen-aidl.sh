#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")" && pwd)"
# Update this path to your Android AIDL binary
aidl="${AIDL:-/home/hoshikava/fod_op_ximi/tools/android-14/aidl}"
out="$root/src/gen"

rm -rf "$out"
mkdir -p "$out"

"$aidl" --lang=rust \
  -I "$root/aidl" --structured --stability=vintf --version=2 \
  --hash=beafa669f2d959dea8f2191a42c2dec41af2e9c2 \
  -o "$out" \
  "$root/aidl/vendor/oplus/hardware/engineer/IEngineer.aidl" \
  "$root/aidl/vendor/oplus/hardware/engineer/VibratorResult.aidl" \
  "$root/aidl/vendor/oplus/hardware/engineer/ReadDataResult.aidl" \
  "$root/aidl/vendor/oplus/hardware/engineer/RFNodeStatus.aidl"

find "$out" -name '*.rs' -exec sed -i '/^#!\[rustfmt::skip\]$/d' {} +