/// Settings / Tools page view and sub-page views.
///
/// Provides 13 navigable settings rows, each leading to a sub-page:
/// Language, Theme, Basic Config, Network Config, DNS Config, Rules Config,
/// Scripts Config, Advanced Config, On-Demand, Hotkeys, Backup & Restore,
/// Disclaimer, About.
///
/// Interactive widgets use entity-capture closures (Entity is Copy).

use gpui::*;
use gpui::prelude::*;

use crate::components::dialog::DialogParams;
use crate::components::toast::ToastData;
use crate::i18n::{I18nStrings, language_display_name};
use crate::navigation::ToolsSubPage;
use crate::theme::{AccentColor, Theme, ThemeMode};
use crate::{
    save_theme_mode, save_language_id, save_accent_color, AppState,
};

// ─── Settings State ──────────────────────────────────────

#[derive(Clone)]
pub struct SettingsData {
    pub mixed_port: String,
    pub http_port: String,
    pub socks_port: String,
    pub redir_port: String,
    pub tproxy_port: String,
    pub log_level: usize,
    pub run_mode: usize,

    pub ipv6_enabled: bool,
    pub interface: String,
    pub routing_mark: String,
    pub tcp_concurrency: String,
    pub udp_fallback: usize,

    pub dns_enabled: bool,
    pub dns_ipv6: bool,
    pub enhanced_mode: usize,
    pub fake_ip_range: String,
    pub fallback_dns: String,
    pub default_dns: String,

    pub tun_enabled: bool,
    pub tun_stack: usize,
    pub tun_device: String,
    pub tun_auto_route: bool,
    pub tun_auto_detect: bool,
    pub sniffer_enabled: bool,
    pub sniffer_override_dest: bool,
    pub sniffer_ports: String,
    pub sniffer_force_dns: bool,

    pub on_demand_enabled: bool,
    pub on_demand_trigger: usize,
    pub on_demand_wifi_list: String,
    pub on_demand_fallback: usize,

    pub hotkey_app: String,
    pub hotkey_dashboard: String,
    pub hotkey_proxies: String,
    pub hotkey_settings: String,
    pub hotkey_core: String,
    pub hotkey_search: String,

    pub backup_webdav_url: String,
    pub backup_auto_interval: usize,
    pub backup_export_format: usize,
    pub backup_last_time: String,
}

impl Default for SettingsData {
    fn default() -> Self {
        Self {
            mixed_port: "7890".into(), http_port: "7891".into(),
            socks_port: "7892".into(), redir_port: "7893".into(),
            tproxy_port: "7894".into(), log_level: 0, run_mode: 0,

            ipv6_enabled: false, interface: "0.0.0.0".into(),
            routing_mark: "0".into(), tcp_concurrency: "50".into(),
            udp_fallback: 0,

            dns_enabled: true, dns_ipv6: false, enhanced_mode: 0,
            fake_ip_range: "198.18.0.1/16".into(),
            fallback_dns: "tls://8.8.8.8".into(),
            default_dns: "223.5.5.5".into(),

            tun_enabled: true, tun_stack: 0, tun_device: "utun".into(),
            tun_auto_route: true, tun_auto_detect: true,
            sniffer_enabled: true, sniffer_override_dest: true,
            sniffer_ports: "80,443,8000-9000".into(),
            sniffer_force_dns: true,

            on_demand_enabled: false, on_demand_trigger: 0,
            on_demand_wifi_list: "Home-WiFi, Office".into(),
            on_demand_fallback: 0,

            hotkey_app: "Ctrl+T".into(), hotkey_dashboard: "Ctrl+1".into(),
            hotkey_proxies: "Ctrl+2".into(), hotkey_settings: "Ctrl+,".into(),
            hotkey_core: "Ctrl+Space".into(), hotkey_search: "Ctrl+F".into(),

            backup_webdav_url: "https://dav.example.com/hopen".into(),
            backup_auto_interval: 0, backup_export_format: 0,
            backup_last_time: "2025-06-24 08:00".into(),
        }
    }
}

