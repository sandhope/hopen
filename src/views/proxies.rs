/// Proxies view — proxy group list, node management, speed test, provider list.

use super::search_input::SearchInput;
use gpui::prelude::*;
use gpui::*;

use crate::i18n::I18nStrings;
use crate::theme::{Theme, CARD_RADIUS};

// ─── Data types ────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum GroupType {
    Selector,
    URLTest,
    Fallback,
    LoadBalance,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum NodeType {
    Direct,
    Reject,
    SS,
    VMess,
    Trojan,
    Hysteria2,
    Socks5,
    HTTP,
    WireGuard,
    TUIC,
    SSH,
}

impl NodeType {
    pub fn label(&self) -> &'static str {
        match self {
            NodeType::Direct => "Direct",
            NodeType::Reject => "Reject",
            NodeType::SS => "SS",
            NodeType::VMess => "VMess",
            NodeType::Trojan => "Trojan",
            NodeType::Hysteria2 => "Hysteria2",
            NodeType::Socks5 => "Socks5",
            NodeType::HTTP => "HTTP",
            NodeType::WireGuard => "WireGuard",
            NodeType::TUIC => "TUIC",
            NodeType::SSH => "SSH",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            NodeType::Direct => "\u{27A1}",
            NodeType::Reject => "\u{274C}",
            NodeType::SS => "\u{1F320}",
            NodeType::VMess => "\u{1F310}",
            NodeType::Trojan => "\u{1F3F0}",
            NodeType::Hysteria2 => "\u{26A1}",
            NodeType::Socks5 => "\u{1F9E6}",
            NodeType::HTTP => "\u{1F310}",
            NodeType::WireGuard => "\u{1F6E1}",
            NodeType::TUIC => "\u{2708}",
            NodeType::SSH => "\u{1F6AA}",
        }
    }
}

// ─── Main view ─────────────────────────────────────────────────────────

pub(super) fn proxies_view(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    search_text: &str,
    expanded_map: &std::collections::HashMap<String, bool>,
    search_input_entity: &Entity<SearchInput>,
) -> impl IntoElement + use<> {
    let groups = build_mock_groups();

    let search_text = search_text.to_string();
    let search_has_text = !search_text.is_empty();
    let expanded_map = expanded_map.clone();

    div()
        .flex()
        .flex_col()
        .size_full()
        .child(render_search_widget(theme, strings, cx, &search_text, search_input_entity))
        .child(
            div()
                .id("proxies-scroll-area")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .px(px(24.0))
                .py(px(8.0))
                .gap(px(8.0))
                .children({
                    let mut group_views = Vec::new();
                    for (gi, g) in groups.iter().enumerate() {
                        let gn = g.name.to_string();
                        let exp = expanded_map.get(&gn).copied().unwrap_or(g.expanded);
                        group_views.push(proxy_group_section(
                            theme, strings, cx, g.name, g.group_type, exp, g.current_idx, &g.nodes, gi,
                        ));
                    }
                    group_views
                })
                .child(providers_section(theme, strings, cx))
                .child(div().h(px(16.0))),
        )
        .when(search_has_text, |s| {
            let ft = search_text.clone();
            s.child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px(px(24.0))
                    .py(px(8.0))
                    .border_t_1()
                    .border_color(rgb(theme.border_light))
                    .bg(rgb(theme.surface))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(theme.text_secondary))
                            .child(format!("Filter: \"{}\"", ft)),
                    )
                    .child(
                        div()
                            .id("proxies-filter-clear")
                            .text_size(px(12.0))
                            .text_color(rgb(theme.accent))
                            .cursor_pointer()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.proxies_search_text.clear();
                                cx.notify();
                            }))
                            .child("Clear"),
                    ),
            )
        })
}

// ─── Mock data ────────────────────────────────────────────────────────

struct MockGroup {
    name: &'static str,
    group_type: GroupType,
    expanded: bool,
    current_idx: usize,
    nodes: Vec<MockNode>,
}

struct MockNode {
    name: &'static str,
    node_type: NodeType,
    delay: Option<u64>,
}

