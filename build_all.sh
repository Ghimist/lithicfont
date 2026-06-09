#!/bin/bash
# lithicfont - 一個字體處理庫
#
# Copyright (c) 2026 ghimist
# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

# ...
set -e

echo "🚀 開始構建純淨版跨平臺二進制矩陣..."
mkdir -p release_bin

# ==========================================
# 0. 環境準備 (如果已安裝會自動跳過)
# ==========================================
# 添加所有目標三元組
rustup target add x86_64-unknown-linux-musl
rustup target add x86_64-pc-windows-msvc
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
rustup target add wasm32-wasip1

# ==========================================
# 1. Linux (x86_64 純靜態)
# ==========================================
echo "📦 構建 Linux (x86_64 musl)..."
cargo build --release --target x86_64-unknown-linux-musl
cp -av target/x86_64-unknown-linux-musl/release/lithicfont release_bin/lithicfont_linux_x64

# ==========================================
# 2. Windows (x86_64 MSVC)
# ==========================================
echo "📦 構建 Windows (x86_64)..."
cargo xwin build --release --target x86_64-pc-windows-msvc
cp -av target/x86_64-pc-windows-msvc/release/lithicfont.exe release_bin/lithicfont_windows_x64.exe

# ==========================================
# 3. macOS (Intel & Apple Silicon)
# ==========================================
echo "📦 構建 macOS (x86_64 & ARM64)..."
export MACOSX_DEPLOYMENT_TARGET=10.13 
cargo zigbuild --release --target x86_64-apple-darwin
cargo zigbuild --release --target aarch64-apple-darwin

cp -av target/x86_64-apple-darwin/release/lithicfont release_bin/lithicfont_macos_intel
cp -av target/aarch64-apple-darwin/release/lithicfont release_bin/lithicfont_macos_arm64

# ==========================================
# 4. WebAssembly (WASI)
# ==========================================
echo "📦 構建 WebAssembly (WASI)..."
# 生成的 .wasm 文件可以在任何安裝了 Wasmtime/Wasmer 的環境，或現代瀏覽器中運行
cargo build --release --target wasm32-wasip1
cp -av target/wasm32-wasip1/release/lithicfont.wasm release_bin/lithicfont.wasm

echo "✅ 構建完成！產物已歸檔至 release_bin/ 目錄。"