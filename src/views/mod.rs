/// Page view skeletons for the Hopen GPUI client.
///
/// Each function returns a view for the corresponding page.
/// Theme-aware: all colours come from the `Theme` parameter.

use gpui::*;
use gpui::prelude::*;

use crate::i18n::{I18nStrings, language_display_name};
use crate::navigation::{Page, ToolsSubPage};
use crate::theme::{Theme, CARD_RADIUS};
use crate::{
    save_theme_mode, save_language_id, AppState, OutboundMode,
    SetOutboundMode, ToggleCore, ToggleSystemProxy, ToggleTunMode,
};

/// Route to the correct page view based on the current navigation state.
///
/// `cx` is the AppView context, needed for interactive elements (e.g. theme toggle).
/// `tools_sub_page` is the active drill-down sub-page within Settings, or `None`
/// for the main settings list.
pub fn render_page(
    page: Page,
    tools_sub_page: Option<ToolsSubPage>,
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
) -> impl IntoElement {
    let strings = cx.global::<crate::i18n::I18nManager>().strings_arc();

    // Compute header and content, accounting for Settings sub-pages.
    // We collect both into AnyElement so the outer div has a single concrete return type.
    let (header_elem, content_elem): (AnyElement, AnyElement) = if page == Page::Tools {
        if let Some(sub) = tools_sub_page {
            let header = render_sub_page_header(sub, theme, cx, &strings).into_any_element();
            let body = render_sub_page_body(sub, theme, cx, &strings).into_any_element();
            (header, body)
        } else {
            let title = strings.page_title(page);
            (
                page_header(title, theme).into_any_element(),
                tools_view(theme, cx, &strings).into_any_element(),
            )
        }
    } else {
        let title = strings.page_title(page);
        let content: AnyElement = match page {
            Page::Dashboard => dashboard_view(theme, cx, &strings).into_any_element(),
            Page::Proxies => proxies_view(theme, &strings).into_any_element(),
            Page::Profiles => profiles_view(theme, &strings).into_any_element(),
            Page::Requests => requests_view(theme, &strings).into_any_element(),
            Page::Connections => connections_view(theme, &strings).into_any_element(),
            Page::Resources => resources_view(theme, &strings).into_any_element(),
            Page::Logs => logs_view(theme, &strings).into_any_element(),
            Page::Tools => unreachable!(),
        };
        (page_header(title, theme).into_any_element(), content)
    };

    div()
        .flex()
        .flex_col()
        .size_full()
        .overflow_y_hidden()
        .child(header_elem)
        .child(content_elem)
}

/// Standard page title header (used by all non-sub pages).
fn page_header(title: &str, theme: &Theme) -> impl IntoElement {
    let title = title.to_string();
    div()
        .flex()
        .px(px(24.0))
        .pt(px(24.0))
        .pb(px(16.0))
        .child(
            div()
                .text_size(px(22.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(theme.text_primary))
                .child(title),
        )
}

/// Header with back button for Settings sub-pages.
fn render_sub_page_header(
    sub: ToolsSubPage,
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let title = match sub {
        ToolsSubPage::Language => strings.page_title_language,
    };
    div()
        .flex()
        .items_center()
        .justify_between()
        .px(px(24.0))
        .pt(px(24.0))
        .pb(px(16.0))
        .child(
            div()
                .text_size(px(22.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(theme.text_primary))
                .child(title),
        )
        .child(back_button(theme, cx))
}

/// Body content for Settings sub-pages.
fn render_sub_page_body(
    sub: ToolsSubPage,
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    match sub {
        ToolsSubPage::Language => language_sub_page_body(theme, cx, strings),
    }
}

// ─── Dashboard ─────────────────────────────────────────────────
//
// Layout (matching FlClash default widgets):
// ┌────────────────────────────────────────┐
// │  Network Speed (full width)            │
// ├──────────────────┬─────────────────────┤
// │ Proxy Control    │ Traffic Usage       │
// │  System Proxy    │  (donut + total)    │
// │  TUN Mode        │                     │
// │  Outbound Mode   │ Network Detection   │
// │                  │  (IP / ISP)         │
// │ LAN IP           │                     │
// ├──────────────────┴─────────────────────┤
// │           Core Control Button          │
// └────────────────────────────────────────┘

fn dashboard_view(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let state = cx.global::<AppState>();
    let core_running = state.core_running;
    let system_proxy = state.system_proxy;
    let tun_mode = state.tun_mode;
    let outbound_mode = state.outbound_mode;
    let upload_speed = state.upload_speed;
    let download_speed = state.download_speed;
    let upload_total = state.upload_total;
    let download_total = state.download_total;
    let public_ip = state.public_ip.clone();
    let lan_ip = state.lan_ip.clone();
    let isp = state.isp.clone();

    div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .px(px(24.0))
        .py(px(8.0))
        // ── Top: Network Speed Card ─────────────────────────
        .child(network_speed_card(
            theme,
            upload_speed,
            download_speed,
            core_running,
            strings,
        ))
        // ── Middle: Two-column grid ──────────────────────────
        .child(
            div()
                .flex()
                .gap(px(16.0))
                // Left column
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .flex_1()
                        // Proxy control card
                        .child(proxy_control_card(
                            theme, system_proxy, tun_mode, outbound_mode, strings,
                        ))
                        // LAN IP card
                        .child(lan_ip_card(theme, lan_ip.as_deref(), strings)),
                )
                // Right column
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.0))
                        .flex_1()
                        // Traffic stats card
                        .child(traffic_stats_card(
                            theme, upload_total, download_total, strings,
                        ))
                        // Network detection card
                        .child(network_detection_card(
                            theme, public_ip.as_deref(), isp.as_deref(), strings,
                        )),
                ),
        )
        // ── Bottom: Core Control Button ────────────────────
        .child(core_control_button(theme, core_running, strings))
}

