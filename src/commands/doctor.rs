use crate::util::system::{detect_package_manager, get_command_path, get_command_version, PackageManager};
use anyhow::Result;
use std::process::{Command, Stdio};

pub fn handle_doctor() -> Result<()> {
    println!("FFHuman System Diagnostics\n");
    println!("{}", "=".repeat(50));

    // System Information
    println!("\nSystem Information:");
    println!("  OS: {}", std::env::consts::OS);
    println!("  Architecture: {}", std::env::consts::ARCH);
    println!("  Family: {}", std::env::consts::FAMILY);

    // Package Manager Detection
    println!("\nPackage Managers:");
    let package_manager = detect_package_manager();
    match &package_manager {
        Some(pm) => {
            let pm_name = match pm {
                PackageManager::Homebrew => "Homebrew (macOS)",
                PackageManager::Apt => "apt (Debian/Ubuntu)",
                PackageManager::Dnf => "dnf (Fedora)",
                PackageManager::Yum => "yum (RHEL/CentOS)",
                PackageManager::Chocolatey => "Chocolatey (Windows)",
            };
            println!("  {} detected", pm_name);
            
            // Show version if available
            let version_cmd = match pm {
                PackageManager::Homebrew => "brew",
                PackageManager::Apt => "apt",
                PackageManager::Dnf => "dnf",
                PackageManager::Yum => "yum",
                PackageManager::Chocolatey => "choco",
            };
            if let Some(version) = get_command_version(version_cmd) {
                println!("     Version: {}", version);
            }
        }
        None => {
            println!("  No supported package manager found");
            println!("     Available options:");
            println!("       - macOS: Install Homebrew (https://brew.sh)");
            println!("       - Debian/Ubuntu: apt (usually pre-installed)");
            println!("       - Fedora: dnf (usually pre-installed)");
            println!("       - RHEL/CentOS: yum (usually pre-installed)");
            println!("       - Windows: Install Chocolatey (https://chocolatey.org)");
        }
    }

    // FFmpeg Status
    println!("\n FFmpeg Status:");
    let ffmpeg_ok = Command::new("ffmpeg")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok();
    
    if ffmpeg_ok {
        println!("  FFmpeg is installed");
        if let Some(version) = get_command_version("ffmpeg") {
            println!("     Version: {}", version);
        }
        if let Some(path) = get_command_path("ffmpeg") {
            println!("     Path: {}", path);
        }
    } else {
        println!("  FFmpeg is NOT installed");
        if let Some(ref pm) = package_manager {
            let install_cmd = match pm {
                PackageManager::Homebrew => "brew install ffmpeg",
                PackageManager::Apt => "sudo apt-get update && sudo apt-get install -y ffmpeg",
                PackageManager::Dnf => "sudo dnf install -y ffmpeg",
                PackageManager::Yum => "sudo yum install -y ffmpeg",
                PackageManager::Chocolatey => "choco install ffmpeg -y",
            };
            println!("     Install command: {}", install_cmd);
        } else {
            println!("     Manual installation required:");
            println!("       - macOS: brew install ffmpeg");
            println!("       - Debian/Ubuntu: sudo apt-get install ffmpeg");
            println!("       - Fedora: sudo dnf install ffmpeg");
            println!("       - RHEL/CentOS: sudo yum install ffmpeg");
            println!("       - Windows: choco install ffmpeg");
        }
    }

    // ffprobe Status
    println!("\n ffprobe Status:");
    let ffprobe_ok = Command::new("ffprobe")
        .arg("-version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok();
    
    if ffprobe_ok {
        println!("  ffprobe is installed");
        if let Some(version) = get_command_version("ffprobe") {
            println!("     Version: {}", version);
        }
        if let Some(path) = get_command_path("ffprobe") {
            println!("     Path: {}", path);
        }
    } else {
        println!("  ffprobe is NOT installed");
        if ffmpeg_ok {
            println!("     Warning: ffmpeg is installed but ffprobe is missing");
            println!("     This is unusual - ffprobe should be included with ffmpeg");
            println!("     Please check your FFmpeg installation");
        } else {
            println!("     ffprobe will be installed with ffmpeg");
        }
    }

    // Summary
    println!("\n{}", "=".repeat(50));
    println!("\nSummary:");
    let mut all_ok = true;
    
    if package_manager.is_none() {
        println!("  No package manager detected (manual installation required)");
        all_ok = false;
    }
    
    if !ffmpeg_ok {
        println!("  FFmpeg is missing");
        all_ok = false;
    } else {
        println!("  FFmpeg is installed");
    }
    
    if !ffprobe_ok {
        println!("  ffprobe is missing");
        all_ok = false;
    } else {
        println!("  ffprobe is installed");
    }

    if all_ok {
        println!("\nSystem is ready to use!");
    } else {
        println!("\nSome issues detected. Please install missing components.");
    }

    Ok(())
}

