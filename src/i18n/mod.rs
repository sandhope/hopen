//! Localised UI strings and runtime language selection.
//!
//! This module owns language packs, system-locale matching, and the global
//! manager used by components throughout the app. Visual styling remains in `theme`.
//!
//! Design follows velotype's i18n architecture:
//! - `I18nStrings` is a flat struct of named fields — no dynamic key-value maps.
//! - `I18nManager` is a GPUI global, accessed via `cx.global::<I18nManager>()`.
//! - Built-in languages: `en-US` (fallback), `zh-CN`.
//! - System locale is auto-detected on startup via `sys-locale`.

use std::sync::Arc;

use gpui::{App, Global};

// ─── The translatable string set ─────────────────────────────────

/// All localisable UI strings for Hopen.
///
/// Some fields are not yet read directly but are defined for future use
/// (theme label display).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct I18nStrings {
    // Dashboard cards
    pub dashboard_network_speed: &'static str,
    pub dashboard_proxy_control: &'static str,
    pub dashboard_lan_ip: &'static str,
    pub dashboard_traffic_usage: &'static str,
    pub dashboard_network_detection: &'static str,

    // Dashboard controls
    pub dashboard_upload: &'static str,
    pub dashboard_download: &'static str,
    pub dashboard_system_proxy: &'static str,
    pub dashboard_tun_mode: &'static str,
    pub dashboard_outbound_mode: &'static str,
    pub dashboard_status_on: &'static str,
    pub dashboard_status_off: &'static str,

    // Core button
    pub core_start: &'static str,
    pub core_stop: &'static str,

    // Outbound modes
    pub outbound_global: &'static str,
    pub outbound_rule: &'static str,
    pub outbound_direct: &'static str,

    // Settings page
    pub settings_language: &'static str,
    pub settings_language_subtitle: &'static str,
    pub settings_theme: &'static str,
    pub settings_theme_subtitle: &'static str,
    pub settings_basic_config: &'static str,
    pub settings_basic_config_subtitle: &'static str,
    pub settings_advanced_config: &'static str,
    pub settings_advanced_config_subtitle: &'static str,
    pub settings_hotkeys: &'static str,
    pub settings_hotkeys_subtitle: &'static str,
    pub settings_backup_restore: &'static str,
    pub settings_backup_restore_subtitle: &'static str,
    pub settings_about: &'static str,
    pub settings_about_subtitle: &'static str,

    // Placeholder pages
    pub placeholder_proxies_title: &'static str,
    pub placeholder_proxies_desc: &'static str,
    pub placeholder_profiles_title: &'static str,
    pub placeholder_profiles_desc: &'static str,
    pub placeholder_requests_title: &'static str,
    pub placeholder_requests_desc: &'static str,
    pub placeholder_connections_title: &'static str,
    pub placeholder_connections_desc: &'static str,
    pub placeholder_resources_title: &'static str,
    pub placeholder_resources_desc: &'static str,
    pub placeholder_logs_title: &'static str,
    pub placeholder_logs_desc: &'static str,

    // Network
    pub network_detecting: &'static str,
    pub network_na: &'static str,
    pub network_unknown: &'static str,
    pub network_ip_label: &'static str,
    pub network_isp_label: &'static str,

    // Page titles (header)
    pub page_title_dashboard: &'static str,
    pub page_title_proxies: &'static str,
    pub page_title_profiles: &'static str,
    pub page_title_requests: &'static str,
    pub page_title_connections: &'static str,
    pub page_title_resources: &'static str,
    pub page_title_logs: &'static str,
    pub page_title_tools: &'static str,

    // App
    pub app_name: &'static str,

    // Theme
    pub theme_dark: &'static str,
    pub theme_light: &'static str,

    // Sub-page navigation
    pub nav_back: &'static str,
    pub page_title_language: &'static str,
}