// ─── Network Speed Card ────────────────────────────────────

/// Full-width card showing real-time upload/download speed with a mini graph.
fn network_speed_card(
    theme: &Theme,
    upload_speed: u64,
    download_speed: u64,
    core_running: bool,
    strings: &I18nStrings,
) -> impl IntoElement {
    let up_str = format_speed(upload_speed);
    let down_str = format_speed(download_speed);
    let up_col = rgb(theme.status_info);
    let down_col = rgb(theme.status_success);
    let speed_color = if core_running {
        rgb(theme.text_primary)
    } else {
        rgb(theme.text_disabled)
    };
    let title = strings.dashboard_network_speed;
    let upload_label = strings.dashboard_upload;
    let download_label = strings.dashboard_download;

    div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .p(px(24.0))
        .rounded(px(CARD_RADIUS))
        .bg(rgb(theme.surface))
        .border_1()
        .border_color(rgb(theme.border_light))
        .child(
            div()
                .text_size(px(14.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme.text_primary))
                .child(title),
        )
        .child(
            div().flex().gap(px(32.0)).items_end()
                // Upload
                .child(
                    div().flex().flex_col().gap(px(4.0))
                        .child(
                            div().flex().items_center().gap(px(6.0))
                                .child(
                                    div().text_size(px(13.0)).text_color(up_col).child("\u{2191}"), // ↑
                                )
                                .child(
                                    div().text_size(px(13.0)).text_color(rgb(theme.text_secondary))
                                        .child(upload_label),
                                ),
                        )
                        .child(
                            div().text_size(px(24.0)).font_weight(FontWeight::BOLD)
                                .text_color(speed_color).child(up_str),
                        ),
                )
                // Download
                .child(
                    div().flex().flex_col().gap(px(4.0))
                        .child(
                            div().flex().items_center().gap(px(6.0))
                                .child(
                                    div().text_size(px(13.0)).text_color(down_col).child("\u{2193}"), // ↓
                                )
                                .child(
                                    div().text_size(px(13.0)).text_color(rgb(theme.text_secondary))
                                        .child(download_label),
                                ),
                        )
                        .child(
                            div().text_size(px(24.0)).font_weight(FontWeight::BOLD)
                                .text_color(speed_color).child(down_str),
                        ),
                )
                // Mini speed bar visual
                .child(speed_bars(theme, upload_speed, download_speed, core_running)),
        )
}