const LOG_LEVELS: &[&str] = &["Info", "Debug", "Warning", "Error"];
const RUN_MODES: &[&str] = &["Rule", "Global", "Direct"];
const UDP_FALLBACKS: &[&str] = &["Direct", "Proxy", "Reject"];
const ENHANCED_MODES: &[&str] = &["Fake-IP", "Redir-Host"];
const TUN_STACKS: &[&str] = &["System", "gVisor", "Mixed"];
const ON_DEMAND_TRIGGERS: &[&str] = &["WiFi SSID", "Network Change", "Manual"];
const ON_DEMAND_FALLBACKS: &[&str] = &["Direct", "Proxy", "Block"];
const BACKUP_INTERVALS: &[&str] = &["Disabled", "1 hour", "6 hours", "12 hours", "24 hours", "7 days"];
const BACKUP_FORMATS: &[&str] = &["ZIP", "TAR", "RAW"];

// ─── Tools / Settings ─────────────────────────────────────────

pub(super) fn tools_view(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    div()
        .flex().flex_col().gap(px(4.0)).px(px(24.0))
        .child(language_entry_item(theme, cx, strings))
        .child(theme_entry_item(theme, cx, strings))
        .child(settings_nav_entry(theme, cx, strings, ToolsSubPage::BasicConfig, "svg/basic-config.svg", strings.settings_basic_config, strings.settings_basic_config_subtitle))
        .child(settings_nav_entry(theme, cx, strings, ToolsSubPage::NetworkConfig, "svg/network-config.svg", strings.settings_network_config, strings.settings_network_config_subtitle))
        .child(settings_nav_entry(theme, cx, strings, ToolsSubPage::DnsConfig, "svg/dns-config.svg", strings.settings_dns_config, strings.settings_dns_config_subtitle))
        .child(settings_nav_entry(theme, cx, strings, ToolsSubPage::RulesConfig, "svg/rules-config.svg", strings.settings_rules_config, strings.settings_rules_config_subtitle))
        .child(settings_nav_entry(theme, cx, strings, ToolsSubPage::ScriptsConfig, "svg/scripts-config.svg", strings.settings_scripts_config, strings.settings_scripts_config_subtitle))
        .child(settings_nav_entry(theme, cx, strings, ToolsSubPage::AdvancedConfig, "svg/advanced-config.svg", strings.settings_advanced_config, strings.settings_advanced_config_subtitle))
        .child(settings_nav_entry(theme, cx, strings, ToolsSubPage::OnDemand, "svg/on-demand.svg", strings.settings_on_demand, strings.settings_on_demand_subtitle))
        .child(settings_nav_entry(theme, cx, strings, ToolsSubPage::Hotkeys, "svg/hotkeys.svg", strings.settings_hotkeys, strings.settings_hotkeys_subtitle))
        .child(settings_nav_entry(theme, cx, strings, ToolsSubPage::BackupRestore, "svg/backup-restore.svg", strings.settings_backup_restore, strings.settings_backup_restore_subtitle))
        .child(settings_nav_entry(theme, cx, strings, ToolsSubPage::Disclaimer, "svg/disclaimer.svg", strings.settings_disclaimer, strings.settings_disclaimer_subtitle))
        .child(settings_nav_entry(theme, cx, strings, ToolsSubPage::About, "svg/about.svg", strings.settings_about, strings.settings_about_subtitle))
}

// ─── Generic navigable settings entry ─────────────────────────

fn settings_nav_entry(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    _strings: &I18nStrings,
    sub_page: ToolsSubPage,
    icon: &'static str,
    title: &str,
    subtitle: &str,
) -> impl IntoElement {
    let id = match sub_page {
        ToolsSubPage::BasicConfig => "settings-nav-basic-config",
        ToolsSubPage::NetworkConfig => "settings-nav-network-config",
        ToolsSubPage::DnsConfig => "settings-nav-dns-config",
        ToolsSubPage::RulesConfig => "settings-nav-rules-config",
        ToolsSubPage::ScriptsConfig => "settings-nav-scripts-config",
        ToolsSubPage::AdvancedConfig => "settings-nav-advanced-config",
        ToolsSubPage::OnDemand => "settings-nav-on-demand",
        ToolsSubPage::Hotkeys => "settings-nav-hotkeys",
        ToolsSubPage::BackupRestore => "settings-nav-backup-restore",
        ToolsSubPage::Disclaimer => "settings-nav-disclaimer",
        ToolsSubPage::About => "settings-nav-about",
        _ => "settings-nav-other",
    };
    settings_nav_row(id, icon, title, subtitle, theme,
        cx.listener(move |this, _, _, cx| {
            this.tools_sub_page = Some(sub_page);
            cx.notify();
        }),
    )
}

// ─── Language Entry ────────────────────────────────

