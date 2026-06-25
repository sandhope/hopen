/// System-wide proxy configuration.
///
/// Each platform has a different mechanism for setting the system proxy.
/// This module provides a unified `set_system_proxy(enable, host, port)` API.
use std::process::Command;

/// Enable or disable the system HTTP/HTTPS proxy pointing at `host:port`.
///
/// Returns `Ok(())` on success, or an error message describing what went wrong.
pub fn set_system_proxy(enable: bool, host: &str, port: u16) -> Result<(), String> {
    if enable {
        set_proxy(host, port)
    } else {
        unset_proxy()
    }
}

// ── Windows ─────────────────────────────────────────────────────

#[cfg(windows)]
fn set_proxy(host: &str, port: u16) -> Result<(), String> {
    let proxy = format!("{host}:{port}");
    let status = Command::new("netsh")
        .args(["winhttp", "set", "proxy", &proxy])
        .status()
        .map_err(|e| format!("Failed to run netsh: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err("netsh returned non-zero exit code. Try running Hopen as Administrator.".into())
    }
}

#[cfg(windows)]
fn unset_proxy() -> Result<(), String> {
    let status = Command::new("netsh")
        .args(["winhttp", "reset", "proxy"])
        .status()
        .map_err(|e| format!("Failed to run netsh: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err("Failed to reset system proxy.".into())
    }
}

// ── macOS ───────────────────────────────────────────────────────

#[cfg(target_os = "macos")]
fn set_proxy(host: &str, port: u16) -> Result<(), String> {
    // Get the active network service name
    let service = current_network_service()?;

    let status = Command::new("networksetup")
        .args(["-setwebproxy", &service, host, &port.to_string()])
        .status()
        .map_err(|e| format!("Failed to run networksetup: {e}"))?;

    if !status.success() {
        return Err("Failed to set web proxy".into());
    }

    let status = Command::new("networksetup")
        .args(["-setsecurewebproxy", &service, host, &port.to_string()])
        .status()
        .map_err(|e| format!("Failed to run networksetup: {e}"))?;

    if !status.success() {
        return Err("Failed to set secure web proxy".into());
    }

    Ok(())
}

#[cfg(target_os = "macos")]
fn unset_proxy() -> Result<(), String> {
    let service = current_network_service()?;

    let _ = Command::new("networksetup")
        .args(["-setwebproxystate", &service, "off"])
        .status();

    let _ = Command::new("networksetup")
        .args(["-setsecurewebproxystate", &service, "off"])
        .status();

    Ok(())
}

#[cfg(target_os = "macos")]
fn current_network_service() -> Result<String, String> {
    let output = Command::new("networksetup")
        .args(["-listallnetworkservices"])
        .output()
        .map_err(|e| format!("Failed to list services: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Pick the first non-header line that isn't "An asterisk..."
    for line in stdout.lines().skip(1) {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('*') {
            return Ok(trimmed.to_owned());
        }
    }
    Err("No network service found.".into())
}

// ── Linux ───────────────────────────────────────────────────────

#[cfg(target_os = "linux")]
fn set_proxy(host: &str, port: u16) -> Result<(), String> {
    let scheme = format!("http://{host}:{port}");

    // Try gsettings (GNOME)
    let gsettings_result = Command::new("gsettings")
        .args(["set", "org.gnome.system.proxy", "mode", "manual"])
        .status()
        .and_then(|_| {
            Command::new("gsettings")
                .args(["set", "org.gnome.system.proxy.http", "host", host])
                .status()
        })
        .and_then(|_| {
            Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.system.proxy.http",
                    "port",
                    &port.to_string(),
                ])
                .status()
        })
        .and_then(|_| {
            Command::new("gsettings")
                .args(["set", "org.gnome.system.proxy.https", "host", host])
                .status()
        })
        .and_then(|_| {
            Command::new("gsettings")
                .args([
                    "set",
                    "org.gnome.system.proxy.https",
                    "port",
                    &port.to_string(),
                ])
                .status()
        });

    if gsettings_result.is_ok() {
        return Ok(());
    }

    // Fallback: set environment variable (will only affect child processes)
    log::warn!("gsettings not available; proxy set via env vars only");
    std::env::set_var("http_proxy", &scheme);
    std::env::set_var("https_proxy", &scheme);
    Ok(())
}

#[cfg(target_os = "linux")]
fn unset_proxy() -> Result<(), String> {
    let _ = Command::new("gsettings")
        .args(["set", "org.gnome.system.proxy", "mode", "none"])
        .status();

    std::env::remove_var("http_proxy");
    std::env::remove_var("https_proxy");
    Ok(())
}