/// Simple bar visualizer for speed (placeholder mini-graph).
fn speed_bars(
    theme: &Theme,
    _up: u64,
    _down: u64,
    core_running: bool,
) -> impl IntoElement {
    let bar_color = if core_running {
        rgb(theme.accent)
    } else {
        rgb(theme.border)
    };

    div().flex().gap(px(3.0)).flex_1().justify_end().items_end().children(
        // Draw 8 bars of varying heights for visual effect
        [0.6, 0.8, 0.5, 1.0, 0.7, 0.9, 0.4, 0.75]
            .iter()
            .map(move |&h| {
                div()
                    .w(px(4.0))
                    .h(px(8.0 + h * 24.0))
                    .rounded(px(2.0))
                    .bg(bar_color)
                    .opacity(h as f32)
            }),
    )
}

// ─── Proxy Control Card (Left Column Top) ─────────────────

/// Card containing System Proxy switch, TUN Mode switch, and Outbound Mode selector.
fn proxy_control_card(
    theme: &Theme,
    system_proxy: bool,
    tun_mode: bool,
    outbound_mode: OutboundMode,
    strings: &I18nStrings,
) -> impl IntoElement {
    // System Proxy switch visuals (eager, used in closure)
    let sp_active = system_proxy;
    let sp_state_text = if sp_active { strings.dashboard_status_on } else { strings.dashboard_status_off };
    let sp_state_color = if sp_active { rgb(theme.status_success) } else { rgb(theme.text_disabled) };
    let sp_track = if sp_active { rgb(theme.accent) } else { rgb(theme.border) };

    // TUN Mode switch visuals
    let tun_active = tun_mode;
    let tun_state_text = if tun_active { strings.dashboard_status_on } else { strings.dashboard_status_off };
    let tun_state_color = if tun_active { rgb(theme.status_success) } else { rgb(theme.text_disabled) };
    let tun_track = if tun_active { rgb(theme.accent) } else { rgb(theme.border) };

    let title = strings.dashboard_proxy_control;
    let sys_proxy_label = strings.dashboard_system_proxy;
    let tun_label = strings.dashboard_tun_mode;
    let outbound_label = strings.dashboard_outbound_mode;

    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .p(px(20.0))
        .rounded(px(CARD_RADIUS))
        .bg(rgb(theme.surface))
        .border_1()
        .border_color(rgb(theme.border_light))
        .child(
            div()
                .text_size(px(14.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme.text_primary))
                .child(title),
        )
        // System Proxy switch
        .child(
            div()
                .id("switch-system-proxy")
                .flex().items_center().justify_between().py(px(2.0))
                .cursor_pointer()
                .on_click(|_: &ClickEvent, _: &mut Window, cx: &mut App| {
                    cx.dispatch_action(&ToggleSystemProxy);
                })
                .child(div().text_size(px(13.0)).text_color(rgb(theme.text_primary)).child(sys_proxy_label))
                .child(switch_pill(sp_state_text, sp_state_color, sp_track)),
        )
        // Divider
        .child(div().h(px(1.0)).bg(rgb(theme.border_light)))
        // TUN Mode switch
        .child(
            div()
                .id("switch-tun-mode")
                .flex().items_center().justify_between().py(px(2.0))
                .cursor_pointer()
                .on_click(|_: &ClickEvent, _: &mut Window, cx: &mut App| {
                    cx.dispatch_action(&ToggleTunMode);
                })
                .child(div().text_size(px(13.0)).text_color(rgb(theme.text_primary)).child(tun_label))
                .child(switch_pill(tun_state_text, tun_state_color, tun_track)),
        )
        // Divider
        .child(div().h(px(1.0)).bg(rgb(theme.border_light)))
        // Outbound Mode section
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(
                    div().text_size(px(13.0)).text_color(rgb(theme.text_secondary))
                        .child(outbound_label),
                )
                .child(outbound_mode_selector(theme, outbound_mode, strings)),
        )
}

/// Visual pill for a switch — state label + track with knob.
fn switch_pill(
    state_text: &str,
    state_color: impl Into<Hsla>,
    track_bg: impl Into<Hsla>,
) -> impl IntoElement {
    let text = state_text.to_string();
    let sc = state_color.into();
    let tb = track_bg.into();
    div().flex().items_center().gap(px(6.0))
        .child(
            div().text_size(px(12.0)).font_weight(FontWeight::MEDIUM)
                .text_color(sc).child(text),
        )
        .child(
            div()
                .flex().w(px(40.0)).h(px(22.0))
                .rounded(px(11.0)).bg(tb)
                .px(px(2.0)).items_center().justify_end()
                .child(
                    div().w(px(18.0)).h(px(18.0))
                        .rounded(px(9.0)).bg(rgb(0xffffff)),
                ),
        )
}

