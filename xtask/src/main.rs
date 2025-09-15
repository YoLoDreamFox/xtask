use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use fs_extra::dir::CopyOptions;
use cargo_metadata::MetadataCommand;

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    if !args.contains(&"--release".to_string()) {
        args.push("--release".to_string());
    }

    let mut custom_target: Option<String> = None;
    let mut cleaned_args = Vec::new();
    let mut skip_next = false;

    for (i, arg) in args.iter().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }
        if arg == "--target" {
            if let Some(val) = args.get(i + 1) {
                custom_target = Some(val.clone());
                skip_next = true;
                continue;
            }
        }
        cleaned_args.push(arg.clone());
    }

    let mut targets: Vec<(&str, &str)> = if let Some(t) = custom_target {
        vec![match t.as_str() {
            "windows" => ("windows", "x86_64-pc-windows-gnu"),
            "linux"   => ("linux", "x86_64-unknown-linux-gnu"),
            "macos"   => ("macos", "x86_64-apple-darwin"),
            "android" => ("android", "aarch64-linux-android"),
            "ios"     => ("ios", "aarch64-apple-ios"),
            _ => {
                eprintln!("Unknown target: {}", t);
                std::process::exit(1);
            }
        }]
    } else {
        vec![
            ("windows", "x86_64-pc-windows-gnu"),
            ("linux", "x86_64-unknown-linux-gnu"),
            ("macos", "x86_64-apple-darwin"),
            ("android", "aarch64-linux-android"),
            ("ios", "aarch64-apple-ios"),
        ]
    };

    if !cfg!(target_os = "macos") {
        targets.retain(|(name, _)| *name != "macos");
        targets.retain(|(name, _)| *name != "ios");
    }

    let dist_dir = Path::new("target").join("dist");
    if !dist_dir.exists() {
        fs::create_dir_all(&dist_dir).expect("Failed to create target/dist folder");
    }

    fn get_pkg_name() -> String {
        let metadata = MetadataCommand::new()
            .exec()
            .expect("Failed to get metadata from Cargo.toml");

        metadata.root_package()
            .expect("Root package not found")
            .name
            .clone()
    }

    let app_pkg_name = get_pkg_name();

    let assets_dir = Path::new("assets");
    let mut errors: Vec<String> = Vec::new();

    for (name, target) in targets {
        println!("🔨 Building for {target}...");

        let status = Command::new("cargo")
            .arg("build")
            .arg("-p")
            .arg(&app_pkg_name)
            .args(&cleaned_args)
            .arg("--target")
            .arg(target)
            .arg("--features")
            .arg(name)
            .status()
            .expect("Failed to run cargo build");

        if !status.success() {
            eprintln!("❌ Build failed for {target}");
            errors.push(name.to_string());
            continue;
        }

        let ext = match name {
            "windows" => ".exe",
            //"android" => ".so",
            _ => "",
        };

        let built_path = Path::new("target")
            .join(target)
            .join("release")
            .join(format!("{}{}", app_pkg_name, ext));

        if !built_path.exists() {
            eprintln!("❌ Expected binary at {}, but it was not found.", built_path.display());
            errors.push(name.to_string());
            continue;
        }

        let platform_dist_dir = dist_dir.join(name);
        fs::create_dir_all(&platform_dist_dir).unwrap();

        let dist_path = platform_dist_dir.join(format!("{}{}", app_pkg_name, ext));

        if let Err(e) = fs::copy(&built_path, &dist_path) {
            eprintln!("❌ Failed to copy {} → {}: {e}",
                      built_path.display(), dist_path.display());
            errors.push(name.to_string());
            continue;
        }

        if assets_dir.exists() {
            let dist_assets_dir = platform_dist_dir.join("assets");
            let mut options = CopyOptions::new();
            options.copy_inside = true;
            options.overwrite = true;

            if let Err(e) = fs_extra::dir::copy(&assets_dir, &dist_assets_dir, &options) {
                eprintln!("❌ Failed to copy assets: {e}");
                errors.push(name.to_string());
                continue;
            }
        }

        println!("✅ Build for {name} is ready: {}", platform_dist_dir.display());
    }

    println!("\n========================");
    if errors.is_empty() {
        println!("🎉 All builds completed successfully!");
    } else {
        println!("⚠️ Build finished with errors.");
        println!("Failed to build: {}", errors.join(", "));
    }
}
