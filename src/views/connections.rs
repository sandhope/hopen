/// Connections view — real-time active connection list, detail panel, search, close.
///
/// Columns: Time | Source | Destination | Proxy Chain | Rule | Speed
/// Detail: upload / download / duration / source / dest / host / network / chain
/// Actions: close single connection, close all, search filter.

use gpui::prelude::*;
use gpui::*;

use crate::i18n::I18nStrings;
use crate::theme::{Theme, CARD_RADIUS};

// ─── Data types ────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct MockConnection {
    pub time: &'static str,
    pub source_ip: &'static str,
    pub source_port: u16,
    pub dest_ip: &'static str,
    pub dest_port: u16,
    pub host: &'static str,
    pub network: ConnectionNetwork,
    pub proxy_chain: &'static str,
    pub rule: &'static str,
    pub rule_type: ConnectionRuleType,
    pub upload_bytes: u64,
    pub download_bytes: u64,
    pub duration_secs: u64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConnectionNetwork {
    Tcp,
    Udp,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ConnectionRuleType {
    Domain,
    IpCidr,
    #[allow(dead_code)]
    Geoip,
    Geosite,
    Match,
    Direct,
}

// ─── Mock data ─────────────────────────────────────────────────────────

fn build_mock_connections() -> Vec<MockConnection> {
    vec![
        MockConnection {
            time: "14:32:05",
            source_ip: "192.168.1.100",
            source_port: 52341,
            dest_ip: "142.250.80.46",
            dest_port: 443,
            host: "www.google.com",
            network: ConnectionNetwork::Tcp,
            proxy_chain: "GLOBAL >> HK 01",
            rule: "google.com",
            rule_type: ConnectionRuleType::Domain,
            upload_bytes: 12_480,
            download_bytes: 245_760,
            duration_secs: 120,
        },
        MockConnection {
            time: "14:32:05",
            source_ip: "192.168.1.100",
            source_port: 52342,
            dest_ip: "140.82.121.3",
            dest_port: 443,
            host: "api.github.com",
            network: ConnectionNetwork::Tcp,
            proxy_chain: "GLOBAL >> SG 01",
            rule: "geosite:github",
            rule_type: ConnectionRuleType::Geosite,
            upload_bytes: 4_096,
            download_bytes: 1_048_576,
            duration_secs: 85,
        },
        MockConnection {
            time: "14:32:04",
            source_ip: "192.168.1.100",
            source_port: 52340,
            dest_ip: "110.242.68.66",
            dest_port: 443,
            host: "www.baidu.com",
            network: ConnectionNetwork::Tcp,
            proxy_chain: "DIRECT",
            rule: "DIRECT",
            rule_type: ConnectionRuleType::Direct,
            upload_bytes: 2_048,
            download_bytes: 96_256,
            duration_secs: 45,
        },
        MockConnection {
            time: "14:32:03",
            source_ip: "192.168.1.100",
            source_port: 52339,
            dest_ip: "142.250.80.46",
            dest_port: 443,
            host: "www.youtube.com",
            network: ConnectionNetwork::Tcp,
            proxy_chain: "Streaming >> US 01",
            rule: "youtube.com",
            rule_type: ConnectionRuleType::Domain,
            upload_bytes: 8_192,
            download_bytes: 53_477_376,
            duration_secs: 340,
        },
        MockConnection {
            time: "14:32:02",
            source_ip: "192.168.1.100",
            source_port: 52338,
            dest_ip: "104.244.42.1",
            dest_port: 443,
            host: "twitter.com",
            network: ConnectionNetwork::Tcp,
            proxy_chain: "GLOBAL >> HK 01",
            rule: "twitter.com",
            rule_type: ConnectionRuleType::Domain,
            upload_bytes: 5_120,
            download_bytes: 327_680,
            duration_secs: 95,
        },
        MockConnection {
            time: "14:32:01",
            source_ip: "127.0.0.1",
            source_port: 52337,
            dest_ip: "127.0.0.1",
            dest_port: 9090,
            host: "localhost:9090",
            network: ConnectionNetwork::Tcp,
            proxy_chain: "DIRECT",
            rule: "DIRECT",
            rule_type: ConnectionRuleType::Direct,
            upload_bytes: 512,
            download_bytes: 2_048,
            duration_secs: 10,
        },
        MockConnection {
            time: "14:31:58",
            source_ip: "192.168.1.100",
            source_port: 52336,
            dest_ip: "151.101.1.140",
            dest_port: 443,
            host: "www.reddit.com",
            network: ConnectionNetwork::Tcp,
            proxy_chain: "GLOBAL >> JP 01",
            rule: "geosite:reddit",
            rule_type: ConnectionRuleType::Geosite,
            upload_bytes: 6_144,
            download_bytes: 102_400,
            duration_secs: 72,
        },
        MockConnection {
            time: "14:31:55",
            source_ip: "192.168.1.100",
            source_port: 52335,
            dest_ip: "142.250.80.46",
            dest_port: 443,
            host: "clients3.google.com",
            network: ConnectionNetwork::Tcp,
            proxy_chain: "GLOBAL >> HK 01",
            rule: "google.com",
            rule_type: ConnectionRuleType::Domain,
            upload_bytes: 1_024,
            download_bytes: 0,
            duration_secs: 3,
        },
        MockConnection {
            time: "14:31:50",
            source_ip: "192.168.1.100",
            source_port: 52334,
            dest_ip: "104.18.37.228",
            dest_port: 443,
            host: "api.openai.com",
            network: ConnectionNetwork::Tcp,
            proxy_chain: "GLOBAL >> US 01",
            rule: "openai.com",
            rule_type: ConnectionRuleType::Domain,
            upload_bytes: 36_864,
            download_bytes: 4_096,
            duration_secs: 210,
        },
        MockConnection {
            time: "14:31:48",
            source_ip: "192.168.1.100",
            source_port: 52333,
            dest_ip: "23.212.6.142",
            dest_port: 443,
            host: "akamaihd.net",
            network: ConnectionNetwork::Udp,
            proxy_chain: "Streaming >> US 01",
            rule: "ipcidr:23.212.0.0/16",
            rule_type: ConnectionRuleType::IpCidr,
            upload_bytes: 1_024,
            download_bytes: 167_772_160,
            duration_secs: 540,
        },
        MockConnection {
            time: "14:31:45",
            source_ip: "192.168.1.100",
            source_port: 60001,
            dest_ip: "119.28.120.75",
            dest_port: 53,
            host: "dns.google",
            network: ConnectionNetwork::Udp,
            proxy_chain: "DNS",
            rule: "MATCH",
            rule_type: ConnectionRuleType::Match,
            upload_bytes: 128,
            download_bytes: 256,
            duration_secs: 1,
        },
        MockConnection {
            time: "14:31:40",
            source_ip: "192.168.1.100",
            source_port: 52332,
            dest_ip: "13.107.42.14",
            dest_port: 443,
            host: "outlook.office365.com",
            network: ConnectionNetwork::Tcp,
            proxy_chain: "GLOBAL >> HK 01",
            rule: "geosite:microsoft",
            rule_type: ConnectionRuleType::Geosite,
            upload_bytes: 16_384,
            download_bytes: 512_000,
            duration_secs: 180,
        },
    ]
}

// ─── Formatters ────────────────────────────────────────────────────────

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_idx = 0;
    while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
        size /= 1024.0;
        unit_idx += 1;
    }
    if unit_idx == 0 {
        format!("{} B", size as u64)
    } else {
        format!("{:.1} {}", size, UNITS[unit_idx])
    }
}