// ─── LAN IP Card (Left Column Bottom) ─────────────────────

fn lan_ip_card(theme: &Theme, lan_ip: Option<&str>, strings: &I18nStrings) -> impl IntoElement {
    let ip = lan_ip.unwrap_or(strings.network_detecting).to_string();
    let title = strings.dashboard_lan_ip;

    div()
        .flex()
        .flex_col()
        .gap(px(6.0))
        .p(px(20.0))
        .rounded(px(CARD_RADIUS))
        .bg(rgb(theme.surface))
        .border_1()
        .border_color(rgb(theme.border_light))
        .child(
            div()
                .text_size(px(14.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme.text_primary))
                .child(title),
        )
        .child(
            div()
                .text_size(px(16.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(theme.text_secondary))
                .child(ip),
        )
}

// ─── Traffic Stats Card (Right Column Top) ─────────────────

/// Card with ring chart and upload/download totals.
fn traffic_stats_card(
    theme: &Theme,
    upload_total: u64,
    download_total: u64,
    strings: &I18nStrings,
) -> impl IntoElement {
    let up_str = format_bytes(upload_total);
    let down_str = format_bytes(download_total);
    let total = upload_total + download_total;
    let up_pct = if total > 0 {
        (upload_total as f64 / total as f64 * 100.0) as u32
    } else {
        50
    };
    let title = strings.dashboard_traffic_usage;
    let upload_label = strings.dashboard_upload;
    let download_label = strings.dashboard_download;

    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .p(px(20.0))
        .rounded(px(CARD_RADIUS))
        .bg(rgb(theme.surface))
        .border_1()
        .border_color(rgb(theme.border_light))
        .child(
            div()
                .text_size(px(14.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme.text_primary))
                .child(title),
        )
        .child(
            div().flex().gap(px(20.0)).items_center()
                // Ring chart visual
                .child(ring_chart(theme, up_pct))
                // Stats labels
                .child(
                    div().flex().flex_col().gap(px(10.0)).flex_1()
                        .child(stat_row(upload_label, &up_str, theme.status_info, theme))
                        .child(stat_row(download_label, &down_str, theme.status_success, theme)),
                ),
        )
}

/// A simple ring/donut chart visual using divs.
fn ring_chart(theme: &Theme, _up_pct: u32) -> impl IntoElement {
    let ring_border = rgb(theme.border_light);

    div()
        .flex()
        .items_center()
        .justify_center()
        .w(px(72.0))
        .h(px(72.0))
        .rounded(px(36.0))
        .border_2()
        .border_color(ring_border)
        .bg(rgb(theme.surface_variant))
        .child(
            div()
                .text_size(px(11.0))
                .text_color(rgb(theme.text_secondary))
                .child(format!("{:.0}%", _up_pct)),
        )
}

fn stat_row(label: &str, value: &str, color: u32, _theme: &Theme) -> impl IntoElement {
    let label = label.to_string();
    let value = value.to_string();
    div().flex().flex_col().gap(px(2.0))
        .child(
            div().text_size(px(11.0)).text_color(rgb(_theme.text_secondary)).child(label),
        )
        .child(
            div().text_size(px(15.0)).font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(color)).child(value),
        )
}

// ─── Network Detection Card (Right Column Bottom) ─────────

fn network_detection_card(
    theme: &Theme,
    public_ip: Option<&str>,
    _isp: Option<&str>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let ip = public_ip.unwrap_or(strings.network_na);
    let isp = _isp.unwrap_or(strings.network_unknown);
    let title = strings.dashboard_network_detection;
    let ip_label = strings.network_ip_label;
    let isp_label = strings.network_isp_label;

    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .p(px(20.0))
        .rounded(px(CARD_RADIUS))
        .bg(rgb(theme.surface))
        .border_1()
        .border_color(rgb(theme.border_light))
        .child(
            div()
                .text_size(px(14.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme.text_primary))
                .child(title),
        )
        .child(
            div().flex().flex_col().gap(px(6.0))
                .child(info_row(ip_label, ip, theme))
                .child(info_row(isp_label, isp, theme)),
        )
}

