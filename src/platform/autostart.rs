/// Auto-start / launch-on-boot management.
///
/// Registers (or unregisters) Hopen to start automatically when the user logs in.

use std::path::PathBuf;

/// Returns `true` if Hopen is currently set to auto-start on login.
#[allow(dead_code)]
pub fn is_auto_start_enabled() -> bool {
    autostart_path()
        .map(|p| p.exists())
        .unwrap_or(false)
}

/// Enable auto-start on login. Creates the platform-specific launcher entry.
pub fn enable_auto_start() -> Result<(), String> {
    let path = autostart_path().map_err(|e| format!("autostart path: {e}"))?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("create autostart dir: {e}"))?;
    }

    create_launcher(&path)?;
    Ok(())
}

/// Disable auto-start on login. Removes the platform-specific launcher entry.
pub fn disable_auto_start() -> Result<(), String> {
    let path = autostart_path().map_err(|e| format!("autostart path: {e}"))?;
    if path.exists() {
        std::fs::remove_file(&path).map_err(|e| format!("remove autostart: {e}"))?;
    }
    Ok(())
}

// ── Platform-specific paths & launcher creation ─────────────────

/// Path to the auto-start entry for this platform.
fn autostart_path() -> Result<PathBuf, String> {
    let path = {
        #[cfg(windows)]
        {
            dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("Microsoft")
                .join("Windows")
                .join("Start Menu")
                .join("Programs")
                .join("Startup")
                .join("Hopen.lnk")
        }
        #[cfg(target_os = "macos")]
        {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("Library")
                .join("LaunchAgents")
                .join("com.hopen.app.plist")
        }
        #[cfg(target_os = "linux")]
        {
            dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("autostart")
                .join("hopen.desktop")
        }
        #[cfg(not(any(windows, target_os = "macos", target_os = "linux")))]
        {
            compile_error!("unsupported platform for auto-start");
            PathBuf::new() // unreachable
        }
    };

    Ok(path)
}

/// Create the platform-specific launcher entry.
fn create_launcher(path: &std::path::Path) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| format!("current_exe: {e}"))?;

    #[cfg(windows)]
    {
        // Create a PowerShell script to generate a .lnk shortcut
        let ps_script = format!(
            r#"
$ws = New-Object -ComObject WScript.Shell
$sc = $ws.CreateShortcut('{}')
$sc.TargetPath = '{}'
$sc.Save()
"#,
            path.display(),
            exe.display()
        );

        let status = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", &ps_script])
            .status()
            .map_err(|e| format!("powershell: {e}"))?;

        if !status.success() {
            return Err("Failed to create startup shortcut".into());
        }
    }

    #[cfg(target_os = "macos")]
    {
        let plist = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.hopen.app</string>
    <key>Program</key>
    <string>{}</string>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <false/>
</dict>
</plist>"#,
            exe.display()
        );

        std::fs::write(path, plist).map_err(|e| format!("write plist: {e}"))?;
    }

    #[cfg(target_os = "linux")]
    {
        let desktop = format!(
            r#"[Desktop Entry]
Type=Application
Name=Hopen
Exec={}
X-GNOME-Autostart-enabled=true
NoDisplay=false
Hidden=false
Comment=Hopen Proxy Client
"#,
            exe.display()
        );

        std::fs::write(path, desktop).map_err(|e| format!("write desktop: {e}"))?;
    }

    Ok(())
}
