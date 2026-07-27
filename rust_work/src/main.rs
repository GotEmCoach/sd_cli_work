use std::process::{Command};
use clap::{Arg, Command as ClapCommand};
use std::path::Path;
use std::io;
use std::fs;
use home::home_dir; // Import home_dir function from home crate
mod generate_knight; // Declare the generate_knight module

fn main() -> io::Result<()> {
    // Initialize clap command-line parser
    let matches = ClapCommand::new("SD CLI Auto Setup")
        .about("Automates setup for Stable Diffusion CLI")
        .arg(
            Arg::new("no-dependencies")
                .short('d')
                .long("no-dependencies")
                .help("Skip installing system dependencies")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-clone")
                .short('c')
                .long("no-clone")
                .help("Skip cloning repositories")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-download")
                .short('l')
                .long("no-download")
                .help("Skip downloading models and LoRAs")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-venv")
                .short('v')
                .long("no-venv")
                .help("Skip setting up Python virtual environment")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("base-knight")
                .short('k')
                .long("base-knight")
                .help("Generate a base knight character")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Extract flags
    let no_dependencies = matches.get_flag("no-dependencies");
    let no_clone = matches.get_flag("no-clone");
    let no_download = matches.get_flag("no-download");
    let no_venv = matches.get_flag("no-venv");
    let workspace_dir = Path::new(&home_dir().unwrap()).join(".sd_cli_work");
    let base_knight_flag: bool = matches.get_flag("base-knight");


    // 1. System Dependencies
    if !no_dependencies {
        println!("Step 1: Verifying system build dependencies...");
        Command::new("sudo")
            .arg("apt-get")
            .arg("update")
            .output()
            .expect("Failed to update package list");

        Command::new("sudo")
            .arg("apt-get")
            .arg("install")
            .arg("-y")
            .arg("cmake")
            .arg("build-essential")
            .arg("python3-venv")
            .arg("git")
            .arg("wget")
            .arg("libvulkan-dev")
            .arg("vulkan-tools")
            .arg("glslc")
            .arg("spirv-headers")
            .output()
            .expect("Failed to install system dependencies");
    }

    // 2. Clone and Compile Stable Diffusion CLI
    if !no_clone {
        println!("Step 2: Securing and compiling stable-diffusion.cpp for Vulkan...");
        let sd_dir = Path::new(&home_dir().unwrap()).join("stable-diffusion.cpp");

        if !sd_dir.exists() {
            println!("stable-diffusion.cpp repository not found, cloning...");
            Command::new("git")
                .arg("clone")
                .arg("--recurse-submodules")
                .arg("https://github.com/leejet/stable-diffusion.cpp.git")
                .arg(sd_dir)
                .output()
                .expect("Failed to clone stable-diffusion.cpp");
        } else {
            println!("stable-diffusion.cpp repository already exists, skipping clone.");
        }
    }
    
    // 3. Sync the GitHub Repository First
    if !no_clone {
        println!("Step 3: Cloning your workspace via SSH...");

        if !workspace_dir.exists() {
            println!("Workspace not found, cloning...");
            Command::new("git")
                .arg("clone")
                .arg("git@github.com:gotemcoach/sd_cli_work.git")
                .arg(&workspace_dir)
                .output()
                .expect("Failed to clone workspace");
        } else {
            println!("Workspace already present, pulling latest changes...");
            Command::new("git")
                .current_dir(&workspace_dir)
                .arg("pull")
                .output()
                .expect("Failed to pull workspace changes");
        }   
    }
        // 4. Download Standalone Models & LoRAs directly to workspace
    if !no_download {
        println!("Step 4: Downloading independent SD 1.5 checkpoint and Pixel Art LoRA...");
        let checkpoints_dir = workspace_dir.join("models/checkpoints");
        let loras_dir = workspace_dir.join("models/loras");

        fs::create_dir_all(&checkpoints_dir).expect("Failed to create checkpoints directory");
        fs::create_dir_all(&loras_dir).expect("Failed to create loras directory");

        let checkpoint_path = checkpoints_dir.join("v1-5-pruned-emaonly.safetensors");
        if !checkpoint_path.exists() {
            println!("SD 1.5 checkpoint not found, downloading...");
            let _wget = Command::new("wget")
                .arg("-q")
                .arg("--show-progress")
                .arg("-O")
                .arg(checkpoint_path)
                .arg("https://huggingface.co/stable-diffusion-v1-5/stable-diffusion-v1-5/resolve/main/v1-5-pruned-emaonly.safetensors")
                .output()
                .expect("Failed to download SD 1.5 checkpoint");
        } else {
            println!("SD 1.5 checkpoint already exists, skipping download.");
        }

        let lora_path = loras_dir.join("PixelArtRedmond15V.safetensors");
        if !lora_path.exists() {
            println!("Pixel Art LoRA not found, downloading...");
            let mut _wget = Command::new("wget")
                .arg("-q")
                .arg("--show-progress")
                .arg("-O")
                .arg(lora_path)
                .arg("https://huggingface.co/artificialguybr/pixelartredmond-1-5v-pixel-art-loras-for-sd-1-5/resolve/main/PixelArtRedmond15V-PixelArt-PIXARFK.safetensors")
                .output()
                .expect("Failed to download Pixel Art LoRA");
        } else {
            println!("Pixel Art LoRA already exists, skipping download.");
        }
    }

    // 5. Python Virtual Environment Configuration (Localized)
    if !no_venv {
        println!("Step 5: Provisioning localized Python virtual environment and dependencies...");
        let venv_dir = workspace_dir.join("venv");

        if !venv_dir.exists() {
            println!("Creating Python virtual environment...");
            let _python = Command::new("python3")
                .arg("-m")
                .arg("venv")
                .arg(&venv_dir)
                .output()
                .expect("Failed to create virtual environment");
        }

        let _pip = Command::new(venv_dir.join("bin").join("pip"))
            .arg("install")
            .arg("--upgrade")
            .arg("pip")
            .output()
            .expect("Failed to upgrade pip");

        let _pip_install = Command::new(venv_dir.join("bin").join("pip"))
            .arg("install")
            .arg("rembg[cpu,cli]")
            .arg("pillow")
            .output()
            .expect("Failed to install Python dependencies");
    }

        // If the base-knight flag is set, generate the base knight character
    if base_knight_flag {
        generate_knight::base_knight();
    }

    Ok(())
}