fn info_row(label: &str, value: &str, theme: &Theme) -> impl IntoElement {
    let label = label.to_string();
    let value = value.to_string();
    div().flex().items_center().justify_between()
        .child(
            div().text_size(px(12.0)).text_color(rgb(theme.text_secondary)).child(label),
        )
        .child(
            div().text_size(px(13.0)).text_color(rgb(theme.text_primary))
                .font_weight(FontWeight::MEDIUM).child(value),
        )
}

// ─── Core Control Button (Bottom full width) ──────────────

/// Full-width button to start/stop the proxy core.
fn core_control_button(
    theme: &Theme,
    core_running: bool,
    strings: &I18nStrings,
) -> impl IntoElement {
    let (label_text, bg_color) = if core_running {
        (strings.core_stop, theme.status_error)
    } else {
        (strings.core_start, theme.accent)
    };
    let label_text = label_text.to_string();

    div()
        .id("btn-core-control")
        .flex()
        .items_center()
        .justify_center()
        .px(px(24.0))
        .py(px(14.0))
        .rounded(px(CARD_RADIUS))
        .bg(rgb(bg_color))
        .cursor_pointer()
        .on_click(|_: &ClickEvent, _: &mut Window, cx: &mut App| {
            cx.dispatch_action(&ToggleCore);
        })
        .child(
            div()
                .text_size(px(15.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0xffffff))
                .child(label_text),
        )
}

// ─── Outbound Mode Selector ──────────────────────────────

fn outbound_mode_selector(theme: &Theme, current: OutboundMode, strings: &I18nStrings) -> impl IntoElement {
    let modes = [OutboundMode::Rule, OutboundMode::Global, OutboundMode::Direct];

    div()
        .flex()
        .gap(px(4.0))
        .bg(rgb(theme.surface_variant))
        .rounded(px(6.0))
        .p(px(4.0))
        .children(modes.into_iter().map(|mode| {
            let is_active = mode == current;
            let label = strings.outbound_mode_label(mode);
            let bg = if is_active {
                rgb(theme.surface)
            } else {
                rgba(0x00000000)
            };
            let fg = if is_active {
                rgb(theme.accent)
            } else {
                rgb(theme.text_secondary)
            };
            let weight = if is_active {
                FontWeight::SEMIBOLD
            } else {
                FontWeight::NORMAL
            };

            div()
                .id(label)
                .flex()
                .items_center()
                .justify_center()
                .px(px(16.0))
                .py(px(8.0))
                .rounded(px(4.0))
                .bg(bg)
                .text_size(px(13.0))
                .font_weight(weight)
                .text_color(fg)
                .cursor_pointer()
                .hover(|s| {
                    if !is_active {
                        s.bg(rgb(theme.surface_variant))
                    } else {
                        s
                    }
                })
                .on_click(move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
                    cx.dispatch_action(&SetOutboundMode);
                })
                .child(label)
        }))
}

// ─── Helpers ──────────────────────────────────────────────

/// Format speed (bytes/sec) to human readable string.
fn format_speed(bytes_per_sec: u64) -> String {
    if bytes_per_sec == 0 {
        return "0 B/s".into();
    }
    let units = ["B/s", "KB/s", "MB/s", "GB/s"];
    let mut size = bytes_per_sec as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < units.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    format!("{:.1} {}", size, units[unit_idx])
}

/// Format bytes to human readable string.
fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        return "0 B".into();
    }
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < units.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    format!("{:.2} {}", size, units[unit_idx])
}

// ─── Proxies ───────────────────────────────────────────────────

fn proxies_view(theme: &Theme, strings: &I18nStrings) -> impl IntoElement {
    placeholder_section(
        strings.placeholder_proxies_title,
        strings.placeholder_proxies_desc,
        theme,
    )
}

// ─── Profiles ──────────────────────────────────────────────────

fn profiles_view(theme: &Theme, strings: &I18nStrings) -> impl IntoElement {
    placeholder_section(
        strings.placeholder_profiles_title,
        strings.placeholder_profiles_desc,
        theme,
    )
}

// ─── Requests ──────────────────────────────────────────────────

fn requests_view(theme: &Theme, strings: &I18nStrings) -> impl IntoElement {
    placeholder_section(
        strings.placeholder_requests_title,
        strings.placeholder_requests_desc,
        theme,
    )
}

