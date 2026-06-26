/// Logs view — real-time log stream, level filter, search, copy, export, clear.

use crate::components::text_input::TextInput;
use gpui::prelude::*;
use gpui::*;

use crate::i18n::I18nStrings;
use crate::theme::{Theme, CARD_RADIUS};

// ─── Data types ────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LogLevelFilter {
    All,
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Clone, PartialEq, Eq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Clone)]
pub struct LogEntry {
    pub time: &'static str,
    pub level: LogLevel,
    pub message: &'static str,
}

// ─── Level helpers ─────────────────────────────────────────────────────

impl LogLevel {
    #[allow(dead_code)]
    pub fn label(&self, strings: &I18nStrings) -> &'static str {
        match self {
            LogLevel::Debug => strings.logs_filter_debug,
            LogLevel::Info => strings.logs_filter_info,
            LogLevel::Warning => strings.logs_filter_warning,
            LogLevel::Error => strings.logs_filter_error,
        }
    }

    pub fn tag(&self, strings: &I18nStrings) -> &'static str {
        // short uppercase tags used in the table
        match self {
            LogLevel::Debug => strings.logs_filter_debug,
            LogLevel::Info => strings.logs_filter_info,
            LogLevel::Warning => strings.logs_filter_warning,
            LogLevel::Error => strings.logs_filter_error,
        }
    }

    pub fn color(&self, theme: &Theme) -> Rgba {
        match self {
            LogLevel::Debug => rgb(theme.text_disabled),
            LogLevel::Info => rgb(theme.status_success),
            LogLevel::Warning => rgb(theme.status_warning),
            LogLevel::Error => rgb(theme.status_error),
        }
    }

    pub fn bg_color(&self, _theme: &Theme) -> Rgba {
        match self {
            LogLevel::Debug => rgba(0x80808020),
            LogLevel::Info => rgba(0x00c85320),
            LogLevel::Warning => rgba(0xff910020),
            LogLevel::Error => rgba(0xff174418),
        }
    }
}

impl LogLevelFilter {
    pub fn label(&self, strings: &I18nStrings) -> &'static str {
        match self {
            LogLevelFilter::All => strings.logs_filter_all,
            LogLevelFilter::Debug => strings.logs_filter_debug,
            LogLevelFilter::Info => strings.logs_filter_info,
            LogLevelFilter::Warning => strings.logs_filter_warning,
            LogLevelFilter::Error => strings.logs_filter_error,
        }
    }

    pub fn matches(&self, level: &LogLevel) -> bool {
        match self {
            LogLevelFilter::All => true,
            LogLevelFilter::Debug => matches!(level, LogLevel::Debug),
            LogLevelFilter::Info => matches!(level, LogLevel::Info),
            LogLevelFilter::Warning => matches!(level, LogLevel::Warning),
            LogLevelFilter::Error => matches!(level, LogLevel::Error),
        }
    }

    pub const ALL: &'static [LogLevelFilter] = &[
        LogLevelFilter::All,
        LogLevelFilter::Debug,
        LogLevelFilter::Info,
        LogLevelFilter::Warning,
        LogLevelFilter::Error,
    ];
}

// ─── Mock data ─────────────────────────────────────────────────────────

