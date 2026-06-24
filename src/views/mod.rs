/// Page view skeletons for the Hopen GPUI client.
///
/// Each function returns a view for the corresponding page.
/// Theme-aware: all colours come from the `Theme` parameter.

use gpui::*;

use crate::navigation::Page;
use crate::theme::{Theme, CARD_RADIUS};
use crate::{
    save_theme_mode, AppState, OutboundMode, SetOutboundMode, ToggleCore, ToggleSystemProxy,
    ToggleTunMode,
};

/// Route to the correct page view based on the current navigation state.
///
/// `cx` is the AppView context, needed for interactive elements (e.g. theme toggle).
pub fn render_page(
    page: Page,
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
) -> impl IntoElement + use<> {
    let title = page.title();

    let header = div()
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
        );

    let content = match page {
        Page::Dashboard => div().child(dashboard_view(theme, cx)),
        Page::Proxies => div().child(proxies_view(theme)),
        Page::Profiles => div().child(profiles_view(theme)),
        Page::Requests => div().child(requests_view(theme)),
        Page::Connections => div().child(connections_view(theme)),
        Page::Resources => div().child(resources_view(theme)),
        Page::Logs => div().child(logs_view(theme)),
        Page::Tools => div().child(tools_view(theme, cx)),
    };

    div()
        .flex()
        .flex_col()
        .size_full()
        .overflow_y_hidden()
        .child(header)
        .child(content)
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
                            theme, system_proxy, tun_mode, outbound_mode,
                        ))
                        // LAN IP card
                        .child(lan_ip_card(theme, lan_ip.as_deref())),
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
                            theme, upload_total, download_total,
                        ))
                        // Network detection card
                        .child(network_detection_card(
                            theme, public_ip.as_deref(), isp.as_deref(),
                        )),
                ),
        )
        // ── Bottom: Core Control Button ────────────────────
        .child(core_control_button(theme, core_running))
}

// ─── Network Speed Card ────────────────────────────────────

