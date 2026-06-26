/// Requests view — real-time request log, detail panel, search/filter, clear logs.

use gpui::prelude::*;
use gpui::*;

use crate::i18n::I18nStrings;
use crate::theme::{Theme, CARD_RADIUS};

// ─── Data types ────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct MockRequest {
    pub time: &'static str,
    pub domain: &'static str,
    pub url: &'static str,
    pub method: &'static str,
    pub status: u16,
    pub proxy_chain: &'static str,
    pub delay_ms: Option<u64>,
    pub rule: &'static str,
    pub rule_type: RuleType,
    pub source_ip: &'static str,
    pub source_port: u16,
    pub dest_ip: &'static str,
    pub dest_port: u16,
    pub headers: Vec<(&'static str, &'static str)>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RuleType {
    Matched,
    Direct,
    Proxy,
}

// ─── Mock data ─────────────────────────────────────────────────────────

fn build_mock_requests() -> Vec<MockRequest> {
    vec![
        MockRequest {
            time: "14:32:05",
            domain: "google.com",
            url: "https://www.google.com/search?q=rust+gpui",
            method: "GET",
            status: 200,
            proxy_chain: "GLOBAL >> HK 01",
            delay_ms: Some(45),
            rule: "google.com",
            rule_type: RuleType::Matched,
            source_ip: "192.168.1.100",
            source_port: 52341,
            dest_ip: "142.250.80.46",
            dest_port: 443,
            headers: vec![
                ("Host", "www.google.com"),
                ("User-Agent", "Mozilla/5.0 (Windows NT 10.0) Chrome/125.0"),
                ("Accept", "text/html,application/xhtml+xml"),
                ("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8"),
            ],
        },
        MockRequest {
            time: "14:32:05",
            domain: "github.com",
            url: "https://api.github.com/repos/zed-industries/zed",
            method: "GET",
            status: 200,
            proxy_chain: "GLOBAL >> SG 01",
            delay_ms: Some(88),
            rule: "PROXY",
            rule_type: RuleType::Proxy,
            source_ip: "192.168.1.100",
            source_port: 52342,
            dest_ip: "140.82.121.3",
            dest_port: 443,
            headers: vec![
                ("Host", "api.github.com"),
                ("User-Agent", "curl/8.7.1"),
                ("Accept", "application/vnd.github+json"),
            ],
        },
        MockRequest {
            time: "14:32:04",
            domain: "baidu.com",
            url: "https://www.baidu.com/",
            method: "GET",
            status: 302,
            proxy_chain: "DIRECT",
            delay_ms: Some(2),
            rule: "DIRECT",
            rule_type: RuleType::Direct,
            source_ip: "192.168.1.100",
            source_port: 52340,
            dest_ip: "110.242.68.66",
            dest_port: 443,
            headers: vec![
                ("Host", "www.baidu.com"),
                ("User-Agent", "Mozilla/5.0 (Windows NT 10.0) Chrome/125.0"),
                ("Accept", "text/html"),
            ],
        },
        MockRequest {
            time: "14:32:03",
            domain: "youtube.com",
            url: "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            method: "GET",
            status: 200,
            proxy_chain: "Streaming >> US 01",
            delay_ms: Some(170),
            rule: "youtube.com",
            rule_type: RuleType::Matched,
            source_ip: "192.168.1.100",
            source_port: 52339,
            dest_ip: "142.250.80.46",
            dest_port: 443,
            headers: vec![
                ("Host", "www.youtube.com"),
                ("User-Agent", "Mozilla/5.0 (Windows NT 10.0) Chrome/125.0"),
                ("Accept", "text/html"),
                ("Cookie", "PREF=...; VISITOR_INFO1_LIVE=..."),
            ],
        },
        MockRequest {
            time: "14:32:02",
            domain: "twitter.com",
            url: "https://twitter.com/home",
            method: "GET",
            status: 200,
            proxy_chain: "GLOBAL >> HK 01",
            delay_ms: Some(55),
            rule: "twitter.com",
            rule_type: RuleType::Matched,
            source_ip: "192.168.1.100",
            source_port: 52338,
            dest_ip: "104.244.42.1",
            dest_port: 443,
            headers: vec![
                ("Host", "twitter.com"),
                ("User-Agent", "Mozilla/5.0 (Windows NT 10.0) Chrome/125.0"),
                ("Accept", "text/html"),
            ],
        },
        MockRequest {
            time: "14:32:01",
            domain: "localhost",
            url: "http://localhost:9090/configs",
            method: "POST",
            status: 204,
            proxy_chain: "DIRECT",
            delay_ms: Some(0),
            rule: "DIRECT",
            rule_type: RuleType::Direct,
            source_ip: "127.0.0.1",
            source_port: 52337,
            dest_ip: "127.0.0.1",
            dest_port: 9090,
            headers: vec![
                ("Host", "localhost:9090"),
                ("User-Agent", "clash-verge/1.0"),
                ("Content-Type", "application/json"),
            ],
        },
        MockRequest {
            time: "14:31:58",
            domain: "reddit.com",
            url: "https://www.reddit.com/r/rust/.json",
            method: "GET",
            status: 200,
            proxy_chain: "GLOBAL >> JP 01",
            delay_ms: Some(135),
            rule: "geosite:reddit",
            rule_type: RuleType::Matched,
            source_ip: "192.168.1.100",
            source_port: 52336,
            dest_ip: "151.101.1.140",
            dest_port: 443,
            headers: vec![
                ("Host", "www.reddit.com"),
                ("User-Agent", "Mozilla/5.0 (Windows NT 10.0) Chrome/125.0"),
                ("Accept", "application/json"),
            ],
        },
        MockRequest {
            time: "14:31:55",
            domain: "clients3.google.com",
            url: "https://clients3.google.com/generate_204",
            method: "GET",
            status: 204,
            proxy_chain: "GLOBAL >> HK 01",
            delay_ms: Some(42),
            rule: "google.com",
            rule_type: RuleType::Matched,
            source_ip: "192.168.1.100",
            source_port: 52335,
            dest_ip: "142.250.80.46",
            dest_port: 443,
            headers: vec![
                ("Host", "clients3.google.com"),
                ("User-Agent", "Mozilla/5.0 (Windows NT 10.0) Chrome/125.0"),
            ],
        },
        MockRequest {
            time: "14:31:50",
            domain: "openai.com",
            url: "https://api.openai.com/v1/chat/completions",
            method: "POST",
            status: 200,
            proxy_chain: "GLOBAL >> US 01",
            delay_ms: Some(320),
            rule: "openai.com",
            rule_type: RuleType::Matched,
            source_ip: "192.168.1.100",
            source_port: 52334,
            dest_ip: "104.18.37.228",
            dest_port: 443,
            headers: vec![
                ("Host", "api.openai.com"),
                ("User-Agent", "OpenAI/NodeJS/4.0"),
                ("Content-Type", "application/json"),
                ("Authorization", "Bearer sk-****"),
            ],
        },
        MockRequest {
            time: "14:31:48",
            domain: "akamaihd.net",
            url: "https://akamaihd.net/video/stream/segment-001.ts",
            method: "GET",
            status: 200,
            proxy_chain: "Streaming >> US 01",
            delay_ms: Some(185),
            rule: "PROXY",
            rule_type: RuleType::Proxy,
            source_ip: "192.168.1.100",
            source_port: 52333,
            dest_ip: "23.212.6.142",
            dest_port: 443,
            headers: vec![
                ("Host", "akamaihd.net"),
                ("User-Agent", "Mozilla/5.0 (Windows NT 10.0) Chrome/125.0"),
            ],
        },
    ]
}

