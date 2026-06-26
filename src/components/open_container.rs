#![allow(dead_code)]
/// Open Container — an expandable/collapsible section.
///
/// Architecture:
/// - Header row with title + chevron indicator
/// - Content area conditionally rendered based on `expanded` flag
/// - Click handler on header toggles the expanded state

use gpui::prelude::*;
use gpui::*;

use crate::theme::Theme;

/// Parameters controlling the open container appearance and behaviour.
pub struct OpenContainerParams {
    /// Title shown in the header.
    pub title: String,
    /// Whether the container content is currently expanded.
    pub expanded: bool,
}

/// Render an expandable container with chevron indicator.
///
/// `content` is a closure that builds the expandable body.
/// `on_toggle` is invoked when the header is clicked.
pub fn open_container(
    params: &OpenContainerParams,
    theme: &Theme,
    content: impl FnOnce() -> AnyElement,
    on_toggle: Box<dyn Fn(&ClickEvent, &mut Window, &mut App)>,
) -> impl IntoElement {
    let title = params.title.clone();
    let expanded = params.expanded;
    let chevron = if expanded { "\u{25B2}" } else { "\u{25BC}" }; // ▲ / ▼

    div()
        .flex()
        .flex_col()
        .rounded(px(12.0))
        .bg(rgb(theme.surface))
        .border_1()
        .border_color(rgb(theme.border_light))
        .child(
            // ── Header (clickable) ──
            div()
                .id("open-container-header")
                .flex()
                .items_center()
                .justify_between()
                .px(px(16.0))
                .py(px(12.0))
                .cursor_pointer()
                .hover(|s| s.bg(rgb(theme.surface_variant)))
                .rounded(px(12.0))
                .on_click(on_toggle)
                .child(
                    div()
                        .text_size(px(14.0))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(theme.text_primary))
                        .child(title),
                )
                .child(
                    div()
                        .text_size(px(10.0))
                        .text_color(rgb(theme.text_secondary))
                        .child(chevron.to_string()),
                ),
        )
        .when(expanded, |s| {
            s.child(
                div()
                    .pt(px(4.0))
                    .pb(px(12.0))
                    .px(px(16.0))
                    .child(content()),
            )
        })
}
