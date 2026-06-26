/// Profiles view — manage configuration profiles (local/subscription),
/// add subscriptions, YAML preview, overwrite management, sort/delete.
///
/// Structure:
/// - Toolbar: search + "Add Subscription" button
/// - Profile list: cards with type badge, name, URL, last update
/// - Add subscription panel (togglable)
/// - Detail panel: tabbed (Info / YAML Preview / Overwrite)

use crate::components::text_input::TextInput;
use crate::state::config::ConfigState;
use gpui::prelude::*;
use gpui::*;

use crate::i18n::I18nStrings;
use crate::theme::{Theme, CARD_RADIUS};

// ─── Data types ────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ProfileType {
    Local,
    Subscription,
}

/// Three ways to add a profile, mirroring FlClash.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AddMode {
    Qr,
    File,
    Url,
}

#[derive(Clone)]
pub struct MockProfile {
    pub name: &'static str,
    pub profile_type: ProfileType,
    pub url: &'static str,
    pub updated: &'static str,
    pub yaml_content: String,
}

// ─── Profile detail tabs ───────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DetailTab {
    Info,
    Yaml,
    Overwrite,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OverwriteSubTab {
    Standard,
    Script,
    Custom,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CustomCategory {
    Groups,
    Icons,
    Proxies,
    Providers,
    Rules,
}

// ─── Helpers ────────────────────────────────────────────────────────────

fn profile_type_badge_color(profile_type: ProfileType, theme: &Theme) -> Hsla {
    match profile_type {
        ProfileType::Local => rgb(theme.status_success).into(),
        ProfileType::Subscription => rgb(theme.accent).into(),
    }
}

fn profile_type_badge_bg(profile_type: ProfileType, theme: &Theme) -> Hsla {
    let mut c = profile_type_badge_color(profile_type, theme);
    c.a = 0.15;
    c
}

// ─── Main view ─────────────────────────────────────────────────────────

pub(super) fn profiles_view(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    selected_index: Option<usize>,
    show_add: bool,
    add_mode: AddMode,
    detail_tab: DetailTab,
    overwrite_sub_tab: OverwriteSubTab,
    search: &Entity<TextInput>,
    url_input: &Entity<TextInput>,
) -> impl IntoElement + use<> {
    // ── Real data from config state ──
    let config_state = cx.global::<ConfigState>();
    let profiles: Vec<MockProfile> = config_state.profiles.iter().map(|p| MockProfile {
        name: Box::leak(p.name.clone().into_boxed_str()),
        profile_type: match p.profile_type {
            crate::state::config::ProfileType::File => ProfileType::Local,
            crate::state::config::ProfileType::Url => ProfileType::Subscription,
        },
        url: Box::leak(p.path.clone().into_boxed_str()),
        updated: Box::leak(p.updated_at.map_or_else(|| "—".to_string(), |t| chrono::DateTime::from_timestamp(t, 0).map_or("—".to_string(), |dt| dt.format("%Y-%m-%d %H:%M").to_string())).into_boxed_str()),
        yaml_content: p.config_content.clone().unwrap_or_else(|| "# Config content not yet loaded\n".to_string()),
    }).collect();

    // Filter by search text
    let search_text = search.read(cx).text().to_string();
    let ft = search_text.to_lowercase();
    let filtered: Vec<(usize, MockProfile)> = profiles
        .iter()
        .enumerate()
        .filter(|(_, p)| {
            ft.is_empty()
                || p.name.to_lowercase().contains(&ft)
                || p.url.to_lowercase().contains(&ft)
        })
        .map(|(i, p)| (i, p.clone()))
        .collect();

    let has_search = !search_text.is_empty();

    // Selected profile
    let selected = selected_index
        .and_then(|si| profiles.get(si))
        .cloned();

    div()
        .flex()
        .flex_col()
        .size_full()
        .gap(px(8.0))
        .px(px(24.0))
        .py(px(8.0))
        // ── Toolbar ────────────────────────────────
        .child(render_toolbar(theme, cx, strings, &search_text, has_search, show_add, add_mode, search))
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
                                let search = search.clone();
                                move |_: &MouseDownEvent, _window, app| {
                                    search.update(app, |t, _| t.clear());
                                    entity.update(app, |this, _| {
                                        this.profiles_selected_index = None;
                                    });
                                }
                            })
                            .child("Clear"),
                    ),
            )
        })
        // ── Add subscription panel ────────────────
        .when(show_add, |s| {
            s.child(render_add_panel(theme, cx, strings, add_mode, url_input))
        })
        // ── Profile list ──────────────────────────
        .child(render_profile_list(theme, cx, strings, &filtered, selected_index))
        // ── Detail panel (when selected) ──────────
        .when_some(selected, |s, prof| {
            s.child(render_detail_panel(theme, cx, strings, &prof, detail_tab, overwrite_sub_tab))
        })
        .child(div().h(px(16.0)))
}