impl I18nStrings {
    /// English (United States) — fallback strings.
    pub fn en_us() -> Self {
        Self {
            dashboard_network_speed: "Network Speed",
            dashboard_proxy_control: "Proxy Control",
            dashboard_lan_ip: "LAN IP",
            dashboard_traffic_usage: "Traffic Usage",
            dashboard_network_detection: "Network Detection",

            dashboard_upload: "Upload",
            dashboard_download: "Download",
            dashboard_system_proxy: "System Proxy",
            dashboard_tun_mode: "TUN Mode",
            dashboard_outbound_mode: "Outbound Mode",
            dashboard_status_on: "ON",
            dashboard_status_off: "OFF",

            core_start: "Start Core",
            core_stop: "Stop Core",

            outbound_global: "Global",
            outbound_rule: "Rule",
            outbound_direct: "Direct",

            settings_language: "Language",
            settings_language_subtitle: "Switch display language",
            settings_theme: "Theme",
            settings_theme_subtitle: "Dark / Light — tap to switch appearance",
            settings_basic_config: "Basic Config",
            settings_basic_config_subtitle: "Port, log level, mode",
            settings_advanced_config: "Advanced Config",
            settings_advanced_config_subtitle: "DNS, TUN, rules",
            settings_hotkeys: "Hotkeys",
            settings_hotkeys_subtitle: "Keyboard shortcuts",
            settings_backup_restore: "Backup & Restore",
            settings_backup_restore_subtitle: "WebDAV sync",
            settings_about: "About",
            settings_about_subtitle: "Version and license info",

            placeholder_proxies_title: "Proxy Groups",
            placeholder_proxies_desc: "Proxy groups will appear here when the core is connected.",
            placeholder_profiles_title: "Profiles",
            placeholder_profiles_desc: "Import or add subscription profiles to get started.",
            placeholder_requests_title: "Request Timeline",
            placeholder_requests_desc: "Real-time request tracking will appear here.",
            placeholder_connections_title: "Active Connections",
            placeholder_connections_desc: "Active connections will be listed here.",
            placeholder_resources_title: "Resources",
            placeholder_resources_desc: "GeoIP, GeoSite, and other resource files will be managed here.",
            placeholder_logs_title: "Core Logs",
            placeholder_logs_desc: "Logs from the proxy core will stream here.",

            network_detecting: "Detecting...",
            network_na: "N/A",
            network_unknown: "Unknown",
            network_ip_label: "IP",
            network_isp_label: "ISP",

            page_title_dashboard: "Dashboard",
            page_title_proxies: "Proxies",
            page_title_profiles: "Profiles",
            page_title_requests: "Requests",
            page_title_connections: "Connections",
            page_title_resources: "Resources",
            page_title_logs: "Logs",
            page_title_tools: "Settings",

            app_name: "Hopen",

            theme_dark: "Dark",
            theme_light: "Light",

            nav_back: "Back",
            page_title_language: "Language",
        }
    }

    /// 简体中文 (Simplified Chinese).
    pub fn zh_cn() -> Self {
        Self {
            dashboard_network_speed: "网络速度",
            dashboard_proxy_control: "代理控制",
            dashboard_lan_ip: "局域网 IP",
            dashboard_traffic_usage: "流量统计",
            dashboard_network_detection: "网络检测",

            dashboard_upload: "上传",
            dashboard_download: "下载",
            dashboard_system_proxy: "系统代理",
            dashboard_tun_mode: "TUN 模式",
            dashboard_outbound_mode: "出站模式",
            dashboard_status_on: "开",
            dashboard_status_off: "关",

            core_start: "启动核心",
            core_stop: "停止核心",

            outbound_global: "全局",
            outbound_rule: "规则",
            outbound_direct: "直连",

            settings_language: "语言",
            settings_language_subtitle: "切换显示语言",
            settings_theme: "主题",
            settings_theme_subtitle: "深色 / 浅色 — 点击切换外观",
            settings_basic_config: "基础配置",
            settings_basic_config_subtitle: "端口、日志级别、模式",
            settings_advanced_config: "高级配置",
            settings_advanced_config_subtitle: "DNS、TUN、规则",
            settings_hotkeys: "快捷键",
            settings_hotkeys_subtitle: "键盘快捷键设置",
            settings_backup_restore: "备份与恢复",
            settings_backup_restore_subtitle: "WebDAV 同步",
            settings_about: "关于",
            settings_about_subtitle: "版本与许可证信息",

            placeholder_proxies_title: "代理组",
            placeholder_proxies_desc: "核心连接后，代理组将显示在此处。",
            placeholder_profiles_title: "订阅配置",
            placeholder_profiles_desc: "导入或添加订阅配置以开始使用。",
            placeholder_requests_title: "请求时间线",
            placeholder_requests_desc: "实时请求追踪将在此处显示。",
            placeholder_connections_title: "活动连接",
            placeholder_connections_desc: "活动连接将在此处列出。",
            placeholder_resources_title: "资源管理",
            placeholder_resources_desc: "GeoIP、GeoSite 等资源文件将在此处管理。",
            placeholder_logs_title: "核心日志",
            placeholder_logs_desc: "代理核心的日志将在此处流式显示。",

            network_detecting: "检测中...",
            network_na: "无",
            network_unknown: "未知",
            network_ip_label: "IP",
            network_isp_label: "运营商",

            page_title_dashboard: "仪表盘",
            page_title_proxies: "代理",
            page_title_profiles: "订阅",
            page_title_requests: "请求",
            page_title_connections: "连接",
            page_title_resources: "资源",
            page_title_logs: "日志",
            page_title_tools: "设置",

            app_name: "Hopen",

            theme_dark: "深色",
            theme_light: "浅色",

            nav_back: "返回",
            page_title_language: "语言",
        }
    }