// ─── Connections ───────────────────────────────────────────────

fn connections_view(theme: &Theme, strings: &I18nStrings) -> impl IntoElement {
    placeholder_section(
        strings.placeholder_connections_title,
        strings.placeholder_connections_desc,
        theme,
    )
}

// ─── Resources ─────────────────────────────────────────────────

fn resources_view(theme: &Theme, strings: &I18nStrings) -> impl IntoElement {
    placeholder_section(
        strings.placeholder_resources_title,
        strings.placeholder_resources_desc,
        theme,
    )
}

// ─── Logs ──────────────────────────────────────────────────────

fn logs_view(theme: &Theme, strings: &I18nStrings) -> impl IntoElement {
    placeholder_section(
        strings.placeholder_logs_title,
        strings.placeholder_logs_desc,
        theme,
    )
}

// ─── Tools / Settings ─────────────────────────────────────────

fn tools_view(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let settings_groups = div()
        .flex()
        .flex_col()
        .gap(px(4.0))
        .px(px(24.0))
        .child(language_entry_item(theme, cx, strings))
        .child(theme_toggle_item(theme, cx, strings))
        .child(settings_item(strings.settings_basic_config, strings.settings_basic_config_subtitle, theme))
        .child(settings_item(strings.settings_advanced_config, strings.settings_advanced_config_subtitle, theme))
        .child(settings_item(strings.settings_hotkeys, strings.settings_hotkeys_subtitle, theme))
        .child(settings_item(strings.settings_backup_restore, strings.settings_backup_restore_subtitle, theme))
        .child(settings_item(strings.settings_about, strings.settings_about_subtitle, theme));

    settings_groups
}

// ─── Language Entry (Settings Row → Sub-page) ─────────────────

/// A clickable settings row that navigates to the language selection sub-page.
fn language_entry_item(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let current_lang = cx.global::<crate::i18n::I18nManager>().current_language_id.clone();
    let display = language_display_name(&current_lang);

    div()
        .id("settings-language-entry")
        .flex()
        .items_center()
        .justify_between()
        .px(px(16.0))
        .py(px(14.0))
        .rounded(px(6.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgb(theme.surface)))
        .on_click(cx.listener(|this, _, _, cx| {
            this.tools_sub_page = Some(ToolsSubPage::Language);
            cx.notify();
        }))
        .child(
            div().flex().flex_col().gap(px(2.0)).child(
                div()
                    .text_size(px(14.0))
                    .text_color(rgb(theme.text_primary))
                    .child(strings.settings_language),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(theme.text_secondary))
                    .child(display),
            ),
        )
        .child(
            div()
                .text_size(px(14.0))
                .text_color(rgb(theme.text_disabled))
                .child("\u{203A}"), // ›
        )
}

// ─── Language Sub-page Body ───────────────────────────────────

fn language_sub_page_body(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let current_lang = cx.global::<crate::i18n::I18nManager>().current_language_id.clone();

    div()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .px(px(24.0))
        .child(
            div()
                .px(px(16.0))
                .py(px(8.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme.text_secondary))
                .child(strings.settings_language),
        )
        .child(lang_option_row(theme, "en-US", "English", &current_lang))
        .child(lang_option_row(theme, "zh-CN", "简体中文", &current_lang))
}

/// Back button that returns to the main settings list.
fn back_button(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
) -> impl IntoElement {
    div()
        .id("btn-back-settings")
        .flex()
        .items_center()
        .justify_center()
        .w(px(32.0))
        .h(px(32.0))
        .rounded(px(8.0))
        .cursor_pointer()
        .bg(rgba(0x00000000))
        .hover(|s| s.bg(rgb(theme.surface)))
        .on_click(cx.listener(|this, _, _, cx| {
            this.tools_sub_page = None;
            cx.notify();
        }))
        .child(
            svg()
                .path("icon/arrow-back.svg")
                .size(px(20.0))
                .text_color(rgb(theme.accent)),
        )
}