// ─── Toolbar ───────────────────────────────────────────────────────────

fn render_toolbar(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    _search_text: &str,
    _has_search: bool,
    show_add: bool,
    add_mode: AddMode,
    search: &Entity<TextInput>,
) -> impl IntoElement + use<> {
    let add_label = match add_mode {
        AddMode::Qr => strings.profiles_add_qr.to_string(),
        AddMode::File => strings.profiles_add_file.to_string(),
        AddMode::Url => strings.profiles_add_url.to_string(),
    };
    let cancel_label = strings.profiles_cancel.to_string();
    let search_rendered = search.update(cx, |t, cx| t.render(theme, cx));

    div()
        .flex()
        .items_center()
        .gap(px(8.0))
        .child(search_rendered)
        // Add subscription button — shows current mode label when panel is open
        .child(
            div()
                .flex()
                .items_center()
                .px(px(14.0))
                .py(px(8.0))
                .rounded(px(CARD_RADIUS))
                .bg(rgb(if show_add { theme.status_warning } else { theme.accent }))
                .cursor_pointer()
                .hover(|s| s.opacity(0.85))
                .on_any_mouse_down({
                    let entity = cx.entity();
                    move |_: &MouseDownEvent, _window, app| {
                        entity.update(app, |this, _| {
                            if this.profiles_show_add {
                                this.profiles_show_add = false;
                                this.profiles_add_mode = AddMode::Url;
                            } else {
                                this.profiles_show_add = true;
                            }
                        });
                    }
                })
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0xffffff))
                        .child(if show_add { cancel_label } else { add_label }),
                ),
        )
}

// ─── Add profile panel (3 modes: QR / File / URL) ────────────────────

fn render_add_panel(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    add_mode: AddMode,
    url_input: &Entity<TextInput>,
) -> impl IntoElement + use<> {
    let title = strings.profiles_add_title.to_string();
    let qr_label = strings.profiles_add_qr.to_string();
    let file_label = strings.profiles_add_file.to_string();
    let url_label_tab = strings.profiles_add_url.to_string();

    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .rounded(px(CARD_RADIUS))
        .bg(rgb(theme.surface))
        .border_1()
        .border_color(rgb(theme.accent))
        .overflow_hidden()
        // Title bar
        .child(
            div()
                .flex()
                .items_center()
                .px(px(16.0))
                .py(px(10.0))
                .bg(rgb(theme.surface_variant))
                .border_b_1()
                .border_color(rgb(theme.border_light))
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(theme.text_primary))
                        .child(title),
                ),
        )
        // Mode tab bar
        .child(
            div()
                .flex()
                .px(px(12.0))
                .pt(px(12.0))
                .gap(px(4.0))
                .child(render_add_mode_tab(theme, cx, &qr_label, AddMode::Qr, add_mode))
                .child(render_add_mode_tab(theme, cx, &file_label, AddMode::File, add_mode))
                .child(render_add_mode_tab(theme, cx, &url_label_tab, AddMode::Url, add_mode)),
        )
        // Mode content
        .child(match add_mode {
            AddMode::Qr => render_qr_panel(theme, strings).into_any_element(),
            AddMode::File => render_file_panel(theme, cx, strings).into_any_element(),
            AddMode::Url => render_url_panel(theme, cx, strings, url_input).into_any_element(),
        })
}