fn build_mock_logs() -> Vec<LogEntry> {
    vec![
        LogEntry { time: "14:30:01", level: LogLevel::Info,  message: "[Core] Clash core started (v1.18.0)" },
        LogEntry { time: "14:30:01", level: LogLevel::Info,  message: "[Config] loading configuration from /etc/clash/config.yaml" },
        LogEntry { time: "14:30:02", level: LogLevel::Debug, message: "[DNS] resolver initialized — primary: 223.5.5.5" },
        LogEntry { time: "14:30:02", level: LogLevel::Info,  message: "[Rule] loaded 342 rules from 'rule-providers'" },
        LogEntry { time: "14:30:03", level: LogLevel::Info,  message: "[Proxy] group 'GLOBAL' initialized with 5 nodes" },
        LogEntry { time: "14:30:03", level: LogLevel::Info,  message: "[Proxy] group 'HK' initialized with 8 nodes" },
        LogEntry { time: "14:30:03", level: LogLevel::Debug, message: "[TUN] interface created: utun4" },
        LogEntry { time: "14:30:04", level: LogLevel::Info,  message: "[Server] HTTP proxy listening on 127.0.0.1:7890" },
        LogEntry { time: "14:30:04", level: LogLevel::Info,  message: "[Server] SOCKS5 proxy listening on 127.0.0.1:7891" },
        LogEntry { time: "14:30:04", level: LogLevel::Info,  message: "[Server] Mixed proxy listening on 127.0.0.1:7892" },
        LogEntry { time: "14:30:05", level: LogLevel::Debug, message: "[DNS] resolved google.com → 142.250.80.46 (rule: proxy)" },
        LogEntry { time: "14:30:05", level: LogLevel::Debug, message: "[DNS] resolved github.com → 140.82.113.3 (rule: proxy)" },
        LogEntry { time: "14:30:05", level: LogLevel::Info,  message: "[Connection] 192.168.1.100 → google.com:443 via HK 01 [MATCH,rule]" },
        LogEntry { time: "14:30:06", level: LogLevel::Info,  message: "[Connection] 192.168.1.100 → github.com:443 via HK 02 [MATCH,rule]" },
        LogEntry { time: "14:30:07", level: LogLevel::Debug, message: "[Proxy] HK 01 latency measured: 45ms" },
        LogEntry { time: "14:30:07", level: LogLevel::Debug, message: "[Proxy] HK 02 latency measured: 62ms" },
        LogEntry { time: "14:30:08", level: LogLevel::Warning, message: "[DNS] upstream timeout for domain 'api.example.com', retrying..." },
        LogEntry { time: "14:30:09", level: LogLevel::Info,  message: "[Connection] 192.168.1.101 → baidu.com:443 via DIRECT [DOMAIN,rule]" },
        LogEntry { time: "14:30:10", level: LogLevel::Error, message: "[Proxy] HK 03 connection refused (ECONNREFUSED), falling back to HK 01" },
        LogEntry { time: "14:30:10", level: LogLevel::Info,  message: "[Connection] 192.168.1.100 → twitter.com:443 via HK 01 [MATCH,rule]" },
        LogEntry { time: "14:30:11", level: LogLevel::Info,  message: "[Connection] 192.168.1.101 → bing.com:443 via DIRECT [DOMAIN,rule]" },
        LogEntry { time: "14:30:12", level: LogLevel::Debug, message: "[Sniffer] detected TLS handshake for youtube.com" },
        LogEntry { time: "14:30:12", level: LogLevel::Info,  message: "[Connection] 192.168.1.100 → youtube.com:443 via HK 01 [MATCH,rule]" },
        LogEntry { time: "14:30:13", level: LogLevel::Warning, message: "[Rule] no matching rule found for 'unknown-service.local', using MATCH" },
        LogEntry { time: "14:30:14", level: LogLevel::Info,  message: "[Connection] 192.168.1.100 → unknown-service.local:8080 via HK 01 [MATCH,rule]" },
        LogEntry { time: "14:30:15", level: LogLevel::Debug, message: "[Cron] auto-updating proxy provider 'free-nodes'" },
        LogEntry { time: "14:30:15", level: LogLevel::Info,  message: "[Provider] free-nodes updated successfully (25 nodes)" },
        LogEntry { time: "14:30:16", level: LogLevel::Error, message: "[Provider] geoip.dat download failed — HTTP 503 (Service Unavailable)" },
        LogEntry { time: "14:30:17", level: LogLevel::Warning, message: "[TUN] packet dropped (invalid checksum) from 192.168.1.1" },
        LogEntry { time: "14:30:18", level: LogLevel::Info,  message: "[Connection] 192.168.1.101 → zhihu.com:443 via DIRECT [DOMAIN,rule]" },
        LogEntry { time: "14:30:19", level: LogLevel::Info,  message: "[Connection] 192.168.1.100 → openai.com:443 via HK 01 [MATCH,rule]" },
        LogEntry { time: "14:30:20", level: LogLevel::Debug, message: "[DNS] cache hit for github.com (TTL: 287s remaining)" },
        LogEntry { time: "14:30:20", level: LogLevel::Info,  message: "[Connection] 192.168.1.100 → slack.com:443 via HK 01 [MATCH,rule]" },
        LogEntry { time: "14:30:21", level: LogLevel::Debug, message: "[Proxy] HK 01 connection pool: 12 active, 8 idle" },
        LogEntry { time: "14:30:22", level: LogLevel::Info,  message: "[Connection] 192.168.1.100 → discord.com:443 via HK 02 [MATCH,rule]" },
        LogEntry { time: "14:30:23", level: LogLevel::Error, message: "[Connection] 192.168.1.101 → api.example.com:443 timeout after 30s (DIRECT)" },
        LogEntry { time: "14:30:24", level: LogLevel::Debug, message: "[Config] hot-reload triggered — `config.yaml` changed on disk" },
        LogEntry { time: "14:30:24", level: LogLevel::Info,  message: "[Config] reloaded successfully (342 rules, 3 proxy groups)" },
    ]
}