fn format_speed(bytes: u64) -> String {
    if bytes >= 1_048_576 {
        format!("{:.1} MB/s", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1_024 {
        format!("{:.1} KB/s", bytes as f64 / 1_024.0)
    } else {
        format!("{} B/s", bytes)
    }
}

fn format_duration(secs: u64) -> String {
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    }
}

// ─── Main view ─────────────────────────────────────────────────────────

pub(super) fn connections_view(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    search_text: &str,
    selected_index: Option<usize>,
) -> impl IntoElement + use<> {
    let connections = build_mock_connections();

    // Filter by search text
    let search_text = search_text.to_string();
    let ft = search_text.to_lowercase();
    let filtered: Vec<(usize, MockConnection)> = connections
        .iter()
        .enumerate()
        .filter(|(_, c)| {
            ft.is_empty()
                || c.host.to_lowercase().contains(&ft)
                || c.source_ip.to_lowercase().contains(&ft)
                || c.dest_ip.to_lowercase().contains(&ft)
                || c.proxy_chain.to_lowercase().contains(&ft)
                || c.rule.to_lowercase().contains(&ft)
        })
        .map(|(i, c)| (i, c.clone()))
        .collect();

    let selected = selected_index
        .and_then(|si| connections.get(si))
        .cloned();

    let has_search = !search_text.is_empty();

    div()
        .flex()
        .flex_col()
        .size_full()
        .gap(px(8.0))
        .px(px(24.0))
        .py(px(8.0))
        // ── Toolbar ────────────────────────────────
        .child(render_toolbar(theme, cx, strings, &search_text, has_search))
        // ── Search indicator ──────────────────────
        .when(has_search, |s| {
            let ft = search_text.clone();
            s.child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(8.0))
                    .py(px(4.0))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(theme.text_secondary))
                            .child(format!("Filter: \"{}\" — {} results", ft, filtered.len())),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(theme.accent))
                            .cursor_pointer()
                            .on_any_mouse_down({
                                let entity = cx.entity();
                                move |_: &MouseDownEvent, _window, app| {
                                    entity.update(app, |this, _| {
                                        this.connections_search_text.clear();
                                        this.connections_selected_index = None;
                                    });
                                }
                            })
                            .child("Clear"),
                    ),
            )
        })
        // ── Connection list table ─────────────────
        .child(render_connection_table(theme, cx, strings, &filtered, selected_index))
        // ── Detail panel (when selected) ─────────
        .when_some(selected, |s, conn| {
            s.child(render_detail_panel(theme, cx, strings, &conn))
        })
        .child(div().h(px(16.0)))
}

