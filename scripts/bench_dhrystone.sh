#!/usr/bin/env sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
BUILD_DIR=${BUILD_DIR:-"$ROOT/target/dhrystone-z80"}
SRC_DIR=${DHRYSTONE_SRC:-"$BUILD_DIR/dhrystone"}
OUT_IHX=${OUT_IHX:-"$BUILD_DIR/dhrystone-z80.ihx"}
OUT_BIN=${OUT_BIN:-"$BUILD_DIR/dhrystone-z80.bin"}

if ! command -v sdcc >/dev/null 2>&1; then
    printf '%s\n' "sdcc was not found. Install SDCC, then rerun this script."
    printf '%s\n' "Example: DHRYSTONE_SRC=/path/to/dhrystone $0"
    exit 1
fi

mkdir -p "$BUILD_DIR"

if [ ! -f "$SRC_DIR/v2.1/dhry_1.c" ] || [ ! -f "$SRC_DIR/v2.1/dhry_2.c" ]; then
    if ! command -v git >/dev/null 2>&1; then
        printf '%s\n' "Dhrystone sources were not found and git is unavailable."
        printf '%s\n' "Set DHRYSTONE_SRC to a checkout of https://github.com/Keith-S-Thompson/dhrystone."
        exit 1
    fi
    rm -rf "$SRC_DIR"
    git clone --depth 1 https://github.com/Keith-S-Thompson/dhrystone "$SRC_DIR"
fi

sdcc \
    -mz80 \
    --std-c89 \
    --opt-code-speed \
    --max-allocs-per-node 10000 \
    -DREG=register \
    -DNOENUMS \
    -DTIME \
    -I"$SRC_DIR/v2.1" \
    "$SRC_DIR/v2.1/dhry_1.c" \
    "$SRC_DIR/v2.1/dhry_2.c" \
    -o "$OUT_IHX"

if command -v makebin >/dev/null 2>&1; then
    makebin -p "$OUT_IHX" "$OUT_BIN"
    printf '%s\n' "Built $OUT_IHX and $OUT_BIN"
else
    printf '%s\n' "Built $OUT_IHX"
    printf '%s\n' "makebin was not found, so no flat .bin was produced."
fi