    /// Resolve the title for a given page.
    pub fn page_title(&self, page: crate::navigation::Page) -> &'static str {
        use crate::navigation::Page;
        match page {
            Page::Dashboard => self.page_title_dashboard,
            Page::Proxies => self.page_title_proxies,
            Page::Profiles => self.page_title_profiles,
            Page::Requests => self.page_title_requests,
            Page::Connections => self.page_title_connections,
            Page::Resources => self.page_title_resources,
            Page::Logs => self.page_title_logs,
            Page::Tools => self.page_title_tools,
        }
    }

    /// Resolve the label for an outbound mode.
    pub fn outbound_mode_label(&self, mode: crate::OutboundMode) -> &'static str {
        use crate::OutboundMode;
        match mode {
            OutboundMode::Global => self.outbound_global,
            OutboundMode::Rule => self.outbound_rule,
            OutboundMode::Direct => self.outbound_direct,
        }
    }
}

// ─── I18n Manager (GPUI Global) ──────────────────────────────────

/// Runtime language manager, stored as a GPUI global.
///
/// `current_language_id` will be used by runtime language switching.
pub struct I18nManager {
    /// Currently active language id, e.g. "en-US" or "zh-CN".
    pub current_language_id: String,
    /// All translated strings, wrapped in Arc for cheap sharing.
    strings: Arc<I18nStrings>,
}

impl Global for I18nManager {}

impl I18nManager {
    /// Initialize the i18n system with the given BCP 47 language id.
    ///
    /// Falls back to `en-US` if the id is not recognized.
    pub fn init_with_language_id(cx: &mut App, language_id: &str) {
        // Gracefully remove any trailing newline / whitespace from the user-config file.
        let id = language_id.trim();
        let strings = I18nStrings::from_language_id(id);
        cx.set_global(Self {
            current_language_id: id.to_owned(),
            strings: Arc::new(strings),
        });
    }

    /// Read current translated strings by reference.
    #[allow(dead_code)]
    pub fn strings(&self) -> &I18nStrings {
        &self.strings
    }

    /// Clone the `Arc` of translated strings — one atomic ref-count bump.
    ///
    /// Prefer this over `.strings().clone()` in hot rendering paths.
    pub fn strings_arc(&self) -> Arc<I18nStrings> {
        Arc::clone(&self.strings)
    }
}

// ─── Helpers ─────────────────────────────────────────────────────

impl I18nStrings {
    /// Build the string set for a known language id; falls back to `en-US`.
    fn from_language_id(id: &str) -> Self {
        match id {
            "zh-CN" | "zh-Hans" | "zh" => Self::zh_cn(),
            _ => Self::en_us(),
        }
    }
}

/// Try to pick a BCP 47 language id from the system locale.
///
/// Uses `sys-locale` to query the OS preference, then maps the locale
/// to one of the supported language ids.
pub fn detect_system_language_id() -> String {
    let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-US"));
    system_locale_to_language_id(&locale)
}

/// Map an OS locale string (e.g. "zh-CN", "zh-Hans-CN", "en-US") to a
/// supported language id. Unrecognised locales fall back to `en-US`.
fn system_locale_to_language_id(locale: &str) -> String {
    let normalized = locale.to_lowercase();

    // Exact match first.
    if normalized.starts_with("zh-cn")
        || normalized.starts_with("zh-hans")
        || normalized.starts_with("zh-sg")
        || normalized.starts_with("zh-hans-cn")
    {
        return String::from("zh-CN");
    }

    // Generic Chinese (zh, zh-TW -> fall back to zh-CN for now)
    if normalized.starts_with("zh") {
        return String::from("zh-CN");
    }

    // All others → English.
    String::from("en-US")
}

// ─── Language Display Name ──────────────────────────────────────

/// Returns the display name of a language in its own script.
/// e.g. "zh-CN" → "简体中文", "en-US" → "English".
pub fn language_display_name(language_id: &str) -> &'static str {
    match language_id {
        "zh-CN" | "zh-Hans" | "zh" => "简体中文",
        _ => "English",
    }
}

// ─── Convenience accessor ────────────────────────────────────────

/// Convenience: retrieve the current `I18nStrings` from GPUI global state.
///
/// Usable from any `Context<T>` or `&App`.
#[allow(dead_code)]
pub fn strings(cx: &App) -> &I18nStrings {
    cx.global::<I18nManager>().strings()
}