// ─── Toolbar ───────────────────────────────────────────────────────────

fn render_toolbar(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    search_text: &str,
    has_search: bool,
) -> impl IntoElement + use<> {
    let placeholder = strings.connections_search_placeholder.to_string();
    let display = search_text.to_string();
    let close_all_label = strings.connections_close_all.to_string();

    div()
        .flex()
        .items_center()
        .gap(px(8.0))
        .child(
            // Search input
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .flex_1()
                .px(px(12.0))
                .py(px(8.0))
                .rounded(px(CARD_RADIUS))
                .bg(rgb(theme.surface))
                .border_1()
                .border_color(rgb(theme.border_light))
                .cursor_pointer()
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(rgb(theme.text_disabled))
                        .flex_shrink_0()
                        .child("\u{1F50D}"),
                )
                .child(
                    div()
                        .flex_1()
                        .text_size(px(13.0))
                        .when(has_search, |s| {
                            s.text_color(rgb(theme.text_primary)).child(display)
                        })
                        .when(!has_search, |s| {
                            s.text_color(rgb(theme.text_disabled)).child(placeholder)
                        }),
                ),
        )
        // Close all button
        .child(
            div()
                .flex()
                .items_center()
                .px(px(14.0))
                .py(px(8.0))
                .rounded(px(CARD_RADIUS))
                .bg(rgb(theme.status_error))
                .cursor_pointer()
                .hover(|s| s.opacity(0.85))
                .on_any_mouse_down({
                    let entity = cx.entity();
                    move |_: &MouseDownEvent, _window, app| {
                        entity.update(app, |this, _| {
                            this.connections_search_text.clear();
                            this.connections_selected_index = None;
                        });
                    }
                })
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0xffffff))
                        .child(close_all_label),
                ),
        )
}

// ─── Connection table ──────────────────────────────────────────────────

fn render_connection_table(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    filtered: &[(usize, MockConnection)],
    selected_index: Option<usize>,
) -> impl IntoElement + use<> {
    let empty = filtered.is_empty();

    div()
        .flex()
        .flex_col()
        .flex_1()
        .rounded(px(CARD_RADIUS))
        .bg(rgb(theme.surface))
        .border_1()
        .border_color(rgb(theme.border_light))
        .overflow_hidden()
        // ── Table header ──────────────────────────
        .child(
            div()
                .flex()
                .items_center()
                .px(px(12.0))
                .py(px(10.0))
                .bg(rgb(theme.surface_variant))
                .border_b_1()
                .border_color(rgb(theme.border_light))
                .text_size(px(11.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme.text_secondary))
                .child(div().w(px(60.0)).child(strings.connections_col_time))
                .child(div().flex_1().child(strings.connections_col_source))
                .child(div().flex_1().child(strings.connections_col_dest))
                .child(div().flex_1().child(strings.connections_col_proxy))
                .child(div().flex_1().child(strings.connections_col_rule))
                .child(div().w(px(72.0)).text_align(TextAlign::Right).child(strings.connections_col_speed)),
        )
        // ── Empty state ───────────────────────────
        .when(empty, |s| {
            s.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .py(px(48.0))
                    .text_size(px(13.0))
                    .text_color(rgb(theme.text_disabled))
                    .child(strings.connections_empty),
            )
        })
        // ── Table body (scrollable) ───────────────
        .when(!empty, |s| {
            s.child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_hidden()
                    .children(
                        filtered
                            .iter()
                            .map(|(orig_idx, conn)| {
                                render_connection_row(theme, cx, strings, *orig_idx, conn, selected_index)
                            }),
                    ),
            )
        })
}