fn language_entry_item(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let current_lang = cx.global::<crate::i18n::I18nManager>().current_language_id.clone();
    let display = language_display_name(&current_lang);
    settings_nav_row("settings-language-entry", "svg/language.svg",
        strings.settings_language, display, theme,
        cx.listener(|this, _, _, cx| {
            this.tools_sub_page = Some(ToolsSubPage::Language);
            cx.notify();
        }),
    )
}

// ─── Theme Entry ───────────────────────────────────

fn theme_entry_item(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let state = cx.global::<AppState>();
    let mode_label = match state.theme_mode {
        ThemeMode::Dark => strings.theme_dark,
        ThemeMode::Light => strings.theme_light,
        ThemeMode::System => strings.theme_system,
    };
    settings_nav_row("settings-theme-entry", "svg/theme.svg",
        strings.settings_theme, mode_label, theme,
        cx.listener(|this, _, _, cx| {
            this.tools_sub_page = Some(ToolsSubPage::Theme);
            cx.notify();
        }),
    )
}

/// Reusable settings row with icon, title, subtitle, and chevron.
fn settings_nav_row(
    id: &'static str,
    icon: &'static str,
    title: &str,
    subtitle: &str,
    theme: &Theme,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let title = title.to_string();
    let subtitle = subtitle.to_string();
    div()
        .id(id)
        .flex().items_center().justify_between()
        .px(px(16.0)).py(px(14.0)).rounded(px(6.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgb(theme.surface)))
        .on_click(on_click)
        .child(
            div().flex().items_center().gap(px(12.0))
                .child(svg().path(icon).size(px(18.0)).text_color(rgb(theme.text_secondary)))
                .child(
                    div().flex().flex_col().gap(px(2.0))
                        .child(div().text_size(px(14.0)).text_color(rgb(theme.text_primary)).child(title))
                        .child(div().text_size(px(12.0)).text_color(rgb(theme.text_secondary)).child(subtitle)),
                ),
        )
        .child(div().text_size(px(14.0)).text_color(rgb(theme.text_disabled)).child("\u{203A}"))
}

// ─── Reusable Visual Widgets ─────────────────────────

fn section_label(theme: &Theme, icon: &'static str, label: &str) -> impl IntoElement {
    let label = label.to_string();
    div()
        .flex().items_center().gap(px(6.0)).px(px(16.0)).py(px(8.0))
        .text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD)
        .text_color(rgb(theme.text_secondary))
        .child(svg().path(icon).size(px(16.0)).text_color(rgb(theme.text_secondary)))
        .child(label)
}

fn toggle_switch(theme: &Theme, enabled: bool) -> impl IntoElement {
    let knob_offset = if enabled { px(20.0) } else { px(3.0) };
    div()
        .w(px(40.0)).h(px(22.0)).rounded(px(11.0))
        .bg(if enabled { Hsla::from(rgb(theme.accent)) } else { Hsla::from(rgb(theme.border)) })
        .flex().items_center()
        .child(div().w(px(16.0)).h(px(16.0)).ml(knob_offset).rounded(px(8.0)).bg(rgb(0xffffff)))
}

fn text_display_row(theme: &Theme, label: &str, value: &str) -> impl IntoElement {
    let label = label.to_string();
    let value = value.to_string();
    div()
        .flex().items_center().justify_between()
        .px(px(16.0)).py(px(10.0)).rounded(px(6.0))
        .hover(|s| s.bg(rgb(theme.surface)))
        .child(div().text_size(px(13.0)).text_color(rgb(theme.text_primary)).child(label))
        .child(div().text_size(px(12.0)).text_color(rgb(theme.text_secondary)).font_family("monospace").child(value))
}

fn placeholder_notice(theme: &Theme, text: &str) -> impl IntoElement {
    let text = text.to_string();
    div().flex().flex_col().items_center().justify_center().py(px(32.0))
        .child(div().text_size(px(13.0)).text_color(rgb(theme.text_disabled)).child(text))
}

fn radio_dot(theme: &Theme, is_active: bool) -> impl IntoElement {
    div()
        .w(px(18.0)).h(px(18.0)).rounded(px(9.0)).border_1()
        .border_color(if is_active { Hsla::from(rgb(theme.accent)) } else { Hsla::from(rgb(theme.border)) })
        .bg(if is_active { Hsla::from(rgb(theme.accent)) } else { Hsla::from(rgba(0x00000000)) })
        .flex().items_center().justify_center()
        .when(is_active, |s| s.child(div().w(px(6.0)).h(px(6.0)).rounded(px(3.0)).bg(rgb(0xffffff))))
}

