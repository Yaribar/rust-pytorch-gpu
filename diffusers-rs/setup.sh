#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC_DIR="${SCRIPT_DIR}/diffusers-src"
DATA_DIR="${SCRIPT_DIR}/data"

# ── 1. Clone diffusers-rs source ──────────────────────────────────────
if [ -d "${SRC_DIR}" ]; then
    echo "diffusers-src/ already exists, skipping clone."
else
    echo "Cloning diffusers-rs..."
    git clone https://github.com/LaurentMazare/diffusers-rs "${SRC_DIR}"
fi

# ── 2. Patch tch / torch-sys versions (0.13 → 0.17) ──────────────────
echo "Patching diffusers-src/Cargo.toml to use tch 0.17..."
if grep -q 'tch = "0.13"' "${SRC_DIR}/Cargo.toml"; then
    sed -i.bak 's/tch = "0.13"/tch = "0.17"/' "${SRC_DIR}/Cargo.toml"
    sed -i.bak 's/torch-sys = "0.13"/torch-sys = "0.17"/' "${SRC_DIR}/Cargo.toml"
    rm -f "${SRC_DIR}/Cargo.toml.bak"
    echo "Patched successfully."
else
    echo "Already patched or unexpected version; skipping."
fi

# ── 3. Download Stable Diffusion v2.1 weights ────────────────────────
HF_REPO="https://huggingface.co/lmz/rust-stable-diffusion-v2-1/resolve/main"
mkdir -p "${DATA_DIR}"

download() {
    local file="$1"
    if [ -f "${DATA_DIR}/${file}" ]; then
        echo "${file} already downloaded."
    else
        echo "Downloading ${file}..."
        curl -L "${HF_REPO}/${file}" -o "${DATA_DIR}/${file}"
    fi
}

download "clip_v2.1.safetensors"
download "unet_v2.1.safetensors"
download "vae_v2.1.safetensors"
download "bpe_simple_vocab_16e6.txt"

echo "Setup complete."