// ─── Main view ─────────────────────────────────────────────────────────

pub(super) fn requests_view(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    search_text: &str,
    selected_index: Option<usize>,
) -> impl IntoElement + use<> {
    let requests = build_mock_requests();

    // Filter by search text
    let search_text = search_text.to_string();
    let ft = search_text.to_lowercase();
    let filtered: Vec<(usize, MockRequest)> = requests
        .iter()
        .enumerate()
        .filter(|(_, r)| {
            ft.is_empty()
                || r.domain.to_lowercase().contains(&ft)
                || r.url.to_lowercase().contains(&ft)
                || r.proxy_chain.to_lowercase().contains(&ft)
                || r.rule.to_lowercase().contains(&ft)
                || r.method.to_lowercase().contains(&ft)
        })
        .map(|(i, r)| (i, r.clone()))
        .collect();

    let selected = selected_index
        .and_then(|si| requests.get(si))
        .cloned();

    let has_search = !search_text.is_empty();

    div()
        .flex()
        .flex_col()
        .w_full()
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
                            .id("requests-filter-clear")
                            .text_size(px(12.0))
                            .text_color(rgb(theme.accent))
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.requests_search_text.clear();
                                this.requests_selected_index = None;
                                cx.notify();
                            }))
                            .child("Clear"),
                    ),
            )
        })
        // ── Request list ──────────────────────────
        .child(render_request_list(theme, cx, strings, &filtered, selected_index))
        // ── Detail panel (when selected) ─────────
        .when_some(selected, |s, req| {
            s.child(render_detail_panel(theme, strings, &req))
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
    let placeholder = strings.requests_search_placeholder.to_string();
    let display = search_text.to_string();
    let clear_label = strings.requests_clear_logs.to_string();

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
        // Clear all button
        .child(
            div()
                .id("requests-clear-all")
                .flex()
                .items_center()
                .px(px(14.0))
                .py(px(8.0))
                .rounded(px(CARD_RADIUS))
                .bg(rgb(theme.status_error))
                .cursor_pointer()
                .hover(|s| s.opacity(0.85))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.requests_search_text.clear();
                    this.requests_selected_index = None;
                    cx.notify();
                }))
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0xffffff))
                        .child(clear_label),
                ),
        )
}

