/// Admin / root privilege detection and elevation.
///
/// On Windows this checks the process token; on Unix it checks EUID.
/// TUN mode and system-proxy changes typically require elevated privileges.

/// Returns `true` if the current process is running with elevated privileges.
pub fn is_admin() -> bool {
    #[cfg(windows)]
    {
        is_admin_windows()
    }
    #[cfg(unix)]
    {
        is_admin_unix()
    }
}

/// Describe what the user needs to do to gain admin privileges on this OS.
pub fn elevation_instructions() -> &'static str {
    #[cfg(windows)]
    {
        "Run Hopen as Administrator (right-click → Run as administrator)"
    }
    #[cfg(target_os = "macos")]
    {
        "Grant privileges via System Preferences, or run: sudo open /Applications/Hopen.app"
    }
    #[cfg(target_os = "linux")]
    {
        "Run Hopen with root privileges: sudo hopen"
    }
}

// ── Platform implementations ────────────────────────────────────

#[cfg(windows)]
fn is_admin_windows() -> bool {
    use std::mem;
    use std::ptr;

    // Windows type aliases — keep the module self-contained.
    type HANDLE = *mut std::ffi::c_void;
    type BOOL = i32;

    #[repr(C)]
    #[allow(non_snake_case)]
    struct TokenElevation {
        TokenIsElevated: u32,
    }

    const TOKEN_QUERY: u32 = 0x0008;
    const TOKEN_ELEVATION: u32 = 20;

    unsafe extern "system" {
        fn GetCurrentProcess() -> HANDLE;
        fn OpenProcessToken(h: HANDLE, access: u32, token: *mut HANDLE) -> BOOL;
        fn GetTokenInformation(
            token: HANDLE,
            info_class: u32,
            info: *mut std::ffi::c_void,
            info_len: u32,
            ret_len: *mut u32,
        ) -> BOOL;
        fn CloseHandle(h: HANDLE) -> BOOL;
    }

    unsafe {
        let mut token: HANDLE = ptr::null_mut();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token) == 0 {
            return false;
        }

        let mut elevation: TokenElevation = mem::zeroed();
        let mut size = mem::size_of::<TokenElevation>() as u32;

        let ok = GetTokenInformation(
            token,
            TOKEN_ELEVATION,
            &mut elevation as *mut _ as *mut _,
            size,
            &mut size,
        );

        CloseHandle(token);

        ok != 0 && elevation.TokenIsElevated != 0
    }
}

#[cfg(unix)]
fn is_admin_unix() -> bool {
    // On Unix, root always has UID 0.
    unsafe { libc::geteuid() == 0 }
}
