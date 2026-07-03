#!/usr/bin/env sh
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
BUILD_DIR=${BUILD_DIR:-"$ROOT/target/dhrystone-z80"}
SRC_DIR=${DHRYSTONE_SRC:-"$BUILD_DIR/dhrystone"}
OUT_IHX=${OUT_IHX:-"$BUILD_DIR/dhrystone-z80.ihx"}
OUT_BIN=${OUT_BIN:-"$BUILD_DIR/dhrystone-z80.bin"}
PATCHED_1="$BUILD_DIR/dhry_1_sdcc.c"
PATCHED_2="$BUILD_DIR/dhry_2_sdcc.c"
PATCH_HEADER="$BUILD_DIR/dhry_sdcc_compat.h"
STUBS="$BUILD_DIR/dhry_sdcc_stubs.c"
DHRYSTONE_RUNS=${DHRYSTONE_RUNS:-5000}

if ! command -v sdcc >/dev/null 2>&1; then
    printf '%s\n' "sdcc was not found. Install SDCC, then rerun this script."
    printf '%s\n' "Example: DHRYSTONE_SRC=/path/to/dhrystone $0"
    exit 1
fi

SDCC_BIN=$(dirname -- "$(command -v sdcc)")
if [ -x "$HOME/scoop/apps/sdcc/current/bin/sdcc.exe" ]; then
    SDCC_BIN="$HOME/scoop/apps/sdcc/current/bin"
fi
PATH="$SDCC_BIN:$PATH"
export PATH

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

cat >"$PATCH_HEADER" <<'EOF'
void *malloc(unsigned int size);
char *strcpy(char *dest, const char *src);
int strcmp(const char *left, const char *right);
void *memcpy(void *dest, const void *src, unsigned int size);
int scanf(const char *format, ...);
long time(long *timer);

void Proc_1(Rec_Pointer Ptr_Val_Par);
void Proc_2(One_Fifty *Int_Par_Ref);
void Proc_3(Rec_Pointer *Ptr_Ref_Par);
void Proc_4(void);
void Proc_5(void);
void Proc_6(Enumeration Enum_Val_Par, Enumeration *Enum_Ref_Par);
void Proc_7(One_Fifty Int_1_Par_Val, One_Fifty Int_2_Par_Val, One_Fifty *Int_Par_Ref);
void Proc_8(Arr_1_Dim Arr_1_Par_Ref, Arr_2_Dim Arr_2_Par_Ref, int Int_1_Par_Val, int Int_2_Par_Val);
Enumeration Func_1(Capital_Letter Ch_1_Par_Val, Capital_Letter Ch_2_Par_Val);
Boolean Func_2(Str_30 Str_1_Par_Ref, Str_30 Str_2_Par_Ref);
Boolean Func_3(Enumeration Enum_Par_Val);
EOF

awk '
    /extern char[[:space:]]+\*malloc[[:space:]]*\(\);/ { next }
    /Enumeration[[:space:]]+Func_1[[:space:]]*\(\);/ { next }
    { print }
    /^#include "dhry.h"/ { print "#include \"dhry_sdcc_compat.h\"" }
' "$SRC_DIR/v2.1/dhry_1.c" >"$PATCHED_1"

awk '
    { print }
    /^#include "dhry.h"/ { print "#include \"dhry_sdcc_compat.h\"" }
' "$SRC_DIR/v2.1/dhry_2.c" >"$PATCHED_2"

perl -0pi -e 's/main \(\)/int main(void)/g;
    s/\{\n    int n;\n    scanf \("%d", &n\);\n    Number_Of_Runs = n;\n  \}/Number_Of_Runs = DHRYSTONE_RUNS;/s;
    s/Proc_1 \(Ptr_Val_Par\)(.*?)\nREG Rec_Pointer Ptr_Val_Par;/void Proc_1(Rec_Pointer Ptr_Val_Par)$1/s;
    s/Proc_2 \(Int_Par_Ref\)(.*?)\nOne_Fifty\s+\*Int_Par_Ref;/void Proc_2(One_Fifty *Int_Par_Ref)$1/s;
    s/Proc_3 \(Ptr_Ref_Par\)(.*?)\nRec_Pointer \*Ptr_Ref_Par;\n/void Proc_3(Rec_Pointer *Ptr_Ref_Par)$1\n/s;
    s/Proc_4 \(\)/void Proc_4(void)/g;
    s/Proc_5 \(\)/void Proc_5(void)/g;' "$PATCHED_1"

