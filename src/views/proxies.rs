/// Proxies view — proxy group list, node management, speed test, provider list.

use crate::components::text_input::TextInput;
use crate::state::proxy::ProxyState;
use gpui::prelude::*;
use gpui::*;

use crate::i18n::I18nStrings;
use crate::theme::{Theme, CARD_RADIUS};

// ─── Data types ────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GroupType {
    Selector,
    URLTest,
    Fallback,
    LoadBalance,
}

#[derive(Clone, Copy, PartialEq, Eq)]
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
    expanded_map: &std::collections::HashMap<String, bool>,
    search: &Entity<TextInput>,
) -> impl IntoElement + use<> {
    // ── Real data from core (via bridge) ──
    let proxy_state = cx.global::<ProxyState>();
    let groups = build_groups_from_state(&proxy_state);

    let search_text = search.read(cx).text().to_string();
    let search_has_text = !search_text.is_empty();
    let expanded_map = expanded_map.clone();
    let search_rendered = search.update(cx, |t, cx| t.render(theme, cx));

    div()
        .flex()
        .flex_col()
        .size_full()
        .child(
            div()
                .flex()
                .items_center()
                .px(px(24.0))
                .py(px(12.0))
                .child(search_rendered),
        )
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
}

// ─── State → view conversion ────────────────────────────────────────────

/// Convert `ProxyState` data to `MockGroup` structure for rendering.
/// Returns groups in display order with delays from state.
fn build_groups_from_state(state: &ProxyState) -> Vec<MockGroup> {
    let mut result = Vec::new();
    for gname in &state.group_order {
        if let Some(group) = state.groups.get(gname) {
            let vg_type = match group.group_type {
                crate::state::proxy_models::GroupType::Select => GroupType::Selector,
                crate::state::proxy_models::GroupType::UrlTest => GroupType::URLTest,
                crate::state::proxy_models::GroupType::Fallback => GroupType::Fallback,
                crate::state::proxy_models::GroupType::LoadBalance | crate::state::proxy_models::GroupType::Relay => GroupType::LoadBalance,
            };
            let current_idx = group.all.iter().position(|p| {
                group.now.as_deref() == Some(&p.name)
            }).unwrap_or(0);
            let nodes: Vec<MockNode> = group.all.iter().map(|p| {
                let delay = state.delays.get(&p.name).and_then(|d| d.map(|ms| ms as u64));
                let ntype = node_type_from_str(&p.proxy_type);
                MockNode {
                    name: leak_str(&p.name),
                    node_type: ntype,
                    delay,
                }
            }).collect();
            result.push(MockGroup {
                name: leak_str(gname),
                group_type: vg_type,
                expanded: false,
                current_idx,
                nodes,
            });
        }
    }
    result
}

fn node_type_from_str(s: &str) -> NodeType {
    match s.to_lowercase().as_str() {
        "ss" | "shadowsocks" => NodeType::SS,
        "vmess" => NodeType::VMess,
        "trojan" => NodeType::Trojan,
        "hysteria2" | "hysteria" => NodeType::Hysteria2,
        "socks5" | "socks" => NodeType::Socks5,
        "http" | "https" => NodeType::HTTP,
        "wireguard" => NodeType::WireGuard,
        "tuic" => NodeType::TUIC,
        "ssh" => NodeType::SSH,
        "direct" => NodeType::Direct,
        "reject" | "block" => NodeType::Reject,
        _ => NodeType::VMess,
    }
}

/// Leak a string to get a &'static str (safe because these live for the app lifetime).
fn leak_str(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
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
                    let v = e.or_insert(expanded);
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
    _cx: &mut Context<crate::app::AppView>,
) -> impl IntoElement + use<> {
    div()
        .flex()
        .flex_col()
        .pt(px(16.0))
        .gap(px(8.0))
        .child(div().text_size(px(14.0)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(theme.text_secondary)).px(px(4.0)).child(strings.proxy_providers))
        .child(
            div()
                .flex().items_center().justify_center().px(px(16.0)).py(px(24.0))
                .rounded(px(CARD_RADIUS)).bg(rgb(theme.surface)).border_1().border_color(rgb(theme.border_light))
                .text_size(px(12.0)).text_color(rgb(theme.text_disabled))
                .child(strings.proxy_no_providers),
        )
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