// ─── Main view ─────────────────────────────────────────────────────────

pub(super) fn logs_view(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    filter_level: LogLevelFilter,
    search: &Entity<TextInput>,
) -> impl IntoElement + use<> {
    let logs = build_mock_logs();

    // Filter by level and search text
    let st = search.read(cx).text().to_string();
    let st_lower = st.to_lowercase();
    let filtered: Vec<(usize, LogEntry)> = logs
        .iter()
        .enumerate()
        .filter(|(_, entry)| {
            let lv = filter_level.matches(&entry.level);
            let kw = st_lower.is_empty()
                || entry.message.to_lowercase().contains(&st_lower);
            lv && kw
        })
        .map(|(i, e)| (i, e.clone()))
        .collect();

    let total = logs.len();
    let showing = filtered.len();
    let has_search = !st.is_empty();

    div()
        .flex()
        .flex_col()
        .w_full()
        .gap(px(8.0))
        .px(px(24.0))
        .py(px(8.0))
        // ── Toolbar ────────────────────────────────
        .child(render_toolbar(theme, cx, strings, filter_level, &st, has_search, search))
        // ── Search indicator ──────────────────────
        .when(has_search, |s| {
            let ft = st.clone();
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
                            .child(format!("Filter: \"{}\" — {} results", ft, showing)),
                    )
                    .child(
                        div()
                            .id("logs-filter-clear")
                            .text_size(px(12.0))
                            .text_color(rgb(theme.accent))
                            .cursor_pointer()
                            .on_click(cx.listener({
                                let search = search.clone();
                                move |_this: &mut crate::app::AppView, _, _, cx| {
                                    search.update(cx, |t, _| t.clear());
                                    cx.notify();
                                }
                            }))
                            .child("Clear"),
                    ),
            )
        })
        // ── Log table ─────────────────────────────
        .child(render_log_table(theme, strings, &filtered))
        // ── Bottom bar ────────────────────────────
        .child(render_bottom_bar(theme, cx, strings, showing, total))
        .child(div().h(px(12.0)))
}

// ─── Toolbar ───────────────────────────────────────────────────────────

