/// Resources (资源管理) page — GeoIP, GeoSite, MMDB, ASN file management.
///
/// Based on FlClash `lib/views/resources.dart`.
/// Shows a list of Geo resource files with their current download URL,
/// local file info, and actions to edit the URL or trigger a sync.

use crate::components::text_input::TextInput;
use gpui::prelude::*;
use gpui::*;

use crate::i18n::I18nStrings;
use crate::theme::{Theme, CARD_RADIUS};

// ─── Resource type enumeration ────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ResourceType {
    GeoIP,
    GeoSite,
    MMDB,
    ASN,
}

impl ResourceType {
    fn label(self, strings: &I18nStrings) -> &'static str {
        match self {
            ResourceType::GeoIP => strings.resources_geoip,
            ResourceType::GeoSite => strings.resources_geosite,
            ResourceType::MMDB => strings.resources_mmdb,
            ResourceType::ASN => strings.resources_asn,
        }
    }

    fn file_name(self) -> &'static str {
        match self {
            ResourceType::GeoIP => "geoip.dat",
            ResourceType::GeoSite => "geosite.dat",
            ResourceType::MMDB => "geoip.metadb",
            ResourceType::ASN => "GeoLite2-ASN.mmdb",
        }
    }

    fn default_url(self) -> &'static str {
        match self {
            ResourceType::GeoIP => {
                "https://github.com/MetaCubeX/meta-rules-dat/releases/download/latest/geoip.dat"
            }
            ResourceType::GeoSite => {
                "https://github.com/MetaCubeX/meta-rules-dat/releases/download/latest/geosite.dat"
            }
            ResourceType::MMDB => {
                "https://github.com/MetaCubeX/meta-rules-dat/releases/download/latest/geoip.metadb"
            }
            ResourceType::ASN => {
                "https://github.com/MetaCubeX/meta-rules-dat/releases/download/latest/GeoLite2-ASN.mmdb"
            }
        }
    }
}

/// All four resource types in display order.
const ALL_RESOURCE_TYPES: &[ResourceType] = &[
    ResourceType::GeoIP,
    ResourceType::GeoSite,
    ResourceType::MMDB,
    ResourceType::ASN,
];

// ─── Mock resource data ────────────────────────────────────────────

/// A mock resource entry for the UI. In production this comes from the core engine.
#[derive(Clone, Debug)]
pub(crate) struct MockResource {
    /// Which Geo resource this is.
    resource_type: ResourceType,
    /// The current download URL (user-customizable).
    pub(crate) url: String,
    /// Whether the URL has been modified from the default.
    _url_modified: bool,
    /// File size on disk in bytes, or None if the file does not exist locally.
    file_size: Option<u64>,
    /// Last modified time as a display string (e.g. "2 hours ago").
    last_modified: Option<String>,
}

fn mock_resources() -> Vec<MockResource> {
    ALL_RESOURCE_TYPES
        .iter()
        .map(|rt| MockResource {
            resource_type: *rt,
            url: rt.default_url().to_string(),
            _url_modified: false,
            file_size: Some(match rt {
                ResourceType::GeoIP => 8_542_312,
                ResourceType::GeoSite => 4_210_876,
                ResourceType::MMDB => 12_345_678,
                ResourceType::ASN => 6_789_012,
            }),
            last_modified: Some(String::from(match rt {
                ResourceType::GeoIP => "3 hours ago",
                ResourceType::GeoSite => "2 hours ago",
                ResourceType::MMDB => "1 day ago",
                ResourceType::ASN => "5 hours ago",
            })),
        })
        .collect()
}

// ─── State for the Resources page ──────────────────────────────────

/// Resources page state stored on AppView.
pub struct ResourcesState {
    /// Current resource entries.
    pub items: Vec<MockResource>,
    /// Which resource index is currently being edited (URL dialog open).
    pub editing_index: Option<usize>,
    /// Which resource index is currently syncing (shows spinner).
    pub syncing_index: Option<usize>,
}

impl ResourcesState {
    pub fn new() -> Self {
        Self {
            items: mock_resources(),
            editing_index: None,
            syncing_index: None,
        }
    }
}

// ─── Helpers ───────────────────────────────────────────────────────

