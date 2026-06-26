//! Build script: handle app icon embedding + Go core compilation.
//!
//! Following Zed's approach:
//! - Windows: convert PNG → ICO via `image` crate, embed with `winresource`.
//! - Linux:     resize PNG → 256×256, save to OUT_DIR for `include_bytes!`.
//! - macOS:     set deployment target (icon handled by app bundle /.icns).
//!
//! Added: Go core (FlClashCore) cross-compilation and embedding.
//! Uses the public metacubex/mihomo module via go.mod — no local source needed.

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Path to the source logo (PNG format).
const LOGO_SRC: &str = "resources/app-icon.png";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Declare custom cfg flag used in src/core/manager.rs
    println!("cargo::rustc-check-cfg=cfg(core_bin_disabled)");

    // ── Go core compilation (all platforms) ──
    build_go_core(&out_dir);

    // ── Windows: generate .ico and embed via resource compiler ──
    #[cfg(windows)]
    {
        let ico_path = generate_ico(LOGO_SRC, &out_dir);

        // Embed the icon using winresource (Windows RC compiler)
        winresource::WindowsResource::new()
            .set_icon(&ico_path.to_string_lossy())
            .compile()
            .expect("failed to compile Windows resources");

        // Re-run if the logo changes
        println!("cargo:rerun-if-changed={}", LOGO_SRC);
    }

    // ── Linux / FreeBSD: resize PNG to 256×256, save to OUT_DIR ──
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    {
        prepare_app_icon_linux(LOGO_SRC, &out_dir);
        println!("cargo:rerun-if-changed={}", LOGO_SRC);
    }

    // ── macOS: set minimum deployment target ──
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.15");
    }
}

// ═══════════════════════════════════════════════════════════════════
// Go core compilation
// ═══════════════════════════════════════════════════════════════════

/// Build the Go IPC core binary and set up embedding.
///
/// Compiles `core-go/` → `$OUT_DIR/FlClashCore(.exe)`.
/// Sets env var `HOPEN_CORE_PATH` for the Rust code to reference.
/// The binary is embedded via `include_bytes!` in release builds.
fn build_go_core(out_dir: &str) {
    let core_dir = Path::new("core-go");
    if !core_dir.join("go.mod").exists() {
        println!("cargo:warning=[hopen] core-go/go.mod not found, skipping Go build");
        println!("cargo:rustc-cfg=core_bin_disabled");
        return;
    }

    let target = env::var("TARGET").unwrap_or_default();
    let (goos, goarch) = map_target_to_go(&target);

    let exe_name = if goos == "windows" {
        "FlClashCore.exe"
    } else {
        "FlClashCore"
    };

    let out_path = PathBuf::from(out_dir).join(exe_name);

    // ── Check Go availability ──
    if !is_go_available() {
        println!("cargo:warning=[hopen] Go toolchain not found — skip core compilation");
        println!("cargo:rustc-cfg=core_bin_disabled");
        return;
    }

    // ── go mod tidy (generates go.sum, downloads deps from Go proxy) ──
    run_go_mod_tidy(core_dir);

    // ── go build ──
    println!(
        "cargo:warning=[hopen] Building FlClashCore for {goos}/{goarch}..."
    );

    let build_status = Command::new("go")
        .args(["build", "-trimpath", "-ldflags=-s -w", "-mod=mod", "-o"])
        .arg(&out_path)
        .arg(".")
        .current_dir(core_dir)
        .env("CGO_ENABLED", "0")
        .env("GOOS", goos)
        .env("GOARCH", goarch)
        .status();

    match build_status {
        Ok(s) if s.success() => {
            println!("cargo:rustc-env=HOPEN_CORE_PATH={}", out_path.display());
            println!(
                "cargo:warning=[hopen] Go core built: {}",
                out_path.display()
            );
        }
        Ok(s) => {
            panic!("Go build failed with exit code: {}", s);
        }
        Err(e) => {
            panic!("Failed to run Go build: {}", e);
        }
    }

    // Re-run if any Go source changes
    println!("cargo:rerun-if-changed=core-go/");
}

/// Check if the Go toolchain is available.
fn is_go_available() -> bool {
    Command::new("go")
        .arg("version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Run `go mod tidy` in the core-go directory.
fn run_go_mod_tidy(core_dir: &Path) {
    let status = Command::new("go")
        .args(["mod", "tidy"])
        .current_dir(core_dir)
        .status();

    match status {
        Ok(s) if s.success() => {}
        Ok(s) => {
            println!("cargo:warning=[hopen] go mod tidy exited with {s}, continuing anyway");
        }
        Err(e) => {
            println!("cargo:warning=[hopen] go mod tidy failed: {e}");
        }
    }
}

/// Map Rust target triple → (GOOS, GOARCH).
fn map_target_to_go(target: &str) -> (&str, &str) {
    let arch = if target.starts_with("x86_64") {
        "amd64"
    } else if target.starts_with("aarch64") || target.starts_with("arm64") {
        "arm64"
    } else if target.starts_with("i686") || target.starts_with("i586") {
        "386"
    } else if target.starts_with("armv7") {
        "arm"
    } else if target.starts_with("riscv64") {
        "riscv64"
    } else {
        "amd64"
    };

    let os = if target.contains("windows") {
        "windows"
    } else if target.contains("apple") || target.contains("darwin") {
        "darwin"
    } else if target.contains("linux") {
        "linux"
    } else if target.contains("freebsd") {
        "freebsd"
    } else {
        "linux"
    };

    (os, arch)
}

// ═══════════════════════════════════════════════════════════════════
// Icon helpers
// ═══════════════════════════════════════════════════════════════════

/// Convert the source PNG into an `.ico` file.
#[cfg(windows)]
fn generate_ico(png_path: &str, out_dir: &str) -> std::path::PathBuf {
    use image::imageops::FilterType;
    use image::ImageEncoder;

    let img = image::open(png_path)
        .unwrap_or_else(|e| panic!("failed to open logo at '{}': {}", png_path, e));

    let resized = img.resize_exact(256, 256, FilterType::Lanczos3);
    let rgba = resized.to_rgba8();

    let ico_path = Path::new(out_dir).join("app_icon.ico");
    let ico_file = std::fs::File::create(&ico_path).expect("failed to create .ico file");

    let encoder = image::codecs::ico::IcoEncoder::new(ico_file);
    encoder
        .write_image(rgba.as_raw(), 256, 256, image::ExtendedColorType::Rgba8)
        .expect("failed to encode .ico file");

    ico_path
}

/// Resize the source PNG to 256×256 and save it to OUT_DIR.
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn prepare_app_icon_linux(png_path: &str, out_dir: &str) {
    use image::imageops::FilterType;

    let img = image::open(png_path).expect("failed to open logo PNG");
    let resized = img.resize(256, 256, FilterType::Lanczos3);
    let icon_out = Path::new(out_dir).join("app_icon.png");
    resized.save(&icon_out).expect("failed to save app icon");
}