fn render_add_mode_tab(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    label: &str,
    mode: AddMode,
    active: AddMode,
) -> impl IntoElement + use<> {
    let is_active = active == mode;
    let label = label.to_string();

    div()
        .flex()
        .items_center()
        .px(px(14.0))
        .py(px(7.0))
        .rounded(px(6.0))
        .text_size(px(12.0))
        .font_weight(if is_active { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
        .text_color(if is_active { rgb(theme.accent) } else { rgb(theme.text_secondary) })
        .bg(if is_active {
            let mut c = rgb(theme.accent);
            c.a = 0.12;
            c
        } else {
            rgba(0x00000000)
        })
        .cursor_pointer()
        .hover(|s| {
            if !is_active {
                s.bg(rgb(theme.surface_variant))
            } else {
                s
            }
        })
        .on_any_mouse_down({
            let entity = cx.entity();
            move |_: &MouseDownEvent, _window, app| {
                entity.update(app, |this, _| {
                    this.profiles_add_mode = mode;
                });
            }
        })
        .child(label)
}

// ─── QR code panel ──────────────────────────────────────────────────────

fn render_qr_panel(
    theme: &Theme,
    strings: &I18nStrings,
) -> impl IntoElement + use<> {
    let desc = strings.profiles_add_qr_desc.to_string();

    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(16.0))
        .px(px(16.0))
        .py(px(32.0))
        .child(
            // Placeholder QR scan area — to be replaced with camera integration
            div()
                .w(px(180.0))
                .h(px(180.0))
                .rounded(px(12.0))
                .bg(rgb(theme.content_bg))
                .border_1()
                .border_color(rgb(theme.border_light))
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .text_size(px(32.0))
                                .text_color(rgb(theme.text_disabled))
                                .child("\u{25A3}"), // QR icon placeholder
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(rgb(theme.text_disabled))
                                .child(desc),
                        ),
                ),
        )
}

// ─── File import panel ──────────────────────────────────────────────────

fn render_file_panel(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement + use<> {
    let select_label = strings.profiles_add_file_select.to_string();
    let _selected_label = strings.profiles_add_file_selected.to_string();
    let save_label = strings.profiles_save.to_string();

    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .px(px(16.0))
        .py(px(12.0))
        .child(
            div()
                .text_size(px(11.0))
                .text_color(rgb(theme.text_secondary))
                .child("Select a YAML or JSON configuration file."),
        )
        .child(
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .px(px(14.0))
                        .py(px(8.0))
                        .rounded(px(6.0))
                        .bg(rgb(theme.accent))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0xffffff))
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.85))
                        .on_any_mouse_down({
                            let entity = cx.entity();
                            move |_: &MouseDownEvent, _window, app| {
                                // TODO: native file picker integration
                                entity.update(app, |_this, _| {
                                    // On file selected:
                                    // core::bridge::add_local_file(&path, app);
                                });
                            }
                        })
                        .child(select_label),
                )
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(rgb(theme.text_disabled))
                        .child("No file selected"),
                ),
        )
        // Action buttons
        .child(
            div().flex().gap(px(8.0)).justify_end()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .px(px(14.0))
                        .py(px(6.0))
                        .rounded(px(6.0))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(theme.text_secondary))
                        .bg(rgb(theme.surface_variant))
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.8))
                        .on_any_mouse_down({
                            let entity = cx.entity();
                            move |_: &MouseDownEvent, _window, app| {
                                entity.update(app, |this, _| {
                                    this.profiles_show_add = false;
                                    this.profiles_add_mode = AddMode::Url;
                                });
                            }
                        })
                        .child(strings.profiles_cancel.to_string()),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .px(px(14.0))
                        .py(px(6.0))
                        .rounded(px(6.0))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0xffffff))
                        .bg(rgb(theme.accent))
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.85))
                        .on_any_mouse_down({
                            let entity = cx.entity();
                            move |_: &MouseDownEvent, _window, app| {
                                entity.update(app, |this, _| {
                                    this.profiles_show_add = false;
                                    this.profiles_add_mode = AddMode::Url;
                                });
                            }
                        })
                        .child(save_label),
                ),
        )
}