fn build_mock_groups() -> Vec<MockGroup> {
    vec![
        MockGroup {
            name: "GLOBAL",
            group_type: GroupType::Selector,
            expanded: true,
            current_idx: 0,
            nodes: vec![
                MockNode { name: "\u{1F1ED}\u{1F1F0} HongKong 01", node_type: NodeType::SS, delay: Some(45) },
                MockNode { name: "\u{1F1F8}\u{1F1EC} Singapore 01", node_type: NodeType::VMess, delay: Some(88) },
                MockNode { name: "\u{1F1EF}\u{1F1F5} Tokyo 01", node_type: NodeType::Trojan, delay: Some(120) },
            ],
        },
        MockGroup {
            name: "Auto Select",
            group_type: GroupType::URLTest,
            expanded: false,
            current_idx: 0,
            nodes: vec![
                MockNode { name: "\u{1F1ED}\u{1F1F0} HongKong 02", node_type: NodeType::SS, delay: Some(45) },
                MockNode { name: "\u{1F1F8}\u{1F1EC} Singapore 02", node_type: NodeType::Hysteria2, delay: Some(52) },
                MockNode { name: "\u{1F1EF}\u{1F1F5} Tokyo 02", node_type: NodeType::VMess, delay: Some(78) },
                MockNode { name: "\u{1F1FA}\u{1F1F8} Los Angeles 01", node_type: NodeType::TUIC, delay: Some(160) },
                MockNode { name: "\u{1F1E9}\u{1F1EA} Frankfurt 01", node_type: NodeType::WireGuard, delay: Some(210) },
            ],
        },
        MockGroup {
            name: "Fallback",
            group_type: GroupType::Fallback,
            expanded: false,
            current_idx: 0,
            nodes: vec![
                MockNode { name: "\u{1F1ED}\u{1F1F0} HongKong 03", node_type: NodeType::SSH, delay: Some(65) },
                MockNode { name: "\u{1F1F8}\u{1F1EC} Singapore 03", node_type: NodeType::Socks5, delay: Some(98) },
                MockNode { name: "\u{1F1EF}\u{1F1F5} Tokyo 03", node_type: NodeType::HTTP, delay: Some(135) },
            ],
        },
        MockGroup {
            name: "Streaming",
            group_type: GroupType::Selector,
            expanded: false,
            current_idx: 0,
            nodes: vec![
                MockNode { name: "\u{1F1FA}\u{1F1F8} Netflix US", node_type: NodeType::VMess, delay: Some(170) },
                MockNode { name: "\u{1F1ED}\u{1F1F0} Netflix HK", node_type: NodeType::Trojan, delay: Some(55) },
                MockNode { name: "Direct", node_type: NodeType::Direct, delay: None },
            ],
        },
    ]
}

fn build_mock_providers() -> Vec<MockProvider> {
    vec![
        MockProvider { name: "Provider A (Subscription)", node_count: 8, updated: "2m ago" },
        MockProvider { name: "Provider B (Local)", node_count: 12, updated: "1h ago" },
    ]
}

struct MockProvider {
    name: &'static str,
    node_count: usize,
    updated: &'static str,
}

// ─── Search bar ───────────────────────────────────────────────────────

fn render_search_widget(
    theme: &Theme,
    strings: &I18nStrings,
    cx: &mut Context<crate::app::AppView>,
    search_text: &str,
    search_entity: &Entity<SearchInput>,
) -> impl IntoElement + use<> {
    let has_text = !search_text.is_empty();
    let display = search_text.to_string();
    let placeholder = strings.proxy_search_placeholder.to_string();
    let focus_handle = search_entity.read(cx).focus_handle_raw().clone();

    div()
        .flex()
        .items_center()
        .gap(px(8.0))
        .px(px(24.0))
        .py(px(12.0))
        .child(
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
                .track_focus(&focus_handle)
                .on_key_down(cx.listener({
                    let entity = search_entity.clone();
                    move |this: &mut crate::app::AppView,
                          event: &KeyDownEvent,
                          window: &mut Window,
                          cx| {
                        let _ = this;
                        entity.read(cx).focus_handle_raw().clone().focus(window);
                        if let Some(ref ch) = event.keystroke.key_char {
                            let s = ch.to_string();
                            this.proxies_search_text.push_str(&s);
                        } else {
                            match event.keystroke.key.as_str() {
                                "backspace" => { this.proxies_search_text.pop(); }
                                "space" => { this.proxies_search_text.push(' '); }
                                _ => {}
                            }
                        }
                        cx.notify();
                    }
                }))
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
                        .when(has_text, |s| {
                            s.text_color(rgb(theme.text_primary)).child(display)
                        })
                        .when(!has_text, |s| {
                            s.text_color(rgb(theme.text_disabled)).child(placeholder)
                        }),
                ),
        )
}

// ─── Proxy group section ──────────────────────────────────────────────

