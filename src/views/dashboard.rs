/// Dashboard view and all its sub-card components.
///
/// Layout (matching FlClash default widgets):
/// ┌────────────────────────────────────────┐
/// │  Network Speed (full width)            │
/// ├──────────────────┬─────────────────────┤
/// │ Proxy Control    │ Traffic Usage       │
/// │  System Proxy    │  (donut + total)    │
/// │  TUN Mode        │                     │
/// │  Outbound Mode   │ Network Detection   │
/// │                  │  (IP / ISP)         │
/// │ LAN IP           │                     │
/// ├──────────────────┴─────────────────────┤
/// │           Core Control Button          │
/// └────────────────────────────────────────┘

use gpui::*;

use crate::components::charts::{self, DonutChartData};
use crate::i18n::I18nStrings;
use crate::theme::{Theme, CARD_RADIUS};
use crate::{
    AppState, OutboundMode,
    SetOutboundMode, ToggleCore, ToggleSystemProxy, ToggleTunMode,
};

// ─── Dashboard ─────────────────────────────────────────────────

pub(super) fn dashboard_view(
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
                // Wave visual for real-time speed indication
                .child(
                    div().flex().flex_col().gap(px(4.0)).flex_1().justify_end()
                        .child(charts::wave_view(theme, theme.status_info, 12.0))
                        .child(charts::wave_view(theme, theme.status_success, 8.0)),
                ),
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

/// Card with donut chart and upload/download totals.
fn traffic_stats_card(
    theme: &Theme,
    upload_total: u64,
    download_total: u64,
    strings: &I18nStrings,
) -> impl IntoElement {
    let title = strings.dashboard_traffic_usage;
    let upload_label = strings.dashboard_upload;
    let download_label = strings.dashboard_download;

    let chart_data = vec![
        DonutChartData {
            value: upload_total as f32,
            color: theme.status_info,
            label: upload_label.to_string(),
        },
        DonutChartData {
            value: download_total as f32,
            color: theme.status_success,
            label: download_label.to_string(),
        },
    ];

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
        .child(charts::donut_chart(&chart_data, theme))
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

// ─── Formatting Helpers ───────────────────────────────────

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