// ─── URL import panel ───────────────────────────────────────────────────

fn render_url_panel(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    url_input: &Entity<TextInput>,
) -> impl IntoElement + use<> {
    let url_label = strings.profiles_url_label.to_string();
    let save_label = strings.profiles_save.to_string();
    let cancel_label = strings.profiles_cancel.to_string();
    let url_rendered = url_input.update(cx, |t, cx| t.render_plain(theme, cx));

    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .px(px(16.0))
        .py(px(12.0))
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .text_size(px(11.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(theme.text_secondary))
                        .child(url_label),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .rounded(px(6.0))
                        .bg(rgb(theme.content_bg))
                        .border_1()
                        .border_color(rgb(theme.border_light))
                        .child(url_rendered),
                ),
        )
        // Action buttons
        .child(
            div().flex().gap(px(8.0)).justify_end()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .px(px(14.0))
                        .py(px(6.0))
                        .rounded(px(6.0))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(theme.text_secondary))
                        .bg(rgb(theme.surface_variant))
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.8))
                        .on_any_mouse_down({
                            let entity = cx.entity();
                            move |_: &MouseDownEvent, _window, app| {
                                entity.update(app, |this, _| {
                                    this.profiles_show_add = false;
                                    this.profiles_add_mode = AddMode::Url;
                                });
                            }
                        })
                        .child(cancel_label),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .px(px(14.0))
                        .py(px(6.0))
                        .rounded(px(6.0))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0xffffff))
                        .bg(rgb(theme.accent))
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.85))
                        .on_any_mouse_down({
                            let entity = cx.entity();
                            let url_input = url_input.clone();
                            move |_: &MouseDownEvent, _window, app| {
                                let url = url_input.read(app).text().to_string();
                                let trimmed = url.trim().to_string();
                                entity.update(app, |this, _| {
                                    this.profiles_show_add = false;
                                    this.profiles_add_mode = AddMode::Url;
                                });
                                if !trimmed.is_empty() {
                                    let _ = crate::core::bridge::add_subscription(&trimmed, app);
                                }
                            }
                        })
                        .child(save_label),
                ),
        )
}

// ─── Profile list ──────────────────────────────────────────────────────

fn render_profile_list(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    filtered: &[(usize, MockProfile)],
    selected_index: Option<usize>,
) -> impl IntoElement + use<> {
    let empty = filtered.is_empty();

    div()
        .flex()
        .flex_col()
        .flex_1()
        .gap(px(6.0))
        .overflow_hidden()
        .when(empty, |s| {
            s.flex()
                .items_center()
                .justify_center()
                .py(px(48.0))
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(rgb(theme.text_disabled))
                        .child(strings.profiles_empty),
                )
        })
        .when(!empty, |s| {
            s.child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .overflow_hidden()
                    .children(
                        filtered
                            .iter()
                            .map(|(orig_idx, prof)| {
                                render_profile_card(theme, cx, strings, *orig_idx, prof, selected_index)
                            }),
                    ),
            )
        })
}

// ─── Profile card ──────────────────────────────────────────────────────