/// Single language option row for the sub-page.
fn lang_option_row(
    theme: &Theme,
    lang_id: &'static str,
    lang_name: &'static str,
    current_lang: &str,
) -> impl IntoElement {
    let is_active = lang_id == current_lang;
    let id = if lang_id == "en-US" { "lang-en" } else { "lang-zh" };
    let lang_id = lang_id.to_string();
    let lang_name = lang_name.to_string();

    div()
        .id(id)
        .flex()
        .items_center()
        .gap(px(10.0))
        .px(px(16.0))
        .py(px(12.0))
        .rounded(px(6.0))
        .cursor_pointer()
        .hover(|s| {
            s.bg(rgb(theme.surface))
        })
        .on_click(move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
            crate::i18n::I18nManager::init_with_language_id(cx, &lang_id);
            save_language_id(&lang_id);
            cx.refresh_windows();
        })
        .child(
            div()
                .w(px(18.0))
                .h(px(18.0))
                .rounded(px(9.0))
                .border_1()
                .border_color(if is_active {
                    Hsla::from(rgb(theme.accent))
                } else {
                    Hsla::from(rgb(theme.border))
                })
                .bg(if is_active {
                    Hsla::from(rgb(theme.accent))
                } else {
                    Hsla::from(rgba(0x00000000))
                })
                .flex()
                .items_center()
                .justify_center()
                .when(is_active, |s| {
                    s.child(div().w(px(6.0)).h(px(6.0)).rounded(px(3.0)).bg(rgb(0xffffff)))
                }),
        )
        .child(
            div()
                .text_size(px(14.0))
                .text_color(if is_active {
                    Hsla::from(rgb(theme.accent))
                } else {
                    Hsla::from(rgb(theme.text_primary))
                })
                .font_weight(if is_active {
                    FontWeight::SEMIBOLD
                } else {
                    FontWeight::NORMAL
                })
                .child(lang_name),
        )
}

// ─── Theme Toggle (interactive) ────────────────────────────────

fn theme_toggle_item(
    theme: &Theme,
    _cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let accent_color = rgb(theme.accent);
    let title = strings.settings_theme;
    let subtitle = strings.settings_theme_subtitle;
    div()
        .id("theme-toggle")
        .flex()
        .items_center()
        .justify_between()
        .px(px(16.0))
        .py(px(14.0))
        .rounded(px(6.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgb(theme.surface)))
        .on_click(|_: &ClickEvent, _: &mut Window, cx: &mut App| {
            cx.update_global::<AppState, _>(|state, _cx| {
                state.theme_mode = state.theme_mode.toggle();
                save_theme_mode(state.theme_mode);
            });
            cx.refresh_windows();
        })
        .child(
            div().flex().flex_col().gap(px(2.0)).child(
                div()
                    .text_size(px(14.0))
                    .text_color(accent_color)
                    .child(title),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(theme.text_secondary))
                    .child(subtitle),
            ),
        )
        .child(
            div()
                .text_size(px(14.0))
                .text_color(rgb(theme.text_disabled))
                .child("\u{203A}"), // ›
        )
}

// ─── Shared Helper Components ──────────────────────────────────

/// A reusable placeholder section with title and description.
fn placeholder_section(title: &str, description: &str, theme: &Theme) -> impl IntoElement {
    let title = title.to_string();
    let description = description.to_string();
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .py(px(60.0))
        .px(px(24.0))
        .gap(px(8.0))
        .child(
            div()
                .text_size(px(40.0))
                .text_color(rgb(theme.text_disabled))
                .child("\u{1F527}"),
        )
        .child(
            div()
                .text_size(px(16.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme.text_secondary))
                .child(title),
        )
        .child(
            div()
                .text_size(px(13.0))
                .text_color(rgb(theme.text_disabled))
                .child(description),
        )
}

/// A static settings list item with title and subtitle.
fn settings_item(title: &str, subtitle: &str, theme: &Theme) -> impl IntoElement {
    let title = title.to_string();
    let subtitle = subtitle.to_string();
    div()
        .flex()
        .items_center()
        .justify_between()
        .px(px(16.0))
        .py(px(14.0))
        .rounded(px(6.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgb(theme.surface)))
        .child(
            div().flex().flex_col().gap(px(2.0)).child(
                div()
                    .text_size(px(14.0))
                    .text_color(rgb(theme.text_primary))
                    .child(title),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(theme.text_secondary))
                    .child(subtitle),
            ),
        )
        .child(
            div()
                .text_size(px(14.0))
                .text_color(rgb(theme.text_disabled))
                .child("\u{203A}"), // ›
        )
}