/// Format file size to human-readable string.
fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Truncate a URL for display, keeping the first part and last segment.
fn truncate_url(url: &str, max_len: usize) -> String {
    if url.len() <= max_len {
        return url.to_string();
    }
    let prefix_len = max_len.saturating_sub(10).min(url.len());
    let prefix = &url[..prefix_len];
    let suffix = &url[url.len().saturating_sub(7)..];
    format!("{}...{}", prefix, suffix)
}

// ─── Main view ─────────────────────────────────────────────────────

pub(super) fn resources_view(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    state: &ResourcesState,
    edit_input: &Entity<TextInput>,
) -> impl IntoElement + use<> {
    let items_len = state.items.len();
    let editing_index = state.editing_index;
    let syncing_index = state.syncing_index;

    let empty_label = strings.resources_empty.to_string();
    let edit_input_clone = edit_input.clone();

    div()
        .flex()
        .flex_col()
        .w_full()
        .px(px(24.0))
        .py(px(16.0))
        .gap(px(12.0))
        .children(
            (0..items_len).map(move |idx| {
                let is_editing = editing_index == Some(idx);
                let is_syncing = syncing_index == Some(idx);

                if is_editing {
                    let resource_type = state
                        .items
                        .get(idx)
                        .map(|r| r.resource_type)
                        .unwrap_or(ResourceType::GeoIP);
                    render_edit_url_dialog(theme, cx, strings, idx, resource_type, &edit_input_clone)
                        .into_any_element()
                } else {
                    let item = state.items.get(idx).cloned();
                    render_resource_card(theme, cx, strings, idx, is_syncing, item)
                        .into_any_element()
                }
            }),
        )
        .when(items_len == 0, |this| {
            this.child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .justify_center()
                    .py(px(60.0))
                    .child(
                        div()
                            .text_size(px(14.0))
                            .text_color(rgb(theme.text_disabled))
                            .child(empty_label),
                    ),
            )
        })
}

// ─── Resource card ─────────────────────────────────────────────────

