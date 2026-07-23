#!/usr/bin/env bash
# Kova Sprite Engine - Automated Bootstrap Sequence

# Halt execution on any error
set -e

print_msg() {
    echo -e "\e[1;36m$1\e[0m"
}

print_msg "Sir, initiating the localized automated environment bootstrap sequence..."

# 1. System Dependencies
print_msg "Step 1: Verifying system build dependencies..."
sudo apt-get update
sudo apt-get install -y cmake build-essential python3-venv git wget libvulkan-dev vulkan-tools glslc spirv-headers

# 2. Clone and Compile Stable Diffusion CLI
print_msg "Step 2: Securing and compiling stable-diffusion.cpp for Vulkan..."
if [ ! -d "$HOME/stable-diffusion.cpp" ]; then
    git clone --recurse-submodules https://github.com/leejet/stable-diffusion.cpp.git "$HOME/stable-diffusion.cpp"
else
    print_msg "stable-diffusion.cpp repository already exists, skipping clone."
fi

mkdir -p "$HOME/stable-diffusion.cpp/build"
cd "$HOME/stable-diffusion.cpp/build"
# Configure for Vulkan explicitly to utilize the AMD GPU
cmake .. -DSD_VULKAN=ON
cmake --build . --config Release -j$(nproc)

# 3. Sync the GitHub Repository First
print_msg "Step 3: Cloning your workspace via SSH..."
if [ ! -d "$HOME/sd_cli_work" ]; then
    cd "$HOME"
    git clone git@github.com:gotemcoach/sd_cli_work.git
else
    print_msg "Workspace ~/sd_cli_work already present. Pulling latest changes..."
    cd "$HOME/sd_cli_work"
    git pull
fi

# 4. Download Standalone Models & LoRAs directly to workspace
print_msg "Step 4: Downloading independent SD 1.5 checkpoint and Pixel Art LoRA..."
mkdir -p "$HOME/sd_cli_work/models/checkpoints"
mkdir -p "$HOME/sd_cli_work/models/loras"

# Download SD 1.5 pruned checkpoint if not already present
if [ ! -f "$HOME/sd_cli_work/models/checkpoints/v1-5-pruned-emaonly.safetensors" ]; then
    wget -q --show-progress -O "$HOME/sd_cli_work/models/checkpoints/v1-5-pruned-emaonly.safetensors" \
      https://huggingface.co/stable-diffusion-v1-5/stable-diffusion-v1-5/resolve/main/v1-5-pruned-emaonly.safetensors
else
    print_msg "SD 1.5 checkpoint already exists, skipping download."
fi

# Download Pixel Art Redmond LoRA if not already present
if [ ! -f "$HOME/sd_cli_work/models/loras/PixelArtRedmond15V.safetensors" ]; then
    wget -q --show-progress -O "$HOME/sd_cli_work/models/loras/PixelArtRedmond15V.safetensors" \
      https://huggingface.co/artificialguybr/pixelartredmond-1-5v-pixel-art-loras-for-sd-1-5/resolve/main/PixelArtRedmond15V-PixelArt-PIXARFK.safetensors
else
    print_msg "Pixel Art LoRA already exists, skipping download."
fi

# 5. Python Virtual Environment Configuration (Localized)
print_msg "Step 5: Provisioning localized Python virtual environment and dependencies..."
cd "$HOME/sd_cli_work"
if [ ! -d "venv" ]; then
    python3 -m venv venv
fi
source venv/bin/activate
pip install --upgrade pip
pip install "rembg[cpu,cli]" pillow

# 6. Generate the Pipeline Execution Script
print_msg "Step 6: Writing the local generation pipeline to your workspace..."
cat << 'EOF' > "$HOME/sd_cli_work/generate_knight.sh"
#!/usr/bin/env bash

# Activate local virtual environment
source "$HOME/sd_cli_work/venv/bin/activate"

# Local LoRA Path Verification
LORA_PATH="$HOME/sd_cli_work/models/loras/PixelArtRedmond15V.safetensors"
if [ ! -f "$LORA_PATH" ]; then
    echo "Warning: LoRA file not found at $LORA_PATH. Please verify the filename."
    exit 1
fi

echo "Generating the base character..."
$HOME/stable-diffusion.cpp/build/bin/sd-cli \
  -m "$HOME/sd_cli_work/models/checkpoints/v1-5-pruned-emaonly.safetensors" \
  --lora-model-dir "$HOME/sd_cli_work/models/loras/" \
  -p "pixel art, 16-bit retro sprite, a highly detailed full body armored medieval knight standing facing forward, wearing a detailed iron visor helmet and full detailed plate armor, holding a detailed broadsword and a detailed shield, realistic metallic armor texture shading, clean background, single character, <lora:PixelArtRedmond15V.safetensors:0.75>" \
  -n "throne, pedestal, wings, scenery, background detail, modern clothes, beanie, t-shirt, simple blocks, text, signature, watermark" \
  --cfg-scale 8.0 -H 512 -W 512 --steps 25 \
  -o /tmp/raw_sprite.png

echo "Stripping background..."
rembg i /tmp/raw_sprite.png /tmp/transparent_sprite.png

echo "Applying pure 16-bit nearest-neighbor grid snap..."
python3 -c "from PIL import Image; img = Image.open('/tmp/transparent_sprite.png'); small = img.resize((128, 128), Image.Resampling.NEAREST); big = small.resize((1024, 1024), Image.Resampling.NEAREST); big.save('./final_pixel_knight.png')"

echo -e "\e[1;36mSir, your freshly generated and properly pixel-snapped knight asset is ready at ./final_pixel_knight.png\e[0m"
EOF

chmod +x "$HOME/sd_cli_work/generate_knight.sh"

print_msg "All systems operational, Sir. The bootstrap sequence has concluded."
print_msg "You may now run ./generate_knight.sh."