fn render_profile_card(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    orig_index: usize,
    prof: &MockProfile,
    selected_index: Option<usize>,
) -> impl IntoElement + use<> {
    let is_selected = selected_index == Some(orig_index);

    let type_label = match prof.profile_type {
        ProfileType::Local => strings.profiles_type_local,
        ProfileType::Subscription => strings.profiles_type_subscription,
    };
    let type_color = profile_type_badge_color(prof.profile_type, theme);
    let type_bg = profile_type_badge_bg(prof.profile_type, theme);

    let card_bg = if is_selected {
        rgb(theme.accent_muted)
    } else {
        rgb(theme.surface)
    };
    let card_border = if is_selected {
        rgb(theme.accent)
    } else {
        rgb(theme.border_light)
    };

    let update_label = strings.profiles_update.to_string();
    let edit_label = strings.profiles_edit.to_string();
    let delete_label = strings.profiles_delete.to_string();

    let is_sub = prof.profile_type == ProfileType::Subscription;
    let display_url = prof.url.to_string();
    let url = display_url.clone();

    div()
        .flex()
        .items_center()
        .justify_between()
        .px(px(14.0))
        .py(px(12.0))
        .rounded(px(CARD_RADIUS))
        .bg(card_bg)
        .border_1()
        .border_color(card_border)
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
                    this.profiles_selected_index = Some(orig_index);
                });
            }
        })
        // Left: type badge + info
        .child(
            div().flex().items_center().gap(px(12.0)).flex_1().overflow_hidden()
                // Type badge
                .child(
                    div()
                        .px(px(8.0))
                        .py(px(4.0))
                        .rounded(px(4.0))
                        .text_size(px(10.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(type_color)
                        .bg(type_bg)
                        .flex_shrink_0()
                        .child(type_label),
                )
                // Name + URL + updated
                .child(
                    div().flex().flex_col().gap(px(3.0)).overflow_hidden()
                        .child(
                            div()
                                .text_size(px(13.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(theme.text_primary))
                                .child(prof.name),
                        )
                        .when(is_sub, |s| {
                            s.child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(rgb(theme.text_disabled))
                                    .overflow_hidden()
                                    .text_ellipsis()
                                    .child(display_url),
                            )
                        })
                        .child(
                            div()
                                .text_size(px(10.0))
                                .text_color(rgb(theme.text_disabled))
                                .child(format!("{} {}", strings.profiles_updated, prof.updated)),
                        ),
                ),
        )
        // Right: action buttons
        .child(
            div().flex().items_center().gap(px(4.0)).flex_shrink_0()
                // Update button (subscription only)
                .when(is_sub, |s| {
                    let label = update_label.clone();
                    let sub_url = url.clone();
                    s.child(
                        div()
                            .flex()
                            .items_center()
                            .px(px(8.0))
                            .py(px(4.0))
                            .rounded(px(4.0))
                            .text_size(px(10.0))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(theme.accent))
                            .bg({
                                let mut c = rgb(theme.accent);
                                c.a = 0.12;
                                c
                            })
                            .cursor_pointer()
                            .hover(|s| s.opacity(0.8))
                            .on_any_mouse_down({
                                let entity = cx.entity();
                                let sub_url = sub_url.clone();
                                move |_: &MouseDownEvent, _window, app| {
                                    let path = sub_url.clone();
                                    entity.update(app, |this, _| {
                                        this.profiles_selected_index = Some(orig_index);
                                    });
                                    crate::core::bridge::update_subscription(&path, app);
                                }
                            })
                            .child(label),
                    )
                })
                // Edit button
                .child(
                    div()
                        .flex()
                        .items_center()
                        .px(px(8.0))
                        .py(px(4.0))
                        .rounded(px(4.0))
                        .text_size(px(10.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(theme.text_secondary))
                        .bg(rgb(theme.surface_variant))
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.8))
                        .on_any_mouse_down({
                            let entity = cx.entity();
                            move |_: &MouseDownEvent, _window, app| {
                                entity.update(app, |this, _| {
                                    this.profiles_selected_index = Some(orig_index);
                                    this.profiles_detail_tab = Some(DetailTab::Yaml);
                                });
                            }
                        })
                        .child(edit_label),
                )
                // Delete button
                .child(
                    div()
                        .flex()
                        .items_center()
                        .px(px(8.0))
                        .py(px(4.0))
                        .rounded(px(4.0))
                        .text_size(px(10.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(theme.status_error))
                        .bg({
                            let mut c = rgb(theme.status_error);
                            c.a = 0.12;
                            c
                        })
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.8))
                        .on_any_mouse_down({
                            let entity = cx.entity();
                            let del_url = url.clone();
                            move |_: &MouseDownEvent, _window, app| {
                                let path = del_url.clone();
                                entity.update(app, |_this, _| {
                                    // handled by bridge helper
                                });
                                crate::core::bridge::delete_profile(&path, orig_index, app);
                            }
                        })
                        .child(delete_label),
                ),
        )
}