// ─── Language Sub-page ───────────────────────────────

pub(super) fn language_sub_page_body(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let current_lang = cx.global::<crate::i18n::I18nManager>().current_language_id.clone();
    div().flex().flex_col().gap(px(2.0)).px(px(24.0))
        .child(div().flex().items_center().gap(px(6.0)).px(px(16.0)).py(px(8.0))
            .text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(theme.text_secondary))
            .child(svg().path("svg/language.svg").size(px(16.0)).text_color(rgb(theme.text_secondary)))
            .child(strings.settings_language))
        .child(lang_option_row(theme, "en-US", "English", &current_lang))
        .child(lang_option_row(theme, "zh-CN", "简体中文", &current_lang))
        .child(lang_option_row(theme, "ja-JP", "日本語", &current_lang))
        .child(lang_option_row(theme, "ko-KR", "한국어", &current_lang))
        .child(lang_option_row(theme, "de-DE", "Deutsch", &current_lang))
        .child(lang_option_row(theme, "fr-FR", "Français", &current_lang))
        .child(lang_option_row(theme, "es-ES", "Español", &current_lang))
        .child(lang_option_row(theme, "pt-BR", "Português", &current_lang))
}

fn lang_option_row(
    theme: &Theme, lang_id: &'static str, lang_name: &'static str, current_lang: &str,
) -> impl IntoElement {
    let is_active = lang_id == current_lang;
    let lang_id_owned = lang_id.to_string();
    let lang_name = lang_name.to_string();
    div().id(lang_id)
        .flex().items_center().gap(px(10.0)).px(px(16.0)).py(px(12.0)).rounded(px(6.0))
        .cursor_pointer().hover(|s| s.bg(rgb(theme.surface)))
        .on_click(move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
            crate::i18n::I18nManager::init_with_language_id(cx, &lang_id_owned);
            save_language_id(&lang_id_owned);
            cx.refresh_windows();
        })
        .child(radio_dot(theme, is_active))
        .child(div().text_size(px(14.0))
            .text_color(if is_active { Hsla::from(rgb(theme.accent)) } else { Hsla::from(rgb(theme.text_primary)) })
            .font_weight(if is_active { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
            .child(lang_name))
}

// ─── Theme Sub-page ──────────────────────────────────

pub(super) fn theme_sub_page_body(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let state = cx.global::<AppState>();
    let current_mode = state.theme_mode;
    let current_accent = state.accent_color;
    div().flex().flex_col().gap(px(12.0)).px(px(24.0))
        .child(section_label(theme, "svg/theme.svg", strings.theme_mode_label))
        .child(theme_mode_row(theme, strings.theme_system, current_mode == ThemeMode::System, ThemeMode::System))
        .child(theme_mode_row(theme, strings.theme_light, current_mode == ThemeMode::Light, ThemeMode::Light))
        .child(theme_mode_row(theme, strings.theme_dark, current_mode == ThemeMode::Dark, ThemeMode::Dark))
        .child(div().h(px(8.0)))
        .child(section_label(theme, "svg/palette.svg", strings.accent_color_label))
        .child(accent_color_grid(theme, current_accent))
}

fn theme_mode_row(theme: &Theme, label: &str, is_active: bool, mode: ThemeMode) -> impl IntoElement {
    let label = label.to_string();
    let id = match mode {
        ThemeMode::Light => "theme-mode-light",
        ThemeMode::Dark => "theme-mode-dark",
        ThemeMode::System => "theme-mode-system",
    };
    div().id(id)
        .flex().items_center().gap(px(10.0)).px(px(16.0)).py(px(12.0)).rounded(px(6.0))
        .cursor_pointer().hover(|s| s.bg(rgb(theme.surface)))
        .on_click(move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
            cx.update_global::<AppState, _>(|state, _cx| { state.theme_mode = mode; save_theme_mode(mode); });
            cx.refresh_windows();
        })
        .child(radio_dot(theme, is_active))
        .child(div().text_size(px(14.0))
            .text_color(if is_active { Hsla::from(rgb(theme.accent)) } else { Hsla::from(rgb(theme.text_primary)) })
            .font_weight(if is_active { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
            .child(label))
}

fn accent_color_grid(theme: &Theme, current: AccentColor) -> impl IntoElement {
    div().flex().flex_wrap().gap(px(10.0)).px(px(16.0))
        .children(AccentColor::ALL.iter().map(move |&accent| accent_swatch(theme, accent, accent == current)))
}

fn accent_swatch(theme: &Theme, accent: AccentColor, is_active: bool) -> impl IntoElement {
    let swatch_color = accent.swatch();
    let size = if is_active { 36.0 } else { 32.0 };
    let border_color = if is_active { Hsla::from(rgb(theme.text_primary)) } else { Hsla::from(rgba(0x00000000)) };
    div().id(accent.label())
        .w(px(40.0)).h(px(40.0)).flex().items_center().justify_center().rounded(px(8.0))
        .cursor_pointer().hover(|s| s.bg(rgb(theme.surface)))
        .on_click(move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
            cx.update_global::<AppState, _>(|state, _cx| { state.accent_color = accent; save_accent_color(accent); });
            cx.refresh_windows();
        })
        .child(div().w(px(size)).h(px(size)).rounded(px(size / 2.0)).bg(rgb(swatch_color)).border_2().border_color(border_color))
}

// ─── Helper macro for interactive rows ───────────────

/// Builds a clickable toggle row inline.
/// `.id()` is **required** before `.on_click()` because GPUI 0.2.2 only
/// exposes `on_click` on `Stateful<Div>` (via `StatefulInteractiveElement`),
/// not on `Div` itself.
macro_rules! toggle_row {
    ($cx:expr, $theme:expr, $label:expr, $enabled:expr, $field:ident) => {
        div()
            .id(stringify!($field))
            .flex().items_center().justify_between()
            .px(px(16.0)).py(px(10.0)).rounded(px(6.0))
            .cursor_pointer().hover(|s| s.bg(rgb($theme.surface)))
            .on_click($cx.listener(move |this, _: &ClickEvent, _: &mut Window, cx| {
                this.settings_data.$field = !this.settings_data.$field;
                cx.notify();
            }))
            .child(div().text_size(px(13.0)).text_color(rgb($theme.text_primary)).child($label))
            .child(toggle_switch($theme, $enabled))
    };
}

/// Builds a clickable select row inline.
macro_rules! select_row {
    ($cx:expr, $theme:expr, $label:expr, $idx:expr, $opts:expr, $field:ident) => {
        div()
            .id(stringify!($field))
            .flex().items_center().justify_between()
            .px(px(16.0)).py(px(10.0)).rounded(px(6.0))
            .cursor_pointer().hover(|s| s.bg(rgb($theme.surface)))
            .on_click($cx.listener(move |this, _: &ClickEvent, _: &mut Window, cx| {
                let len = $opts.len();
                this.settings_data.$field = (this.settings_data.$field + 1) % len;
                cx.notify();
            }))
            .child(div().text_size(px(13.0)).text_color(rgb($theme.text_primary)).child($label))
            .child(div().text_size(px(13.0)).text_color(rgb($theme.accent)).font_weight(FontWeight::SEMIBOLD)
                .child($opts.get($idx).copied().unwrap_or("—")))
    };
}

// ─── Basic Config Sub-page ───────────────────────────

pub(super) fn basic_config_sub_page_body(
    theme: &Theme,
    data: SettingsData,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    div().flex().flex_col().gap(px(12.0)).px(px(24.0))
        .child(section_label(theme, "svg/basic-config.svg", strings.settings_section_port))
        .child(text_display_row(theme, "Mixed Port", &data.mixed_port))
        .child(text_display_row(theme, "HTTP Port", &data.http_port))
        .child(text_display_row(theme, "SOCKS Port", &data.socks_port))
        .child(text_display_row(theme, "Redir Port", &data.redir_port))
        .child(text_display_row(theme, "TProxy Port", &data.tproxy_port))
        .child(div().h(px(4.0)))
        .child(section_label(theme, "svg/basic-config.svg", strings.settings_section_log))
        .child(select_row!(cx, theme, strings.settings_section_log, data.log_level, LOG_LEVELS, log_level))
        .child(div().h(px(4.0)))
        .child(section_label(theme, "svg/basic-config.svg", strings.settings_section_mode))
        .child(select_row!(cx, theme, strings.settings_section_mode, data.run_mode, RUN_MODES, run_mode))
}

// ─── Network Config Sub-page ─────────────────────────

pub(super) fn network_config_sub_page_body(
    theme: &Theme,
    data: SettingsData,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    div().flex().flex_col().gap(px(12.0)).px(px(24.0))
        .child(section_label(theme, "svg/network-config.svg", strings.settings_section_network))
        .child(toggle_row!(cx, theme, "IPv6", data.ipv6_enabled, ipv6_enabled))
        .child(text_display_row(theme, "Interface", &data.interface))
        .child(text_display_row(theme, "Routing Mark", &data.routing_mark))
        .child(text_display_row(theme, "TCP Concurrency", &data.tcp_concurrency))
        .child(select_row!(cx, theme, "UDP Fallback Policy", data.udp_fallback, UDP_FALLBACKS, udp_fallback))
}

// ─── DNS Config Sub-page ─────────────────────────────

pub(super) fn dns_config_sub_page_body(
    theme: &Theme,
    data: SettingsData,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    div().flex().flex_col().gap(px(12.0)).px(px(24.0))
        .child(section_label(theme, "svg/dns-config.svg", strings.settings_section_dns))
        .child(toggle_row!(cx, theme, "Enable", data.dns_enabled, dns_enabled))
        .child(toggle_row!(cx, theme, "IPv6", data.dns_ipv6, dns_ipv6))
        .child(select_row!(cx, theme, "Enhanced Mode", data.enhanced_mode, ENHANCED_MODES, enhanced_mode))
        .child(text_display_row(theme, "Fake-IP Range", &data.fake_ip_range))
        .child(text_display_row(theme, "Fallback DNS", &data.fallback_dns))
        .child(text_display_row(theme, "Default DNS", &data.default_dns))
}

// ─── Rules Config Sub-page ───────────────────────────

pub(super) fn rules_config_sub_page_body(
    theme: &Theme,
    _data: SettingsData,
    _cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    div().flex().flex_col().gap(px(12.0)).px(px(24.0))
        .child(section_label(theme, "svg/rules-config.svg", strings.settings_rules_config))
        .child(text_display_row(theme, "Rule Provider 1", "default-rules.yaml"))
        .child(text_display_row(theme, "Rule Provider 2", "custom-rules.yaml"))
        .child(text_display_row(theme, "Matching Strategy", "Longest Prefix"))
        .child(placeholder_notice(theme, "Rule management with add/edit/remove will be available here."))
}

// ─── Scripts Config Sub-page ─────────────────────────

pub(super) fn scripts_config_sub_page_body(
    theme: &Theme,
    _data: SettingsData,
    _cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    div().flex().flex_col().gap(px(12.0)).px(px(24.0))
        .child(section_label(theme, "svg/scripts-config.svg", strings.settings_scripts_config))
        .child(text_display_row(theme, "Script 1", "example.js"))
        .child(text_display_row(theme, "Script 2", "custom-req.js"))
        .child(placeholder_notice(theme, "Script management with enable/disable and edit will be available here."))
}

// ─── Advanced Config Sub-page ────────────────────────

pub(super) fn advanced_config_sub_page_body(
    theme: &Theme,
    data: SettingsData,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    div().flex().flex_col().gap(px(12.0)).px(px(24.0))
        .child(section_label(theme, "svg/advanced-config.svg", strings.settings_section_tun))
        .child(toggle_row!(cx, theme, "Enable", data.tun_enabled, tun_enabled))
        .child(select_row!(cx, theme, "Stack", data.tun_stack, TUN_STACKS, tun_stack))
        .child(text_display_row(theme, "Device", &data.tun_device))
        .child(toggle_row!(cx, theme, "Auto Route", data.tun_auto_route, tun_auto_route))
        .child(toggle_row!(cx, theme, "Auto Detect Interface", data.tun_auto_detect, tun_auto_detect))
        .child(div().h(px(4.0)))
        .child(section_label(theme, "svg/advanced-config.svg", strings.settings_section_sniffer))
        .child(toggle_row!(cx, theme, "Enable", data.sniffer_enabled, sniffer_enabled))
        .child(toggle_row!(cx, theme, "Override Dest", data.sniffer_override_dest, sniffer_override_dest))
        .child(text_display_row(theme, "Ports", &data.sniffer_ports))
        .child(toggle_row!(cx, theme, "Force DNS Mapping", data.sniffer_force_dns, sniffer_force_dns))
}

// ─── On-Demand Sub-page ──────────────────────────────

pub(super) fn on_demand_sub_page_body(
    theme: &Theme,
    data: SettingsData,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    div().flex().flex_col().gap(px(12.0)).px(px(24.0))
        .child(section_label(theme, "svg/on-demand.svg", strings.settings_on_demand))
        .child(toggle_row!(cx, theme, "Enable", data.on_demand_enabled, on_demand_enabled))
        .child(select_row!(cx, theme, "Trigger Mode", data.on_demand_trigger, ON_DEMAND_TRIGGERS, on_demand_trigger))
        .child(text_display_row(theme, "WiFi List", &data.on_demand_wifi_list))
        .child(select_row!(cx, theme, "Fallback Policy", data.on_demand_fallback, ON_DEMAND_FALLBACKS, on_demand_fallback))
}

// ─── Hotkeys Sub-page ────────────────────────────────

pub(super) fn hotkeys_sub_page_body(
    theme: &Theme,
    data: SettingsData,
    _cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    div().flex().flex_col().gap(px(12.0)).px(px(24.0))
        .child(section_label(theme, "svg/hotkeys.svg", strings.settings_hotkeys))
        .child(hotkey_display_row(theme, "Toggle Theme", &data.hotkey_app))
        .child(hotkey_display_row(theme, "Dashboard", &data.hotkey_dashboard))
        .child(hotkey_display_row(theme, "Proxies", &data.hotkey_proxies))
        .child(hotkey_display_row(theme, "Settings", &data.hotkey_settings))
        .child(hotkey_display_row(theme, "Toggle Core", &data.hotkey_core))
        .child(hotkey_display_row(theme, "Search", &data.hotkey_search))
        .child(placeholder_notice(theme, "Click a hotkey row to rebind. Recording will be available here."))
}

fn hotkey_display_row(theme: &Theme, label: &str, keys: &str) -> impl IntoElement {
    let label = label.to_string();
    let keys = keys.to_string();
    div().flex().items_center().justify_between()
        .px(px(16.0)).py(px(10.0)).rounded(px(6.0))
        .cursor_pointer().hover(|s| s.bg(rgb(theme.surface)))
        .child(div().text_size(px(13.0)).text_color(rgb(theme.text_primary)).child(label))
        .child(
            div().flex().gap(px(4.0))
                .children(keys.split('+').map(|k| {
                    div().px(px(6.0)).py(px(2.0)).rounded(px(4.0))
                        .border_1().border_color(rgb(theme.border)).bg(rgb(theme.content_bg))
                        .text_size(px(11.0)).font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(theme.accent)).font_family("monospace")
                        .child(k.trim().to_string())
                })),
        )
}