perl -0pi -e 's/Proc_6 \(Enum_Val_Par, Enum_Ref_Par\)(.*?)\nEnumeration\s+Enum_Val_Par;\nEnumeration \*Enum_Ref_Par;/void Proc_6(Enumeration Enum_Val_Par, Enumeration *Enum_Ref_Par)$1/s;
    s/Proc_7 \(Int_1_Par_Val, Int_2_Par_Val, Int_Par_Ref\)(.*?)\nOne_Fifty\s+Int_1_Par_Val;\nOne_Fifty\s+Int_2_Par_Val;\nOne_Fifty\s+\*Int_Par_Ref;/void Proc_7(One_Fifty Int_1_Par_Val, One_Fifty Int_2_Par_Val, One_Fifty *Int_Par_Ref)$1/s;
    s/Proc_8 \(Arr_1_Par_Ref, Arr_2_Par_Ref, Int_1_Par_Val, Int_2_Par_Val\)(.*?)\nArr_1_Dim\s+Arr_1_Par_Ref;\nArr_2_Dim\s+Arr_2_Par_Ref;\nint\s+Int_1_Par_Val;\nint\s+Int_2_Par_Val;/void Proc_8(Arr_1_Dim Arr_1_Par_Ref, Arr_2_Dim Arr_2_Par_Ref, int Int_1_Par_Val, int Int_2_Par_Val)$1/s;
    s/Enumeration Func_1 \(Ch_1_Par_Val, Ch_2_Par_Val\)(.*?)\nCapital_Letter\s+Ch_1_Par_Val;\nCapital_Letter\s+Ch_2_Par_Val;/Enumeration Func_1(Capital_Letter Ch_1_Par_Val, Capital_Letter Ch_2_Par_Val)$1/s;
    s/Boolean Func_2 \(Str_1_Par_Ref, Str_2_Par_Ref\)(.*?)\nStr_30\s+Str_1_Par_Ref;\nStr_30\s+Str_2_Par_Ref;/Boolean Func_2(Str_30 Str_1_Par_Ref, Str_30 Str_2_Par_Ref)$1/s;
    s/Boolean Func_3 \(Enum_Par_Val\)(.*?)\nEnumeration Enum_Par_Val;/Boolean Func_3(Enumeration Enum_Par_Val)$1/s;' "$PATCHED_2"

cat >"$STUBS" <<EOF
int putchar(int ch)
{
    return ch;
}

int scanf(const char *format, int *value)
{
    (void)format;
    *value = $DHRYSTONE_RUNS;
    return 1;
}
EOF

sdcc \
    -mz80 \
    --std-c89 \
    --opt-code-speed \
    --max-allocs-per-node 10000 \
    -DNOENUMS \
    -DTIME \
    -DDHRYSTONE_RUNS="$DHRYSTONE_RUNS" \
    -I"$SRC_DIR/v2.1" \
    -I"$BUILD_DIR" \
    -c \
    "$PATCHED_1" \
    -o "$BUILD_DIR/dhry_1.rel"

sdcc \
    -mz80 \
    --std-c89 \
    --opt-code-speed \
    --max-allocs-per-node 10000 \
    -DNOENUMS \
    -DTIME \
    -I"$SRC_DIR/v2.1" \
    -I"$BUILD_DIR" \
    -c \
    "$PATCHED_2" \
    -o "$BUILD_DIR/dhry_2.rel"

sdcc \
    -mz80 \
    --std-c89 \
    --opt-code-speed \
    --max-allocs-per-node 10000 \
    -c \
    "$STUBS" \
    -o "$BUILD_DIR/dhry_sdcc_stubs.rel"

sdcc \
    -mz80 \
    "$BUILD_DIR/dhry_1.rel" \
    "$BUILD_DIR/dhry_2.rel" \
    "$BUILD_DIR/dhry_sdcc_stubs.rel" \
    -o "$OUT_IHX"

if command -v makebin >/dev/null 2>&1; then
    makebin -p "$OUT_IHX" "$OUT_BIN"
    printf '%s\n' "Built $OUT_IHX and $OUT_BIN"
else
    printf '%s\n' "Built $OUT_IHX"
    printf '%s\n' "makebin was not found, so no flat .bin was produced."
fi