// ─── Detail panel ──────────────────────────────────────────────────────

fn render_detail_panel(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    prof: &MockProfile,
    detail_tab: DetailTab,
    overwrite_sub_tab: OverwriteSubTab,
) -> impl IntoElement + use<> {
    let type_label = match prof.profile_type {
        ProfileType::Local => strings.profiles_type_local,
        ProfileType::Subscription => strings.profiles_type_subscription,
    };
    let type_color = profile_type_badge_color(prof.profile_type, theme);

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
                .gap(px(8.0))
                .px(px(16.0))
                .py(px(10.0))
                .bg(rgb(theme.surface_variant))
                .border_b_1()
                .border_color(rgb(theme.border_light))
                .child(
                    div()
                        .px(px(6.0))
                        .py(px(2.0))
                        .rounded(px(3.0))
                        .text_size(px(10.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(type_color)
                        .bg({
                            let mut c = type_color;
                            c.a = 0.15;
                            c
                        })
                        .child(type_label),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(theme.text_primary))
                        .child(prof.name),
                ),
        )
        // Tab bar
        .child(render_detail_tabs(theme, cx, strings, detail_tab))
        // Tab content
        .child(match detail_tab {
            DetailTab::Info => render_info_tab(theme, prof, strings).into_any_element(),
            DetailTab::Yaml => render_yaml_tab(theme, prof, strings).into_any_element(),
            DetailTab::Overwrite => render_overwrite_tab(theme, cx, strings, overwrite_sub_tab).into_any_element(),
        })
}

// ─── Detail tab bar ────────────────────────────────────────────────────

fn render_detail_tabs(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    active: DetailTab,
) -> impl IntoElement + use<> {
    let info_label = strings.profiles_detail_tab_info.to_string();
    let yaml_label = strings.profiles_detail_tab_yaml.to_string();
    let overwrite_label = strings.profiles_detail_tab_overwrite.to_string();

    div()
        .flex()
        .border_b_1()
        .border_color(rgb(theme.border_light))
        .child(render_tab_button(theme, cx, &info_label, DetailTab::Info, active))
        .child(render_tab_button(theme, cx, &yaml_label, DetailTab::Yaml, active))
        .child(render_tab_button(theme, cx, &overwrite_label, DetailTab::Overwrite, active))
}

fn render_tab_button(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    label: &str,
    tab: DetailTab,
    active: DetailTab,
) -> impl IntoElement + use<> {
    let is_active = active == tab;
    let label = label.to_string();
    let text_color = if is_active {
        rgb(theme.accent)
    } else {
        rgb(theme.text_secondary)
    };

    div()
        .flex()
        .items_center()
        .px(px(16.0))
        .py(px(8.0))
        .text_size(px(12.0))
        .font_weight(if is_active { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
        .text_color(text_color)
        .border_b_2()
        .border_color(if is_active { rgb(theme.accent) } else { rgba(0x00000000) })
        .cursor_pointer()
        .hover(|s| {
            if !is_active {
                s.bg(rgb(theme.surface_variant))
            } else {
                s
            }
        })
        .on_any_mouse_down({
            let entity = cx.entity();
            move |_: &MouseDownEvent, _window, app| {
                entity.update(app, |this, _| {
                    this.profiles_detail_tab = Some(tab);
                });
            }
        })
        .child(label)
}

// ─── Info tab ──────────────────────────────────────────────────────────

fn render_info_tab(
    theme: &Theme,
    prof: &MockProfile,
    strings: &I18nStrings,
) -> impl IntoElement + use<> {
    let is_sub = prof.profile_type == ProfileType::Subscription;

    div()
        .flex()
        .flex_col()
        .gap(px(10.0))
        .p(px(16.0))
        .child(
            div().flex().items_center().gap(px(24.0))
                .child(info_row(theme, strings.profiles_name_label, prof.name))
                .child(info_row(theme, strings.profiles_updated, prof.updated)),
        )
        .when(is_sub, |s| {
            s.child(
                div().flex().flex_col().gap(px(2.0))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(theme.text_disabled))
                            .child(strings.profiles_url_label),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(theme.text_primary))
                            .font_weight(FontWeight::MEDIUM)
                            .child(prof.url),
                    ),
            )
        })
}

