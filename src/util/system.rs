use anyhow::{bail, Context, Result};
use std::io::{self, Write};
use std::process::{Command, Stdio};

#[derive(Debug, Clone)]
pub enum PackageManager {
    Homebrew,
    Apt,
    Yum,
    Dnf,
    Chocolatey,
}

pub fn detect_package_manager() -> Option<PackageManager> {
    // Check for Homebrew (macOS)
    if Command::new("brew")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
    {
        return Some(PackageManager::Homebrew);
    }

    // Check for apt (Debian/Ubuntu)
    if Command::new("apt")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
    {
        return Some(PackageManager::Apt);
    }

    // Check for dnf (Fedora)
    if Command::new("dnf")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
    {
        return Some(PackageManager::Dnf);
    }

    // Check for yum (RHEL/CentOS)
    if Command::new("yum")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
    {
        return Some(PackageManager::Yum);
    }

    // Check for Chocolatey (Windows)
    if Command::new("choco")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok()
    {
        return Some(PackageManager::Chocolatey);
    }

    None
}

fn prompt_user(message: &str) -> Result<bool> {
    eprint!("{} (y/n): ", message);
    io::stderr().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let trimmed = input.trim().to_lowercase();
    Ok(trimmed == "y" || trimmed == "yes")
}

fn install_ffmpeg(package_manager: PackageManager) -> Result<()> {
    let (command, args, _needs_sudo) = match package_manager {
        PackageManager::Homebrew => {
            ("brew", vec!["install", "ffmpeg"], false)
        }
        PackageManager::Apt => {
            ("sh", vec!["-c", "sudo apt-get update && sudo apt-get install -y ffmpeg"], false)
        }
        PackageManager::Dnf => {
            ("sh", vec!["-c", "sudo dnf install -y ffmpeg"], false)
        }
        PackageManager::Yum => {
            ("sh", vec!["-c", "sudo yum install -y ffmpeg"], false)
        }
        PackageManager::Chocolatey => {
            ("choco", vec!["install", "ffmpeg", "-y"], false)
        }
    };

    eprintln!("Installing FFmpeg...");
    let status = if matches!(package_manager, PackageManager::Apt | PackageManager::Yum | PackageManager::Dnf) {
        // For shell commands with sudo
        Command::new(command)
            .args(&args)
            .status()
            .context("Failed to execute installation command")?
    } else {
        // For direct commands
        Command::new(command)
            .args(&args)
            .status()
            .context("Failed to execute installation command")?
    };

    if !status.success() {
        bail!("FFmpeg installation failed. Please install manually.");
    }

    eprintln!("FFmpeg installed successfully!");
    Ok(())
}

pub fn ensure_ffmpeg_exists() -> Result<()> {
    let ok = Command::new("ffmpeg")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok();
    
    if ok {
        return Ok(());
    }

    // FFmpeg not found, try to install
    eprintln!("FFmpeg not found on PATH.");
    
    let package_manager = match detect_package_manager() {
        Some(pm) => pm,
        None => {
            eprintln!("\nNo supported package manager found. Please install FFmpeg manually:");
            eprintln!(" - macOS: brew install ffmpeg");
            eprintln!(" - Debian/Ubuntu: sudo apt-get install ffmpeg");
            eprintln!(" - Fedora: sudo dnf install ffmpeg");
            eprintln!(" - RHEL/CentOS: sudo yum install ffmpeg");
            eprintln!(" - Windows: choco install ffmpeg");
            bail!("FFmpeg not found and no package manager available for auto-installation.");
        }
    };

    let install_cmd = match package_manager {
        PackageManager::Homebrew => "brew install ffmpeg",
        PackageManager::Apt => "sudo apt-get update && sudo apt-get install -y ffmpeg",
        PackageManager::Dnf => "sudo dnf install -y ffmpeg",
        PackageManager::Yum => "sudo yum install -y ffmpeg",
        PackageManager::Chocolatey => "choco install ffmpeg -y",
    };

    if !prompt_user(&format!("Would you like to install FFmpeg using: {}?", install_cmd))? {
        eprintln!("Installation cancelled. Please install FFmpeg manually and try again.");
        bail!("FFmpeg not found. Installation cancelled.");
    }

    install_ffmpeg(package_manager)?;

    // Verify installation
    let ok = Command::new("ffmpeg")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok();
    
    if !ok {
        bail!("FFmpeg installation completed but verification failed. Please check your installation.");
    }

    Ok(())
}

pub fn ensure_ffprobe_exists() -> Result<()> {
    let ok = Command::new("ffprobe")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok();
    
    if ok {
        return Ok(());
    }

    // ffprobe not found, but it's usually included with ffmpeg
    // Check if ffmpeg exists first
    let ffmpeg_ok = Command::new("ffmpeg")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok();

    if !ffmpeg_ok {
        // If ffmpeg is also missing, try to install it (which should include ffprobe)
        ensure_ffmpeg_exists()?;
        
        // Re-check ffprobe after installing ffmpeg
        let ok = Command::new("ffprobe")
            .arg("-version")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok();
        
        if !ok {
            bail!("ffprobe not found. FFmpeg was installed but ffprobe is missing. This is unusual - please check your FFmpeg installation.");
        }
        return Ok(());
    }

    // ffmpeg exists but ffprobe doesn't - this is unusual
    eprintln!("ffprobe not found on PATH, but ffmpeg is installed.");
    eprintln!("ffprobe should be included with ffmpeg. Please check your FFmpeg installation.");
    bail!("ffprobe not found on PATH. Install ffmpeg (ffprobe included) and try again.");
}

pub fn get_command_version(command: &str) -> Option<String> {
    Command::new(command)
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| {
            String::from_utf8(output.stdout)
                .ok()
                .map(|s| s.lines().next().unwrap_or("").to_string())
        })
}

pub fn get_command_path(command: &str) -> Option<String> {
    if cfg!(windows) {
        Command::new("where")
            .arg(command)
            .output()
            .ok()
            .and_then(|output| {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.lines().next().unwrap_or("").trim().to_string())
            })
    } else {
        Command::new("which")
            .arg(command)
            .output()
            .ok()
            .and_then(|output| {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            })
    }
}