fn proxy_group_section(
    theme: &Theme,
    strings: &I18nStrings,
    cx: &mut Context<crate::app::AppView>,
    name: &str,
    group_type: GroupType,
    expanded: bool,
    current_idx: usize,
    nodes: &[MockNode],
    group_index: usize,
) -> impl IntoElement + use<> {
    let badge_bg = match group_type {
        GroupType::URLTest => rgb(theme.status_info),
        GroupType::Fallback => rgb(theme.status_warning),
        GroupType::LoadBalance => rgb(theme.accent),
        GroupType::Selector => rgb(theme.status_success),
    };
    let badge_text = match group_type {
        GroupType::Selector => strings.proxy_group_selector,
        GroupType::URLTest => strings.proxy_group_urltest,
        GroupType::Fallback => strings.proxy_group_fallback,
        GroupType::LoadBalance => strings.proxy_group_lb,
    };
    let chevron = if expanded { "\u{25BC}" } else { "\u{25B6}" };
    let group_name = name.to_string();

    div()
        .flex()
        .flex_col()
        .rounded(px(CARD_RADIUS))
        .bg(rgb(theme.surface))
        .border_1()
        .border_color(rgb(theme.border_light))
        .child({
            let gname = group_name.clone();
            div()
                .id(("proxy-group-header", group_index))
                .flex()
                .items_center()
                .justify_between()
                .px(px(16.0))
                .py(px(12.0))
                .cursor_pointer()
                .hover(|s| s.bg(rgb(theme.surface_variant)))
                .on_click(cx.listener(move |this, _, _, cx| {
                    let e = this.proxies_expanded.entry(gname.clone());
                    let v = e.or_insert(false);
                    *v = !*v;
                    cx.notify();
                }))
                .child(
                    div().flex().items_center().gap(px(8.0))
                        .child(div().text_size(px(10.0)).text_color(rgb(theme.text_secondary)).child(chevron))
                        .child(div().text_size(px(14.0)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(theme.text_primary)).child(group_name.clone()))
                        .child(div().text_size(px(10.0)).px(px(6.0)).py(px(2.0)).rounded(px(4.0)).bg(badge_bg).text_color(rgb(0xffffff)).child(badge_text)),
                )
                .child(
                    div()
                        .id(("proxy-group-test-all", group_index))
                        .flex()
                        .items_center()
                        .gap(px(4.0))
                        .px(px(10.0))
                        .py(px(6.0))
                        .rounded(px(6.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(theme.border)))
                        .on_click(cx.listener(|this, _, _, cx| {
                            let _ = this; cx.notify();
                        }))
                        .child(div().text_size(px(11.0)).text_color(rgb(theme.text_secondary)).child(strings.proxy_test_all)),
                )
        })
        .when(expanded, |s| {
            s.child(
                div()
                    .flex()
                    .flex_col()
                    .border_t_1()
                    .border_color(rgb(theme.border_light))
                    .children({
                        let mut node_views = Vec::new();
                        for (ni, n) in nodes.iter().enumerate() {
                            let is_cur = ni == current_idx;
                            node_views.push(proxy_node_row(theme, strings, cx, n, is_cur, group_index, ni));
                        }
                        node_views
                    }),
            )
        })
}

// ─── Single proxy node row ────────────────────────────────────────────

