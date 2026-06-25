/// Search Input Field — a pre-styled search input with icon and clear button.
///
/// Architecture:
/// - Search icon (🔍) as prefix
/// - Clear button (✕) as suffix when text is non-empty
/// - Rounded pill shape with accent border on focus
///
/// Usage:
/// ```ignore
/// search_input_field(
///     &SearchInputFieldParams {
///         value: &search_text,
///         placeholder: "Search...",
///         focused: is_focused,
///     },
///     theme,
/// )
/// ```

use gpui::prelude::*;
use gpui::*;

use crate::theme::Theme;

/// Parameters for the search input field.
pub struct SearchInputFieldParams<'a> {
    /// Current search text.
    pub value: &'a str,
    /// Placeholder text when empty.
    pub placeholder: &'a str,
    /// Whether the input has focus.
    pub focused: bool,
}

/// Render a search input field with icon and clear button.
pub fn search_input_field(params: &SearchInputFieldParams, theme: &Theme) -> impl IntoElement {
    let border_color = if params.focused {
        rgb(theme.accent)
    } else {
        rgb(theme.border_light)
    };

    let text_color = if params.value.is_empty() {
        rgb(theme.text_disabled)
    } else {
        rgb(theme.text_primary)
    };

    let display_text = if params.value.is_empty() {
        params.placeholder.to_string()
    } else {
        params.value.to_string()
    };

    let has_text = !params.value.is_empty();

    div()
        .flex()
        .items_center()
        .gap(px(8.0))
        .px(px(14.0))
        .py(px(8.0))
        .min_h(px(36.0))
        .rounded(px(20.0))
        .bg(rgb(theme.surface))
        .border_1()
        .border_color(border_color)
        .child(
            // Search icon
            div()
                .text_size(px(14.0))
                .text_color(rgb(theme.text_secondary))
                .child("\u{1F50D}"), // 🔍
        )
        .child(
            div()
                .flex_1()
                .text_size(px(14.0))
                .text_color(text_color)
                .child(display_text),
        )
        .when(has_text, |s| {
            s.child(
                // Clear button
                div()
                    .w(px(18.0))
                    .h(px(18.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded(px(9.0))
                    .bg(rgb(theme.text_disabled))
                    .cursor_pointer()
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(rgb(theme.surface))
                            .child("\u{2715}"), // ✕
                    ),
            )
        })
}