// ─── Single connection row ─────────────────────────────────────────────

fn render_connection_row(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    _strings: &I18nStrings,
    orig_index: usize,
    conn: &MockConnection,
    selected_index: Option<usize>,
) -> impl IntoElement + use<> {
    let is_selected = selected_index == Some(orig_index);
    let time_str = conn.time.to_string();
    let source = format!("{}:{}", conn.source_ip, conn.source_port);
    let dest = format!("{}:{}", conn.dest_ip, conn.dest_port);
    let host = conn.host.to_string();
    let proxy = conn.proxy_chain.to_string();
    let rule = conn.rule.to_string();

    let rule_color = match conn.rule_type {
        ConnectionRuleType::Direct => rgb(theme.status_success),
        ConnectionRuleType::Domain => rgb(theme.accent),
        ConnectionRuleType::IpCidr => rgb(theme.status_info),
        ConnectionRuleType::Geoip => rgb(theme.status_warning),
        ConnectionRuleType::Geosite => rgb(theme.status_info),
        ConnectionRuleType::Match => rgb(theme.text_secondary),
    };

    let speed_bytes = if conn.duration_secs > 0 {
        conn.download_bytes / conn.duration_secs
    } else {
        conn.download_bytes
    };
    let speed_text = format_speed(speed_bytes);

    let network_label = match conn.network {
        ConnectionNetwork::Tcp => "TCP",
        ConnectionNetwork::Udp => "UDP",
    };
    let network_color = match conn.network {
        ConnectionNetwork::Tcp => rgb(theme.status_info),
        ConnectionNetwork::Udp => rgb(theme.status_warning),
    };

    let row_bg = if is_selected {
        rgb(theme.accent_muted)
    } else {
        rgba(0x00000000)
    };

    div()
        .id(("connection-row", orig_index))
        .flex()
        .items_center()
        .px(px(12.0))
        .py(px(7.0))
        .bg(row_bg)
        .border_b_1()
        .border_color(rgb(theme.border_light))
        .cursor_pointer()
        .hover(|s| {
            if !is_selected {
                s.bg(rgb(theme.surface_variant))
            } else {
                s
            }
        })
        .on_any_mouse_down({
            let entity = cx.entity();
            move |_: &MouseDownEvent, _window, app| {
                entity.update(app, |this, _| {
                    this.connections_selected_index = Some(orig_index);
                });
            }
        })
        // Time
        .child(
            div()
                .w(px(60.0))
                .text_size(px(11.0))
                .text_color(rgb(theme.text_secondary))
                .child(time_str),
        )
        // Source (w/ host tooltip)
        .child(
            div()
                .flex_1()
                .flex()
                .flex_col()
                .overflow_hidden()
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(rgb(theme.text_primary))
                        .overflow_hidden()
                        .text_ellipsis()
                        .child(source),
                )
                .child(
                    div()
                        .text_size(px(10.0))
                        .text_color(rgb(theme.text_disabled))
                        .overflow_hidden()
                        .text_ellipsis()
                        .child(host),
                ),
        )
        // Destination
        .child(
            div()
                .flex_1()
                .text_size(px(11.0))
                .text_color(rgb(theme.text_secondary))
                .overflow_hidden()
                .text_ellipsis()
                .child(dest),
        )
        // Proxy chain + network badge
        .child(
            div()
                .flex_1()
                .flex()
                .items_center()
                .gap(px(6.0))
                .child(
                    div()
                        .px(px(5.0))
                        .py(px(2.0))
                        .rounded(px(3.0))
                        .text_size(px(9.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(network_color)
                        .bg({
                            let mut c = network_color;
                            c.a = 0.15;
                            c
                        })
                        .child(network_label),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(rgb(theme.text_secondary))
                        .overflow_hidden()
                        .text_ellipsis()
                        .child(proxy),
                ),
        )
        // Rule
        .child(
            div()
                .flex_1()
                .text_size(px(11.0))
                .text_color(rule_color)
                .overflow_hidden()
                .text_ellipsis()
                .child(rule),
        )
        // Speed + close button
        .child(
            div()
                .w(px(72.0))
                .flex()
                .items_center()
                .gap(px(4.0))
                .justify_end()
                .child(
                    div()
                        .text_size(px(11.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(theme.text_secondary))
                        .text_align(TextAlign::Right)
                        .child(speed_text),
                )
                .child(
                    div()
                        .text_size(px(10.0))
                        .text_color(rgb(theme.status_error))
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.7))
                        .on_any_mouse_down({
                            let entity = cx.entity();
                            move |_: &MouseDownEvent, _window, app| {
                                entity.update(app, |this, _| {
                                    this.connections_selected_index = None;
                                });
                            }
                        })
                        .child("\u{2716}"),
                ),
        )
}

