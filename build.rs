//! Build script: handle app icon embedding per platform.
//!
//! Following Zed's approach:
//! - Windows: convert PNG → ICO via `image` crate, embed with `winresource`.
//! - Linux:     resize PNG → 256×256, save to OUT_DIR for `include_bytes!`.
//! - macOS:     set deployment target (icon handled by app bundle /.icns).

use std::env;
use std::path::Path;

/// Path to the source logo (PNG format).
const LOGO_SRC: &str = "resources/app-icon.png";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

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
        // macOS icon is handled via .icns in the app bundle, not here.
    }
}

// ════════════════════════════════════════════════════════════════

/// Convert the source PNG into an `.ico` file.
///
/// Uses the `image` crate's built-in ICO encoder (single frame, 256×256).
/// This is sufficient for modern Windows; multi-resolution ICOs offer
/// diminishing returns since Windows 10 scales from a 256px source.
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
///
/// At runtime, `include_bytes!(concat!(env!("OUT_DIR"), "/app_icon.png"))`
/// can be used to embed the icon directly into the binary.
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn prepare_app_icon_linux(png_path: &str, out_dir: &str) {
    use image::imageops::FilterType;

    let img = image::open(png_path).expect("failed to open logo PNG");
    let resized = img.resize(256, 256, FilterType::Lanczos3);
    let icon_out = Path::new(out_dir).join("app_icon.png");
    resized.save(&icon_out).expect("failed to save app icon");
}