fn info_row(theme: &Theme, label: &str, value: &str) -> impl IntoElement {
    let label = label.to_string();
    let value = value.to_string();
    div()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .min_w(px(120.0))
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
                .child(value),
        )
}

// ─── YAML preview tab ──────────────────────────────────────────────────

fn render_yaml_tab(
    theme: &Theme,
    prof: &MockProfile,
    _strings: &I18nStrings,
) -> impl IntoElement + use<> {
    let yaml_content = prof.yaml_content.to_string();

    div()
        .flex()
        .flex_col()
        .overflow_hidden()
        .p(px(16.0))
        .bg(rgb(theme.content_bg))
        .child(
            div()
                .flex()
                .flex_col()
                .overflow_hidden()
                .max_h(px(300.0))
                .rounded(px(4.0))
                .p(px(12.0))
                .bg(rgb(theme.surface))
                .border_1()
                .border_color(rgb(theme.border_light))
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(rgb(theme.text_primary))
                        .child(yaml_content),
                ),
        )
}

// ─── Overwrite tab ─────────────────────────────────────────────────────

fn render_overwrite_tab(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    sub_tab: OverwriteSubTab,
) -> impl IntoElement + use<> {
    let standard_label = strings.profiles_overwrite_standard.to_string();
    let script_label = strings.profiles_overwrite_script.to_string();
    let custom_label = strings.profiles_overwrite_custom.to_string();

    div()
        .flex()
        .flex_col()
        .child(
            // Sub-tab bar
            div()
                .flex()
                .px(px(16.0))
                .pt(px(8.0))
                .gap(px(4.0))
                .child(render_ow_sub_tab(theme, cx, &standard_label, OverwriteSubTab::Standard, sub_tab))
                .child(render_ow_sub_tab(theme, cx, &script_label, OverwriteSubTab::Script, sub_tab))
                .child(render_ow_sub_tab(theme, cx, &custom_label, OverwriteSubTab::Custom, sub_tab)),
        )
        // Content by sub-tab
        .child(match sub_tab {
            OverwriteSubTab::Standard => render_standard_overwrite(theme, strings).into_any_element(),
            OverwriteSubTab::Script => render_script_overwrite(theme, strings).into_any_element(),
            OverwriteSubTab::Custom => render_custom_overwrite(theme, cx, strings).into_any_element(),
        })
}