/// Full-width card showing real-time upload/download speed with a mini graph.
fn network_speed_card(
    theme: &Theme,
    upload_speed: u64,
    download_speed: u64,
    core_running: bool,
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
                .child("Network Speed"),
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
                                        .child("Upload"),
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
                                        .child("Download"),
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
) -> impl IntoElement {
    // System Proxy switch visuals (eager, used in closure)
    let sp_active = system_proxy;
    let sp_state_text = if sp_active { "ON" } else { "OFF" };
    let sp_state_color = if sp_active { rgb(theme.status_success) } else { rgb(theme.text_disabled) };
    let sp_track = if sp_active { rgb(theme.accent) } else { rgb(theme.border) };

    // TUN Mode switch visuals
    let tun_active = tun_mode;
    let tun_state_text = if tun_active { "ON" } else { "OFF" };
    let tun_state_color = if tun_active { rgb(theme.status_success) } else { rgb(theme.text_disabled) };
    let tun_track = if tun_active { rgb(theme.accent) } else { rgb(theme.border) };

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
                .child("Proxy Control"),
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
                .child(div().text_size(px(13.0)).text_color(rgb(theme.text_primary)).child("System Proxy"))
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
                .child(div().text_size(px(13.0)).text_color(rgb(theme.text_primary)).child("TUN Mode"))
                .child(switch_pill(tun_state_text, tun_state_color, tun_track)),
        )
        // Divider
        .child(div().h(px(1.0)).bg(rgb(theme.border_light)))
        // Outbound Mode section
        .child(
            div().flex().flex_col().gap(px(8.0))
                .child(
                    div().text_size(px(13.0)).text_color(rgb(theme.text_secondary))
                        .child("Outbound Mode"),
                )
                .child(outbound_mode_selector(theme, outbound_mode)),
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

fn lan_ip_card(theme: &Theme, lan_ip: Option<&str>) -> impl IntoElement {
    let ip = lan_ip.unwrap_or("Detecting...").to_string();

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
                .child("LAN IP"),
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
) -> impl IntoElement {
    let up_str = format_bytes(upload_total);
    let down_str = format_bytes(download_total);
    let total = upload_total + download_total;
    let up_pct = if total > 0 {
        (upload_total as f64 / total as f64 * 100.0) as u32
    } else {
        50
    };

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
                .child("Traffic Usage"),
        )
        .child(
            div().flex().gap(px(20.0)).items_center()
                // Ring chart visual
                .child(ring_chart(theme, up_pct))
                // Stats labels
                .child(
                    div().flex().flex_col().gap(px(10.0)).flex_1()
                        .child(stat_row("Upload", &up_str, theme.status_info, theme))
                        .child(stat_row("Download", &down_str, theme.status_success, theme)),
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
) -> impl IntoElement {
    let ip = public_ip.unwrap_or("N/A");
    let isp = _isp.unwrap_or("Unknown");

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
                .child("Network Detection"),
        )
        .child(
            div().flex().flex_col().gap(px(6.0))
                .child(info_row("IP", ip, theme))
                .child(info_row("ISP", isp, theme)),
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
) -> impl IntoElement {
    let (label_text, bg_color) = if core_running {
        ("Stop Core", theme.status_error)
    } else {
        ("Start Core", theme.accent)
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

fn outbound_mode_selector(theme: &Theme, current: OutboundMode) -> impl IntoElement {
    let modes = [OutboundMode::Rule, OutboundMode::Global, OutboundMode::Direct];

    div()
        .flex()
        .gap(px(4.0))
        .bg(rgb(theme.surface_variant))
        .rounded(px(6.0))
        .p(px(4.0))
        .children(modes.into_iter().map(|mode| {
            let is_active = mode == current;
            let label = mode.label().to_string();
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
                .id(mode.label())
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

fn proxies_view(theme: &Theme) -> impl IntoElement {
    placeholder_section(
        "Proxy Groups",
        "Proxy groups will appear here when the core is connected.",
        theme,
    )
}

// ─── Profiles ──────────────────────────────────────────────────

fn profiles_view(theme: &Theme) -> impl IntoElement {
    placeholder_section(
        "Profiles",
        "Import or add subscription profiles to get started.",
        theme,
    )
}

// ─── Requests ──────────────────────────────────────────────────

fn requests_view(theme: &Theme) -> impl IntoElement {
    placeholder_section(
        "Request Timeline",
        "Real-time request tracking will appear here.",
        theme,
    )
}

// ─── Connections ───────────────────────────────────────────────

fn connections_view(theme: &Theme) -> impl IntoElement {
    placeholder_section(
        "Active Connections",
        "Active connections will be listed here.",
        theme,
    )
}

// ─── Resources ─────────────────────────────────────────────────

fn resources_view(theme: &Theme) -> impl IntoElement {
    placeholder_section(
        "Resources",
        "GeoIP, GeoSite, and other resource files will be managed here.",
        theme,
    )
}

// ─── Logs ──────────────────────────────────────────────────────

fn logs_view(theme: &Theme) -> impl IntoElement {
    placeholder_section(
        "Core Logs",
        "Logs from the proxy core will stream here.",
        theme,
    )
}

// ─── Tools / Settings ─────────────────────────────────────────

fn tools_view(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
) -> impl IntoElement {
    let settings_groups = div()
        .flex()
        .flex_col()
        .gap(px(4.0))
        .px(px(24.0))
        .child(theme_toggle_item(theme, cx))
        .child(settings_item("Basic Config", "Port, log level, mode", theme))
        .child(settings_item(
            "Advanced Config",
            "DNS, TUN, rules",
            theme,
        ))
        .child(settings_item("Hotkeys", "Keyboard shortcuts", theme))
        .child(settings_item("Backup & Restore", "WebDAV sync", theme))
        .child(settings_item("About", "Version and license info", theme));

    settings_groups
}

// ─── Theme Toggle (interactive) ────────────────────────────────

fn theme_toggle_item(
    theme: &Theme,
    _cx: &mut Context<crate::app::AppView>,
) -> impl IntoElement {
    let accent_color = rgb(theme.accent);
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
                    .child("Theme"),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(theme.text_secondary))
                    .child("Dark / Light — tap to switch appearance"),
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
