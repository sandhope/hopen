/// Shared helper components used across multiple view modules.

use gpui::*;

use crate::theme::Theme;

/// A reusable placeholder section with title and description.
pub(super) fn placeholder_section(title: &str, description: &str, theme: &Theme) -> impl IntoElement {
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
pub(super) fn settings_item(title: &str, subtitle: &str, theme: &Theme) -> impl IntoElement {
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

/// Back button that returns to the main settings list.
pub(super) fn back_button(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
) -> impl IntoElement {
    div()
        .id("btn-back-settings")
        .flex()
        .items_center()
        .justify_center()
        .w(px(36.0))
        .h(px(36.0))
        .rounded(px(8.0))
        .cursor_pointer()
        .bg(rgba(0x00000000))
        .hover(|s| s.bg(rgb(theme.surface)))
        .on_click(cx.listener(|this, _, _, cx| {
            this.tools_sub_page = None;
            cx.notify();
        }))
        .child(
            svg()
                .path("icon/arrow-back.svg")
                .size(px(22.0))
                .text_color(rgb(theme.accent)),
        )
}