// ─── Backup & Restore Sub-page ───────────────────────

pub(super) fn backup_restore_sub_page_body(
    theme: &Theme,
    data: SettingsData,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    div().flex().flex_col().gap(px(12.0)).px(px(24.0))
        .child(section_label(theme, "svg/backup-restore.svg", strings.settings_backup_restore))
        .child(text_display_row(theme, "WebDAV URL", &data.backup_webdav_url))
        .child(select_row!(cx, theme, "Auto Backup", data.backup_auto_interval, BACKUP_INTERVALS, backup_auto_interval))
        .child(select_row!(cx, theme, "Export Format", data.backup_export_format, BACKUP_FORMATS, backup_export_format))
        .child(text_display_row(theme, "Last Backup", &data.backup_last_time))
        .child(div().h(px(8.0)))
        .child(div().flex().flex_col().gap(px(8.0)).px(px(16.0))
            .child(
                div().id("backup-now-btn")
                    .flex().items_center().gap(px(10.0))
                    .px(px(16.0)).py(px(12.0)).rounded(px(8.0))
                    .bg(rgb(theme.accent)).text_color(rgb(0xffffff))
                    .cursor_pointer().hover(|s| s.bg(rgb(theme.accent_hover)))
                    .on_click(cx.listener(move |this, _: &ClickEvent, _: &mut Window, cx| {
                        this.settings_data.backup_last_time = "Just now".into();
                        this.toasts.push(ToastData::success("Backup completed successfully"));
                        cx.notify();
                    }))
                    .child(svg().path("svg/backup-restore.svg").size(px(16.0)).text_color(rgb(0xffffff)))
                    .child(div().text_size(px(14.0)).font_weight(FontWeight::SEMIBOLD).child("Backup Now")),
            )
            .child(
                div().id("export-config-btn")
                    .flex().items_center().gap(px(10.0))
                    .px(px(16.0)).py(px(12.0)).rounded(px(8.0))
                    .bg(rgb(theme.accent)).text_color(rgb(0xffffff))
                    .cursor_pointer().hover(|s| s.bg(rgb(theme.accent_hover)))
                    .on_click(cx.listener(|this, _: &ClickEvent, _: &mut Window, cx| {
                        this.active_dialog = Some(DialogParams::confirm(
                            "Export Configuration",
                            "Save current configuration to a file. All settings including profiles, rules, and preferences will be included.",
                        ));
                        cx.notify();
                    }))
                    .child(svg().path("svg/basic-config.svg").size(px(16.0)).text_color(rgb(0xffffff)))
                    .child(div().text_size(px(14.0)).font_weight(FontWeight::SEMIBOLD).child("Export Configuration")),
            )
            .child(
                div().id("restore-backup-btn")
                    .flex().items_center().gap(px(10.0))
                    .px(px(16.0)).py(px(12.0)).rounded(px(8.0))
                    .bg(rgb(theme.status_error)).text_color(rgb(0xffffff))
                    .cursor_pointer().hover(|s| s.bg(rgb(0xdc2626)))
                    .on_click(cx.listener(|this, _: &ClickEvent, _: &mut Window, cx| {
                        this.active_dialog = Some(DialogParams::danger(
                            "Restore from Backup",
                            "This will overwrite your current configuration with the backup data. This action cannot be undone. Continue?",
                        ));
                        cx.notify();
                    }))
                    .child(svg().path("svg/advanced-config.svg").size(px(16.0)).text_color(rgb(0xffffff)))
                    .child(div().text_size(px(14.0)).font_weight(FontWeight::SEMIBOLD).child("Restore from Backup")),
            )
        )
}