fn proxy_node_row(
    theme: &Theme,
    strings: &I18nStrings,
    cx: &mut Context<crate::app::AppView>,
    node: &MockNode,
    is_current: bool,
    group_index: usize,
    node_index: usize,
) -> impl IntoElement + use<> {
    let name = node.name.to_string();
    let type_label = node.node_type.label();
    let type_icon = node.node_type.icon();

    let delay_text = match node.delay {
        Some(d) => format_delay(d),
        None => strings.proxy_delay_na.to_string(),
    };
    let delay_color = match node.delay {
        Some(d) if d < 100 => rgb(theme.status_success),
        Some(d) if d < 300 => rgb(theme.status_warning),
        Some(_) => rgb(theme.status_error),
        None => rgb(theme.text_disabled),
    };

    let sel_label = if is_current { strings.proxy_current } else { strings.proxy_select };
    let sel_bg = if is_current { rgb(theme.accent_muted) } else { rgba(0x00000000) };
    let sel_fg = if is_current { rgb(theme.accent) } else { rgb(theme.text_disabled) };

    let nm = name.clone();

    div()
        .id(("proxy-node-row", group_index * 100 + node_index))
        .flex()
        .items_center()
        .justify_between()
        .px(px(16.0))
        .py(px(10.0))
        .hover(|s| s.bg(rgb(theme.surface_variant)))
        .cursor_pointer()
        .on_click(cx.listener(move |this, _, _, cx| {
            let _ = this; let _ = &nm; cx.notify();
        }))
        .child(
            div().flex().items_center().gap(px(8.0)).flex_1().overflow_hidden()
                .child(div().w(px(16.0)).h(px(16.0)).flex().items_center().justify_center()
                    .child(div().w(px(8.0)).h(px(8.0)).rounded(px(4.0)).bg(if is_current { rgb(theme.accent) } else { rgba(0x00000000) })))
                .child(div().text_size(px(14.0)).flex_shrink_0().child(type_icon))
                .child(
                    div().flex().flex_col().gap(px(2.0)).overflow_hidden()
                        .child(div().text_size(px(13.0)).font_weight(FontWeight::MEDIUM).text_color(if is_current { rgb(theme.accent) } else { rgb(theme.text_primary) }).overflow_hidden().text_ellipsis().child(name.clone()))
                        .child(div().text_size(px(11.0)).text_color(rgb(theme.text_disabled)).child(type_label)),
                ),
        )
        .child(
            div().flex().items_center().gap(px(8.0)).flex_shrink_0()
                .child({
                    let nm2 = name.clone();
                    div()
                        .id(("proxy-node-delay-test", group_index * 100 + node_index))
                        .flex().items_center().gap(px(3.0)).px(px(8.0)).py(px(3.0)).rounded(px(4.0))
                        .bg(rgb(theme.surface_variant)).cursor_pointer().hover(|s| s.bg(rgb(theme.border)))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            let _ = this; let _ = &nm2; cx.notify();
                        }))
                        .child(div().text_size(px(12.0)).font_weight(FontWeight::MEDIUM).text_color(delay_color).child(delay_text.clone()))
                        .when(node.delay.is_some(), |s| s.child(div().text_size(px(11.0)).text_color(rgb(theme.text_disabled)).child("ms")))
                })
                .child(div().px(px(10.0)).py(px(4.0)).rounded(px(4.0)).text_size(px(11.0)).font_weight(FontWeight::MEDIUM).bg(sel_bg).text_color(sel_fg).child(sel_label)),
        )
}

// ─── Providers section ─────────────────────────────────────────────────

fn providers_section(
    theme: &Theme,
    strings: &I18nStrings,
    cx: &mut Context<crate::app::AppView>,
) -> impl IntoElement + use<> {
    let providers = build_mock_providers();

    div()
        .flex()
        .flex_col()
        .pt(px(16.0))
        .gap(px(8.0))
        .child(div().text_size(px(14.0)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(theme.text_secondary)).px(px(4.0)).child(strings.proxy_providers))
        .children({
            let mut provider_views = Vec::new();
            for (pi, p) in providers.iter().enumerate() {
                let name = p.name.to_string();
                let ncount = p.node_count;
                let upd = p.updated.to_string();
                let pn = name.clone();

                provider_views.push(
                    div()
                        .id(("proxy-provider-row", pi))
                        .flex().items_center().justify_between().px(px(16.0)).py(px(12.0))
                        .rounded(px(CARD_RADIUS)).bg(rgb(theme.surface)).border_1().border_color(rgb(theme.border_light))
                        .hover(|s| s.bg(rgb(theme.surface_variant))).cursor_pointer()
                        .on_click(cx.listener(move |this, _, _, cx| {
                            let _ = this; let _ = &pn; cx.notify();
                        }))
                        .child(
                            div().flex().flex_col().gap(px(2.0))
                                .child(div().text_size(px(13.0)).font_weight(FontWeight::MEDIUM).text_color(rgb(theme.text_primary)).child(name))
                                .child(
                                    div().flex().items_center().gap(px(6.0))
                                        .child(div().text_size(px(11.0)).text_color(rgb(theme.text_disabled)).child(format!("{} {}", ncount, strings.proxy_provider_nodes)))
                                        .child(div().w(px(3.0)).h(px(3.0)).rounded(px(1.5)).bg(rgb(theme.text_disabled)))
                                        .child(div().text_size(px(11.0)).text_color(rgb(theme.text_disabled)).child(upd)),
                                ),
                        )
                        .child(div().text_size(px(14.0)).text_color(rgb(theme.text_disabled)).child("\u{203A}")),
                );
            }
            provider_views
        })
}

// ─── Helpers ───────────────────────────────────────────────────────────

fn format_delay(ms: u64) -> String {
    if ms == 0 {
        "<1ms".into()
    } else if ms >= 1000 {
        format!("{:.1}s", ms as f64 / 1000.0)
    } else {
        format!("{ms}")
    }
}