fn render_resource_card(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    index: usize,
    is_syncing: bool,
    item: Option<MockResource>,
) -> impl IntoElement + use<> {
    let resource = match &item {
        Some(r) => r,
        None => {
            return div().into_any_element();
        }
    };

    let entity = cx.entity();

    let type_label = resource.resource_type.label(strings).to_string();
    let file_name = resource.resource_type.file_name().to_string();
    let size_str = resource
        .file_size
        .map(format_file_size)
        .unwrap_or_else(|| strings.resources_no_file.to_string());
    let time_str = resource
        .last_modified
        .clone()
        .unwrap_or_else(|| strings.resources_unknown.to_string());
    let url_display = truncate_url(&resource.url, 60);

    let syncing_label = strings.resources_syncing.to_string();
    let edit_label = strings.resources_edit_url.to_string();
    let sync_label = strings.resources_sync.to_string();
    let file_size_label = strings.resources_file_size.to_string();
    let last_updated_label = strings.resources_last_updated.to_string();
    let url_label = strings.resources_url_label.to_string();

    let entity_clone = entity.clone();

    div()
        .flex()
        .flex_col()
        .rounded(px(CARD_RADIUS))
        .bg(rgb(theme.surface))
        .border_1()
        .border_color(rgb(theme.border_light))
        .overflow_hidden()
        .child(
            // ── Card body ──
            div()
                .flex()
                .flex_col()
                .px(px(16.0))
                .py(px(14.0))
                .gap(px(10.0))
                // ── Header: icon + type label + file name ──
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(8.0))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(28.0))
                                .h(px(28.0))
                                .rounded(px(6.0))
                                .bg(rgba(theme.accent & 0x00FFFFFF | 0x15000000))
                                .child(
                                    div()
                                        .text_size(px(14.0))
                                        .text_color(rgb(theme.accent))
                                        .child("\u{1F4E6}"), // 📦
                                ),
                        )
                        .child(
                            div()
                                .text_size(px(15.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(theme.text_primary))
                                .child(type_label),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(rgb(theme.text_disabled))
                                .ml(px(4.0))
                                .child(format!("({})", file_name)),
                        ),
                )
                // ── File info row ──
                .child(
                    div()
                        .flex()
                        .gap(px(24.0))
                        .child(
                            div().flex().flex_col().gap(px(2.0))
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(rgb(theme.text_disabled))
                                        .child(file_size_label),
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(rgb(theme.text_secondary))
                                        .child(size_str),
                                ),
                        )
                        .child(
                            div().flex().flex_col().gap(px(2.0))
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(rgb(theme.text_disabled))
                                        .child(last_updated_label),
                                )
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .text_color(rgb(theme.text_secondary))
                                        .child(time_str),
                                ),
                        ),
                )
                // ── URL row ──
                .child(
                    div().flex().flex_col().gap(px(2.0))
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(rgb(theme.text_disabled))
                                .child(url_label),
                        )
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(theme.text_secondary))
                                .font_family("monospace")
                                .child(url_display),
                        ),
                ),
        )
        .child(
            // ── Action bar ──
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .px(px(16.0))
                .py(px(10.0))
                .border_t_1()
                .border_color(rgb(theme.border_light))
                .bg(rgb(theme.surface_variant))
                // Edit URL button
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(4.0))
                        .px(px(12.0))
                        .py(px(6.0))
                        .rounded(px(6.0))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(theme.text_secondary))
                        .bg(rgb(theme.surface))
                        .border_1()
                        .border_color(rgb(theme.border_light))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(theme.surface_variant)))
                        .on_any_mouse_down({
                            let entity = entity_clone.clone();
                            move |_: &MouseDownEvent, _window, app| {
                                entity.update(app, |this, cx| {
                                    let url = this
                                        .resources_state
                                        .items
                                        .get(index)
                                        .map(|r| r.url.clone())
                                        .unwrap_or_default();
                                    this.resources_state.editing_index = Some(index);
                                    this.resources_edit_input.update(cx, |t: &mut TextInput, _| t.set_text(url));
                                });
                                app.refresh_windows();
                            }
                        })
                        .child(edit_label),
                )
                // Sync button
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap(px(4.0))
                        .px(px(12.0))
                        .py(px(6.0))
                        .rounded(px(6.0))
                        .text_size(px(12.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(if is_syncing {
                            rgb(theme.text_disabled)
                        } else {
                            rgb(theme.accent)
                        })
                        .bg(rgba(theme.accent & 0x00FFFFFF | 0x10000000))
                        .when(!is_syncing, |s| {
                            s.cursor_pointer()
                                .hover(|s| s.bg(rgba(theme.accent & 0x00FFFFFF | 0x18000000)))
                        })
                        .on_any_mouse_down({
                            let entity = entity_clone.clone();
                            move |_: &MouseDownEvent, _window, app| {
                                entity.update(app, |this, _| {
                                    // Toggle sync state and update mock timestamp.
                                    if this.resources_state.syncing_index == Some(index) {
                                        this.resources_state.syncing_index = None;
                                        if let Some(item) =
                                            this.resources_state.items.get_mut(index)
                                        {
                                            item.last_modified =
                                                Some(String::from("just now"));
                                        }
                                    } else if this.resources_state.syncing_index.is_none()
                                    {
                                        this.resources_state.syncing_index = Some(index);
                                    }
                                });
                                app.refresh_windows();
                            }
                        })
                        .child(if is_syncing {
                            syncing_label.into_any_element()
                        } else {
                            sync_label.into_any_element()
                        }),
                ),
        )
        .into_any_element()
}

// ─── Edit URL dialog (inline card) ─────────────────────────────────