fn render_toolbar(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    filter_level: LogLevelFilter,
    _search_text: &str,
    _has_search: bool,
    search: &Entity<TextInput>,
) -> impl IntoElement + use<> {
    let clear_label = strings.logs_clear.to_string();
    let search_rendered = search.update(cx, |t, cx| t.render_plain(theme, cx));

    div()
        .flex()
        .items_center()
        .gap(px(8.0))
        // ── Level filter buttons ──────────────────
        .child({
            let entity = cx.entity();
            let mut btns = div().flex().items_center().gap(px(4.0));
            for &filter in LogLevelFilter::ALL {
                let is_active = filter_level == filter;
                let lbl = filter.label(strings).to_string();
                let color = match filter {
                    LogLevelFilter::All => rgb(theme.text_secondary),
                    LogLevelFilter::Debug => rgb(theme.text_disabled),
                    LogLevelFilter::Info => rgb(theme.status_success),
                    LogLevelFilter::Warning => rgb(theme.status_warning),
                    LogLevelFilter::Error => rgb(theme.status_error),
                };
                let bg = if is_active {
                    let mut c = color;
                    c.a = 0.18;
                    c
                } else {
                    rgba(0x00000000)
                };
                let border_color = if is_active {
                    color
                } else {
                    rgb(theme.border_light)
                };
                let fw = if is_active { FontWeight::MEDIUM } else { FontWeight::NORMAL };
                let tc = if is_active { color } else { rgb(theme.text_secondary) };
                {
                    let entity = entity.clone();
                    btns = btns.child(
                        div()
                            .flex()
                            .items_center()
                            .px(px(10.0))
                            .py(px(6.0))
                            .rounded(px(CARD_RADIUS))
                            .text_size(px(12.0))
                            .font_weight(fw)
                            .text_color(tc)
                            .bg(bg)
                            .border_1()
                            .border_color(border_color)
                            .cursor_pointer()
                            .hover(move |s| {
                                if !is_active {
                                    s.bg(rgb(theme.surface_variant))
                                } else {
                                    s
                                }
                            })
                            .on_any_mouse_down(move |_: &MouseDownEvent, _window, app| {
                                entity.update(app, |this, _| {
                                    this.logs_filter_level = filter;
                                });
                                app.refresh_windows();
                            })
                            .child(lbl),
                    );
                }
            }
            btns
        })
        // ── Spacer ────────────────────────────────
        .child(div().flex_1())
        // ── Search input ──────────────────────────
        .child(
            div()
                .w(px(192.0))
                .child(search_rendered),
        )
        // ── Clear button ──────────────────────────
        .child(
            div()
                .id("logs-clear-all")
                .flex()
                .items_center()
                .px(px(14.0))
                .py(px(6.0))
                .rounded(px(CARD_RADIUS))
                .bg(rgb(theme.status_error))
                .cursor_pointer()
                .hover(|s| s.opacity(0.85))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.logs_filter_level = LogLevelFilter::All;
                    cx.notify();
                }))
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0xffffff))
                        .child(clear_label),
                ),
        )
}


// ─── Log table ─────────────────────────────────────────────────────────

fn render_log_table(
    theme: &Theme,
    strings: &I18nStrings,
    filtered: &[(usize, LogEntry)],
) -> impl IntoElement + use<> {
    let empty = filtered.is_empty();
    let time_label = strings.logs_col_time.to_string();
    let level_label = strings.logs_col_level.to_string();
    let msg_label = strings.logs_col_message.to_string();

    div()
        .flex()
        .flex_col()
        .rounded(px(CARD_RADIUS))
        .bg(rgb(theme.surface))
        .border_1()
        .border_color(rgb(theme.border_light))
        // ── Table header ──────────────────────────
        .child(
            div()
                .flex()
                .items_center()
                .px(px(16.0))
                .py(px(10.0))
                .bg(rgb(theme.surface_variant))
                .border_b_1()
                .border_color(rgb(theme.border_light))
                .text_size(px(11.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme.text_secondary))
                .child(div().w(px(80.0)).flex_shrink_0().child(time_label))
                .child(div().w(px(72.0)).flex_shrink_0().child(level_label))
                .child(div().flex_1().overflow_hidden().child(msg_label)),
        )
        // ── Empty state ──────────────────────────
        .when(empty, |s| {
            s.child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .py(px(48.0))
                    .text_size(px(13.0))
                    .text_color(rgb(theme.text_disabled))
                    .child(strings.logs_empty),
            )
        })
        // ── Table body ───────────────────────────
        .when(!empty, |s| {
            s.child(
                div()
                    .id("log-table-body")
                    .flex()
                    .flex_col()
                    .overflow_y_scroll()
                    .children(
                        filtered
                            .iter()
                            .map(|(orig_idx, entry)| {
                                render_log_row(theme, strings, *orig_idx, entry)
                            }),
                    ),
            )
        })
}