// ─── Detail panel ──────────────────────────────────────────────────────

fn render_detail_panel(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    conn: &MockConnection,
) -> impl IntoElement + use<> {
    let upload = format_bytes(conn.upload_bytes);
    let download = format_bytes(conn.download_bytes);
    let duration = format_duration(conn.duration_secs);
    let source = format!("{}:{}", conn.source_ip, conn.source_port);
    let dest = format!("{}:{}", conn.dest_ip, conn.dest_port);
    let host = conn.host.to_string();
    let network_label = match conn.network {
        ConnectionNetwork::Tcp => "TCP",
        ConnectionNetwork::Udp => "UDP",
    };
    let close_label = strings.connections_close_single.to_string();
    let rule_color = match conn.rule_type {
        ConnectionRuleType::Direct => rgb(theme.status_success),
        ConnectionRuleType::Domain => rgb(theme.accent),
        ConnectionRuleType::IpCidr => rgb(theme.status_info),
        ConnectionRuleType::Geoip => rgb(theme.status_warning),
        ConnectionRuleType::Geosite => rgb(theme.status_info),
        ConnectionRuleType::Match => rgb(theme.text_secondary),
    };

    div()
        .flex()
        .flex_col()
        .rounded(px(CARD_RADIUS))
        .bg(rgb(theme.surface))
        .border_1()
        .border_color(rgb(theme.border_light))
        .overflow_hidden()
        // Title bar
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(10.0))
                .px(px(16.0))
                .py(px(10.0))
                .bg(rgb(theme.surface_variant))
                .border_b_1()
                .border_color(rgb(theme.border_light))
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(theme.text_primary))
                        .child(strings.connections_detail_title),
                )
                .child(
                    div().flex_1().text_size(px(12.0)).text_color(rgb(theme.text_primary)).overflow_hidden().text_ellipsis().child(host),
                )
                // Close button in title
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(4.0))
                        .px(px(10.0))
                        .py(px(4.0))
                        .rounded(px(4.0))
                        .text_size(px(11.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0xffffff))
                        .bg(rgb(theme.status_error))
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.85))
                        .on_any_mouse_down({
                            let entity = cx.entity();
                            move |_: &MouseDownEvent, _window, app| {
                                entity.update(app, |this, _| {
                                    this.connections_selected_index = None;
                                });
                            }
                        })
                        .child(close_label),
                ),
        )
        // Content grid
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .p(px(16.0))
                // Traffic: Upload / Download
                .child(
                    div().flex().gap(px(24.0))
                        .child(detail_row(theme, strings.connections_detail_upload, &upload))
                        .child(detail_row(theme, strings.connections_detail_download, &download))
                        .child(detail_row(theme, strings.connections_detail_duration, &duration)),
                )
                // Source / Destination
                .child(
                    div().flex().gap(px(24.0))
                        .child(detail_row(theme, strings.connections_detail_source, &source))
                        .child(detail_row(theme, strings.connections_detail_destination, &dest)),
                )
                // Host / Network / Chain
                .child(
                    div().flex().gap(px(24.0))
                        .child(detail_row(theme, strings.connections_detail_host, conn.host))
                        .child(detail_row(theme, strings.connections_detail_network, network_label))
                        .child(detail_row(theme, strings.connections_detail_chain, conn.proxy_chain)),
                )
                // Proxy / Rule
                .child(
                    div().flex().gap(px(24.0))
                        .child(detail_row(theme, strings.connections_detail_proxy, conn.proxy_chain))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(2.0))
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(rgb(theme.text_disabled))
                                        .child(strings.connections_detail_rule),
                                )
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(rule_color)
                                        .child(conn.rule),
                                ),
                        ),
                ),
        )
}

fn detail_row(theme: &Theme, label: &str, value: &str) -> impl IntoElement {
    let label = label.to_string();
    let value = value.to_string();
    div()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .min_w(px(100.0))
        .child(
            div()
                .text_size(px(11.0))
                .text_color(rgb(theme.text_disabled))
                .child(label),
        )
        .child(
            div()
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(theme.text_primary))
                .overflow_hidden()
                .text_ellipsis()
                .child(value),
        )
}