// ─── Disclaimer Sub-page ─────────────────────────────

pub(super) fn disclaimer_sub_page_body(
    theme: &Theme,
    _data: SettingsData,
    _cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let disclaimer = strings.disclaimer_text.to_string();
    div().flex().flex_col().gap(px(16.0)).px(px(32.0)).py(px(24.0))
        .child(div().flex().flex_col().rounded(px(12.0))
            .bg(rgb(theme.surface)).border_1().border_color(rgb(theme.border_light))
            .px(px(24.0)).py(px(24.0))
            .child(div().text_size(px(28.0)).mb(px(12.0)).child("\u{26A0}\u{FE0F}"))
            .child(div().text_size(px(14.0)).text_color(rgb(theme.text_primary)).line_height(px(22.0)).child(disclaimer)))
}

// ─── About Sub-page ──────────────────────────────────

pub(super) fn about_sub_page_body(
    theme: &Theme,
    _data: SettingsData,
    _cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    div().flex().flex_col().gap(px(16.0)).px(px(24.0))
        .child(div().flex().flex_col().items_center().py(px(16.0))
            .child(div().flex().items_center().justify_center()
                .w(px(64.0)).h(px(64.0)).rounded(px(16.0))
                .bg(rgba(theme.accent & 0x00FFFFFF | 0x15000000))
                .child(svg().path("svg/dashboard.svg").size(px(36.0)).text_color(rgb(theme.accent))))
            .child(div().h(px(8.0)))
            .child(div().text_size(px(20.0)).font_weight(FontWeight::BOLD).text_color(rgb(theme.text_primary)).child(strings.app_name))
            .child(div().h(px(4.0)))
            .child(div().text_size(px(13.0)).text_color(rgb(theme.text_secondary)).child(strings.about_description)))
        .child(div().flex().flex_col().rounded(px(12.0))
            .bg(rgb(theme.surface)).border_1().border_color(rgb(theme.border_light)).overflow_hidden()
            .child(text_display_row(theme, strings.about_version, "0.1.0"))
            .child(div().h(px(1.0)).bg(rgb(theme.border_light)))
            .child(text_display_row(theme, strings.about_license, "MIT"))
            .child(div().h(px(1.0)).bg(rgb(theme.border_light)))
            .child(text_display_row(theme, strings.about_website, "github.com/sandhope/hopen"))
            .child(div().h(px(1.0)).bg(rgb(theme.border_light)))
            .child(text_display_row(theme, strings.about_source, "github.com/sandhope/hopen")))
        .child(div().flex().items_center().justify_center().py(px(12.0))
            .child(div().text_size(px(12.0)).text_color(rgb(theme.text_disabled)).child(strings.about_copyright)))
}