// ─── Single log row ────────────────────────────────────────────────────

fn render_log_row(
    theme: &Theme,
    strings: &I18nStrings,
    _orig_index: usize,
    entry: &LogEntry,
) -> impl IntoElement + use<> {
    let time_str = entry.time.to_string();
    let msg_str = entry.message.to_string();
    let level_bg = entry.level.bg_color(theme);
    let level_fg = entry.level.color(theme);
    let level_tag = entry.level.tag(strings).to_string();

    div()
        .id(("log-row", _orig_index))
        .flex()
        .items_center()
        .px(px(16.0))
        .py(px(6.0))
        .border_b_1()
        .border_color(rgb(theme.border_light))
        .hover(|s| s.bg(rgb(theme.surface_variant)))
        // Time
        .child(
            div()
                .w(px(80.0))
                .flex_shrink_0()
                .text_size(px(11.0))
                .text_color(rgb(theme.text_disabled))
                .font_family("JetBrains Mono, Consolas, monospace")
                .child(time_str),
        )
        // Level badge
        .child(
            div()
                .w(px(72.0))
                .flex_shrink_0()
                .flex()
                .items_center()
                .child(
                    div()
                        .px(px(6.0))
                        .py(px(2.0))
                        .rounded(px(3.0))
                        .bg(level_bg)
                        .text_size(px(10.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(level_fg)
                        .child(level_tag),
                ),
        )
        // Message
        .child(
            div()
                .flex_1()
                .text_size(px(11.0))
                .text_color(rgb(theme.text_primary))
                .font_family("JetBrains Mono, Consolas, monospace")
                .overflow_hidden()
                .text_ellipsis()
                .child(msg_str),
        )
}

// ─── Bottom bar ────────────────────────────────────────────────────────

fn render_bottom_bar(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    showing: usize,
    total: usize,
) -> impl IntoElement + use<> {
    let count_text = format!("{} {} / {} {}", strings.logs_showing, showing, total, strings.logs_total);
    let copy_label = strings.logs_copy.to_string();
    let export_label = strings.logs_export.to_string();

    div()
        .flex()
        .items_center()
        .justify_between()
        .px(px(8.0))
        .py(px(6.0))
        .child(
            div()
                .text_size(px(11.0))
                .text_color(rgb(theme.text_disabled))
                .child(count_text),
        )
        .child(
            div().flex().items_center().gap(px(6.0))
                // Copy button
                .child(
                    div()
                        .id("logs-copy")
                        .flex()
                        .items_center()
                        .gap(px(4.0))
                        .px(px(10.0))
                        .py(px(5.0))
                        .rounded(px(CARD_RADIUS))
                        .bg(rgb(theme.surface))
                        .border_1()
                        .border_color(rgb(theme.border_light))
                        .text_size(px(11.0))
                        .text_color(rgb(theme.text_secondary))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(theme.surface_variant)))
                        .on_click(cx.listener(|this, _ev, _window, cx| {
                            // TODO: copy logs to clipboard
                            let _ = this;
                            cx.notify();
                        }))
                        .child(copy_label),
                )
                // Export button
                .child(
                    div()
                        .id("logs-export")
                        .flex()
                        .items_center()
                        .gap(px(4.0))
                        .px(px(10.0))
                        .py(px(5.0))
                        .rounded(px(CARD_RADIUS))
                        .bg(rgb(theme.surface))
                        .border_1()
                        .border_color(rgb(theme.border_light))
                        .text_size(px(11.0))
                        .text_color(rgb(theme.text_secondary))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(theme.surface_variant)))
                        .on_click(cx.listener(|this, _ev, _window, cx| {
                            // TODO: export logs to file
                            let _ = this;
                            cx.notify();
                        }))
                        .child(export_label),
                ),
        )
}