// ─── Request list table ────────────────────────────────────────────────

fn render_request_list(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    filtered: &[(usize, MockRequest)],
    selected_index: Option<usize>,
) -> impl IntoElement + use<> {
    let empty = filtered.is_empty();

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
                .child(div().flex_1().child(strings.requests_col_time))
                .child(div().flex_1().child(strings.requests_col_domain))
                .child(div().flex_1().child(strings.requests_col_method))
                .child(div().flex_1().child(strings.requests_col_proxy))
                .child(div().w(px(72.0)).text_align(TextAlign::Right).child(strings.requests_col_delay))
                .child(div().flex_1().child(strings.requests_col_rule)),
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
                    .child(strings.requests_empty),
            )
        })
        // ── Table body (scrollable) ───────────────
        .when(!empty, |s| {
            s.child(
                div()
                    .id("request-list-body")
                    .flex()
                    .flex_col()
                    .overflow_y_scroll()
                    .children(
                        filtered
                            .iter()
                            .enumerate()
                            .map(|(vi, (orig_idx, req))| {
                                render_request_row(theme, cx, strings, *orig_idx, req, vi, selected_index)
                            }),
                    ),
            )
        })
}

// ─── Single request row ────────────────────────────────────────────────

fn render_request_row(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    orig_index: usize,
    req: &MockRequest,
    _view_index: usize,
    selected_index: Option<usize>,
) -> impl IntoElement + use<> {
    let is_selected = selected_index == Some(orig_index);
    let method = req.method.to_string();
    let domain_str = req.domain.to_string();
    let time_str = req.time.to_string();
    let proxy = req.proxy_chain.to_string();

    let method_color = match req.method {
        "GET" => rgb(theme.status_success),
        "POST" => rgb(theme.status_info),
        "PUT" => rgb(theme.status_warning),
        "DELETE" => rgb(theme.status_error),
        _ => rgb(theme.text_secondary),
    };

    let status_color = if req.status < 300 {
        rgb(theme.status_success)
    } else if req.status < 400 {
        rgb(theme.status_warning)
    } else {
        rgb(theme.status_error)
    };

    let delay_text = match req.delay_ms {
        Some(d) => format_delay(d),
        None => strings.proxy_delay_na.to_string(),
    };
    let delay_color = match req.delay_ms {
        Some(d) if d < 100 => rgb(theme.status_success),
        Some(d) if d < 300 => rgb(theme.status_warning),
        Some(_) => rgb(theme.status_error),
        None => rgb(theme.text_disabled),
    };

    let rule_text = match req.rule_type {
        RuleType::Direct => strings.requests_rule_direct,
        RuleType::Proxy => strings.requests_rule_proxy,
        RuleType::Matched => req.rule,
    };
    let rule_color = match req.rule_type {
        RuleType::Direct => rgb(theme.status_success),
        RuleType::Matched => rgb(theme.accent),
        RuleType::Proxy => rgb(theme.status_info),
    };

    let row_bg = if is_selected {
        rgb(theme.accent_muted)
    } else {
        rgba(0x00000000)
    };

    div()
        .id(("request-row", orig_index))
        .flex()
        .items_center()
        .px(px(16.0))
        .py(px(8.0))
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
        .on_click(cx.listener(move |this, _, _, cx| {
            this.requests_selected_index = Some(orig_index);
            cx.notify();
        }))
        // Time
        .child(
            div()
                .flex_1()
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(theme.text_secondary))
                .child(time_str),
        )
        // Domain
        .child(
            div()
                .flex_1()
                .text_size(px(12.0))
                .text_color(rgb(theme.text_primary))
                .overflow_hidden()
                .text_ellipsis()
                .child(domain_str),
        )
        // Method + Status
        .child(
            div()
                .flex_1()
                .flex()
                .items_center()
                .gap(px(6.0))
                .child(
                    div()
                        .text_size(px(11.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(method_color)
                        .child(method),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(status_color)
                        .child(format!("{}", req.status)),
                ),
        )
        // Proxy chain
        .child(
            div()
                .flex_1()
                .text_size(px(11.0))
                .text_color(rgb(theme.text_secondary))
                .overflow_hidden()
                .text_ellipsis()
                .child(proxy),
        )
        // Delay
        .child(
            div()
                .w(px(72.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::MEDIUM)
                .text_color(delay_color)
                .text_align(TextAlign::Right)
                .child(delay_text),
        )
        // Rule
        .child(
            div()
                .flex_1()
                .text_size(px(11.0))
                .text_color(rule_color)
                .overflow_hidden()
                .text_ellipsis()
                .child(rule_text),
        )
}

// ─── Detail panel ──────────────────────────────────────────────────────

fn render_detail_panel(
    theme: &Theme,
    strings: &I18nStrings,
    req: &MockRequest,
) -> impl IntoElement + use<> {
    let method_color = match req.method {
        "GET" => rgb(theme.status_success),
        "POST" => rgb(theme.status_info),
        "PUT" => rgb(theme.status_warning),
        "DELETE" => rgb(theme.status_error),
        _ => rgb(theme.text_secondary),
    };

    let status_color = if req.status < 300 {
        rgb(theme.status_success)
    } else if req.status < 400 {
        rgb(theme.status_warning)
    } else {
        rgb(theme.status_error)
    };

    let delay_text: String = match req.delay_ms {
        Some(d) => format!("{}ms", format_delay(d)),
        None => strings.proxy_delay_na.to_string(),
    };

    let source = format!("{}:{}", req.source_ip, req.source_port);
    let dest = format!("{}:{}", req.dest_ip, req.dest_port);

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
                        .child(strings.requests_detail_title),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(method_color)
                        .child(req.method),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(status_color)
                        .child(format!("{}", req.status)),
                )
                .child(
                    div().flex_1().text_size(px(12.0)).text_color(rgb(theme.text_primary)).overflow_hidden().text_ellipsis().child(req.domain),
                ),
        )
        // Content
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .p(px(16.0))
                // URL
                .child(detail_row(theme, strings.requests_detail_url, req.url))
                // Grid: Method / Status / Delay / Time
                .child(
                    div().flex().gap(px(24.0))
                        .child(detail_row(theme, strings.requests_detail_method, req.method))
                        .child(detail_row(theme, strings.requests_detail_status, &format!("{}", req.status)))
                        .child(detail_row(theme, strings.requests_detail_delay, &delay_text))
                        .child(detail_row(theme, strings.requests_detail_time, req.time)),
                )
                // Proxy chain / Rule
                .child(
                    div().flex().gap(px(24.0))
                        .child(detail_row(theme, strings.requests_detail_proxy, req.proxy_chain))
                        .child(detail_row(theme, strings.requests_detail_rule, req.rule)),
                )
                // Source / Destination
                .child(
                    div().flex().gap(px(24.0))
                        .child(detail_row(theme, strings.requests_detail_source, &source))
                        .child(detail_row(theme, strings.requests_detail_destination, &dest)),
                )
                // Divider
                .child(div().h(px(1.0)).bg(rgb(theme.border_light)))
                // Headers
                .child(
                    div().flex().flex_col().gap(px(4.0))
                        .child(
                            div()
                                .text_size(px(12.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(theme.text_secondary))
                                .child(strings.requests_detail_headers),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(3.0))
                                .children(req.headers.iter().map(|(k, v)| {
                                    header_row(theme, k, v)
                                })),
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

fn header_row(theme: &Theme, key: &str, value: &str) -> impl IntoElement {
    let key = key.to_string();
    let value = value.to_string();
    div()
        .flex()
        .gap(px(8.0))
        .child(
            div()
                .text_size(px(11.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme.accent))
                .flex_shrink_0()
                .child(key),
        )
        .child(
            div()
                .text_size(px(11.0))
                .text_color(rgb(theme.text_secondary))
                .overflow_hidden()
                .text_ellipsis()
                .child(value),
        )
}

// ─── Helpers ───────────────────────────────────────────────────────────

fn format_delay(ms: u64) -> String {
    if ms == 0 {
        "<1ms".into()
    } else if ms >= 1000 {
        format!("{:.1}s", ms as f64 / 1000.0)
    } else {
        format!("{}ms", ms)
    }
}