fn render_edit_url_dialog(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
    index: usize,
    resource_type: ResourceType,
    edit_input: &Entity<TextInput>,
) -> impl IntoElement + use<> {
    let entity = cx.entity();

    let type_label = resource_type.label(strings);
    let title_str = format!("{} — {}", type_label, strings.resources_edit_url);
    let url_label = strings.resources_url_label.to_string();
    let save_label = strings.resources_save.to_string();
    let cancel_label = strings.resources_cancel.to_string();
    let reset_label = strings.resources_reset_default.to_string();

    let entity_clone = entity.clone();
    let entity_clone2 = entity.clone();
    let entity_clone3 = entity.clone();
    let edit_rendered = edit_input.update(cx, |t, cx| t.render_plain(theme, cx));

    div()
        .flex()
        .flex_col()
        .rounded(px(CARD_RADIUS))
        .bg(rgb(theme.surface))
        .border_1()
        .border_color(rgb(theme.accent))
        .overflow_hidden()
        .child(
            // ── Header ──
            div()
                .flex()
                .items_center()
                .justify_between()
                .px(px(16.0))
                .py(px(12.0))
                .border_b_1()
                .border_color(rgb(theme.border_light))
                .child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(theme.text_primary))
                        .child(title_str),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .w(px(24.0))
                        .h(px(24.0))
                        .rounded(px(4.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(theme.surface_variant)))
                        .on_any_mouse_down({
                            let entity = entity_clone.clone();
                            move |_: &MouseDownEvent, _window, app| {
                                entity.update(app, |this, cx| {
                                    this.resources_state.editing_index = None;
                                    this.resources_edit_input.update(cx, |t: &mut TextInput, _| t.clear());
                                });
                                app.refresh_windows();
                            }
                        })
                        .child(
                            div()
                                .text_size(px(14.0))
                                .text_color(rgb(theme.text_secondary))
                                .child("\u{2715}"), // ✕
                        ),
                ),
        )
        .child(
            // ── Body ──
            div()
                .flex()
                .flex_col()
                .px(px(16.0))
                .py(px(14.0))
                .gap(px(10.0))
                .child(
                    div().flex().flex_col().gap(px(4.0))
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(theme.text_secondary))
                                .child(url_label),
                        )
                        .child(
                            div()
                                .rounded(px(6.0))
                                .border_1()
                                .border_color(rgb(theme.border))
                                .bg(rgb(theme.content_bg))
                                .min_h(px(36.0))
                                .child(edit_rendered),
                        ),
                )
                // ── Actions ──
                .child(
                    div().flex().items_center().gap(px(8.0))
                        // Save button
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .px(px(14.0))
                                .py(px(7.0))
                                .rounded(px(6.0))
                                .text_size(px(13.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(theme.content_bg))
                                .bg(rgb(theme.accent))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(theme.accent_hover)))
                                .on_any_mouse_down({
                                    let entity = entity.clone();
                                    move |_: &MouseDownEvent, _window, app| {
                                        entity.update(app, |this, cx| {
                                            let new_url =
                                                this.resources_edit_input.read(cx).text().to_string();
                                            if let Some(item) =
                                                this.resources_state.items.get_mut(index)
                                            {
                                                item.url = new_url;
                                                item._url_modified = true;
                                            }
                                            this.resources_state.editing_index = None;
                                            this.resources_edit_input.update(cx, |t: &mut TextInput, _| t.clear());
                                        });
                                        app.refresh_windows();
                                    }
                                })
                                .child(save_label),
                        )
                        // Reset to default button
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .px(px(14.0))
                                .py(px(7.0))
                                .rounded(px(6.0))
                                .text_size(px(13.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(theme.text_secondary))
                                .bg(rgb(theme.surface_variant))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(theme.border)))
                                .on_any_mouse_down({
                                    let entity = entity_clone2.clone();
                                    move |_: &MouseDownEvent, _window, app| {
                                        entity.update(app, |this, cx| {
                                            this.resources_edit_input.update(cx, |t: &mut TextInput, _| {
                                                t.set_text(resource_type.default_url());
                                            });
                                        });
                                        app.refresh_windows();
                                    }
                                })
                                .child(reset_label),
                        )
                        .child(div().flex_1())
                        // Cancel button
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(6.0))
                                .px(px(14.0))
                                .py(px(7.0))
                                .rounded(px(6.0))
                                .text_size(px(13.0))
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(theme.text_secondary))
                                .bg(rgb(theme.surface_variant))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(theme.border)))
                                .on_any_mouse_down({
                                    let entity = entity_clone3.clone();
                                    move |_: &MouseDownEvent, _window, app| {
                                        entity.update(app, |this, cx| {
                                            this.resources_state.editing_index = None;
                                            this.resources_edit_input.update(cx, |t: &mut TextInput, _| t.clear());
                                        });
                                        app.refresh_windows();
                                    }
                                })
                                .child(cancel_label),
                        ),
                ),
        )
}
