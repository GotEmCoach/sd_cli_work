use std::process::Command;
use std::path::Path;

pub fn base_knight() {
    // Step 1: Check for LoRA file
    let lora_path = Path::new("~/.sd_cli_work/models/loras/PixelArtRedmond15V.safetensors");
    if !lora_path.exists() {
        eprintln!("Warning: LoRA file not found at {}. Please verify the filename.", lora_path.display());
        return;
    }

    // Step 2: Generate the base character using sd-cli
    let output_path = "/tmp/raw_sprite.png";
    let command = Command::new("~/.sd_cli_work/build/bin/sd-cli")
        .arg("-m")
        .arg("~/.sd_cli_work/models/checkpoints/v1-5-pruned-emaonly.safetensors")
        .arg("--lora-model-dir")
        .arg("~/.sd_cli_work/models/loras/")
        .arg("-p")
        .arg("pixel art, 16-bit retro sprite, a highly detailed full body armored medieval knight standing facing forward, wearing a detailed iron visor helmet and full detailed plate armor, holding a detailed broadsword and a detailed shield, realistic metallic armor texture shading, clean background, single character, <lora:PixelArtRedmond15V.safetensors:0.75>")
        .arg("-n")
        .arg("throne, pedestal, wings, scenery, background detail, modern clothes, beanie, t-shirt, simple blocks, text, signature, watermark")
        .arg("--cfg-scale")
        .arg("8.0")
        .arg("-H")
        .arg("512")
        .arg("-W")
        .arg("512")
        .arg("--steps")
        .arg("25")
        .arg("-o")
        .arg(output_path)
        .output()
        .expect("Failed to run sd-cli");

    if !command.status.success() {
        eprintln!("Error running sd-cli: {}", String::from_utf8_lossy(&command.stderr));
        return;
    }

    // Step 3: Strip background using rembg
    let rembg_cmd = Command::new("rembg")
        .arg("i")
        .arg("/tmp/raw_sprite.png")
        .arg("/tmp/transparent_sprite.png")
        .output()
        .expect("Failed to run rembg");

    if !rembg_cmd.status.success() {
        eprintln!("Error running rembg: {}", String::from_utf8_lossy(&rembg_cmd.stderr));
        return;
    }

    // Step 4: Apply 16-bit nearest-neighbor grid snap using PIL
    // You can use the `image` crate to handle the image resizing
    // This is a simplified version of the `image` crate code
    let img = image::open("/tmp/transparent_sprite.png")
        .expect("Failed to open image");
    let small = img.resize(128, 128, image::imageops::FilterType::Nearest);
    let big = small.resize(1024, 1024, image::imageops::FilterType::Nearest);
    big.save("final_pixel_knight.png")
        .expect("Failed to save image");

    // Step 5: Output message
    println!("Sir, your freshly generated and properly pixel-snapped knight asset is ready at final_pixel_knight.png");
}