fn render_ow_sub_tab(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    label: &str,
    tab: OverwriteSubTab,
    active: OverwriteSubTab,
) -> impl IntoElement + use<> {
    let is_active = active == tab;
    let label = label.to_string();

    div()
        .flex()
        .items_center()
        .px(px(12.0))
        .py(px(6.0))
        .rounded(px(6.0))
        .text_size(px(11.0))
        .font_weight(if is_active { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
        .text_color(if is_active { rgb(theme.accent) } else { rgb(theme.text_secondary) })
        .bg(if is_active {
            let mut c = rgb(theme.accent);
            c.a = 0.12;
            c
        } else {
            rgba(0x00000000)
        })
        .cursor_pointer()
        .hover(|s| {
            if !is_active {
                s.bg(rgb(theme.surface_variant))
            } else {
                s
            }
        })
        .on_any_mouse_down({
            let entity = cx.entity();
            move |_: &MouseDownEvent, _window, app| {
                entity.update(app, |this, _| {
                    this.profiles_overwrite_sub_tab = Some(tab);
                });
            }
        })
        .child(label)
}

// ─── Overwrite: Standard ───────────────────────────────────────────────

fn render_standard_overwrite(
    theme: &Theme,
    _strings: &I18nStrings,
) -> impl IntoElement + use<> {
    div()
        .flex()
        .flex_col()
        .gap(px(10.0))
        .p(px(16.0))
        .child(
            div()
                .text_size(px(12.0))
                .text_color(rgb(theme.text_secondary))
                .child("Standard overwrite allows overriding proxy, rule, and DNS settings from the profile."),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(ow_toggle_row(theme, "Allow LAN", true))
                .child(ow_toggle_row(theme, "IPv6", false))
                .child(ow_toggle_row(theme, "Unified Delay", true))
                .child(ow_toggle_row(theme, "TCP Concurrent", false)),
        )
}

fn ow_toggle_row(theme: &Theme, label: &str, enabled: bool) -> impl IntoElement {
    let label = label.to_string();
    div()
        .flex()
        .items_center()
        .justify_between()
        .px(px(8.0))
        .py(px(6.0))
        .child(
            div()
                .text_size(px(12.0))
                .text_color(rgb(theme.text_primary))
                .child(label),
        )
        .child(
            div()
                .w(px(36.0))
                .h(px(20.0))
                .rounded(px(10.0))
                .bg(if enabled { rgb(theme.status_success) } else { rgb(theme.text_disabled) })
                .child(
                    div()
                        .w(px(16.0))
                        .h(px(16.0))
                        .mt(px(2.0))
                        .rounded(px(8.0))
                        .bg(rgb(0xffffff))
                        .when(enabled, |s| s.ml(px(18.0)))
                        .when(!enabled, |s| s.ml(px(2.0))),
                ),
        )
}

// ─── Overwrite: Script ─────────────────────────────────────────────────

fn render_script_overwrite(
    theme: &Theme,
    _strings: &I18nStrings,
) -> impl IntoElement + use<> {
    div()
        .flex()
        .flex_col()
        .gap(px(10.0))
        .p(px(16.0))
        .child(
            div()
                .text_size(px(12.0))
                .text_color(rgb(theme.text_secondary))
                .child("Write JavaScript to dynamically modify the configuration before it is applied."),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .rounded(px(4.0))
                .bg(rgb(theme.content_bg))
                .border_1()
                .border_color(rgb(theme.border_light))
                .overflow_hidden()
                .child(
                    div()
                        .px(px(12.0))
                        .py(px(8.0))
                        .bg(rgb(theme.surface_variant))
                        .border_b_1()
                        .border_color(rgb(theme.border_light))
                        .text_size(px(10.0))
                        .text_color(rgb(theme.text_disabled))
                        .child("override.js"),
                )
                .child(
                    div()
                        .p(px(12.0))
                        .text_size(px(11.0))
                        .text_color(rgb(theme.text_primary))
                        .child("// JavaScript override script\nfunction main(config) {\n  // Modify config object here\n  config['log-level'] = 'debug';\n  return config;\n}\n"),
                ),
        )
}

// ─── Overwrite: Custom ─────────────────────────────────────────────────

fn render_custom_overwrite(
    theme: &Theme,
    _cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement + use<> {
    let categories: &[(CustomCategory, &str)] = &[
        (CustomCategory::Groups, strings.profiles_overwrite_groups),
        (CustomCategory::Icons, strings.profiles_overwrite_icons),
        (CustomCategory::Proxies, strings.profiles_overwrite_proxies),
        (CustomCategory::Providers, strings.profiles_overwrite_providers),
        (CustomCategory::Rules, strings.profiles_overwrite_rules),
    ];

    div()
        .flex()
        .flex_col()
        .gap(px(6.0))
        .p(px(16.0))
        .children(categories.iter().map(|(_cat, label)| {
            let label = label.to_string();
            div()
                .flex()
                .items_center()
                .justify_between()
                .px(px(12.0))
                .py(px(10.0))
                .rounded(px(6.0))
                .bg(rgb(theme.surface))
                .border_1()
                .border_color(rgb(theme.border_light))
                .cursor_pointer()
                .hover(|s| s.bg(rgb(theme.surface_variant)))
                .child(
                    div()
                        .text_size(px(13.0))
                        .text_color(rgb(theme.text_primary))
                        .child(label),
                )
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(rgb(theme.text_disabled))
                        .child("\u{203A}"),
                )
        }))
}
