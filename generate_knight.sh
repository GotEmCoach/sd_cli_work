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
