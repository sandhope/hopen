/// Input component — styled text input container.
///
/// Architecture:
/// - Visual container with label, text display, focus border, and error state
/// - Actual text editing is handled by the parent view via `on_key_down`
/// - Supports prefix/suffix, password masking, and clear/error states
///
/// Usage:
/// ```ignore
/// input_field(
///     &InputParams { label: Some("Name"), value: &text, placeholder: "...", .. },
///     theme,
/// )
/// ```

use gpui::*;

use crate::theme::Theme;

/// Parameters for rendering an input field.
pub struct InputParams<'a> {
    /// Optional label displayed above the input.
    pub label: Option<&'a str>,
    /// Current text value to display.
    pub value: &'a str,
    /// Placeholder text shown when value is empty.
    pub placeholder: &'a str,
    /// Whether the input currently has focus (shows accent border).
    pub focused: bool,
    /// Whether the input is in an error state (shows red border).
    pub error: bool,
    /// Error message displayed below the input.
    pub error_message: Option<&'a str>,
    /// Whether the input is disabled.
    pub disabled: bool,
    /// Whether to mask text (for password fields).
    pub password: bool,
    /// Whether to show a prefix icon area (caller renders prefix via child).
    pub has_prefix: bool,
    /// Whether to show a suffix area (caller renders suffix via child).
    pub has_suffix: bool,
}

impl<'a> Default for InputParams<'a> {
    fn default() -> Self {
        Self {
            label: None,
            value: "",
            placeholder: "",
            focused: false,
            error: false,
            error_message: None,
            disabled: false,
            password: false,
            has_prefix: false,
            has_suffix: false,
        }
    }
}

/// Render a styled input field.
///
/// Callers can add prefix/suffix children by including them before/after
/// the text element in a wrapper.
pub fn input_field(params: &InputParams, theme: &Theme) -> impl IntoElement {
    let border_color = if params.error {
        rgb(theme.status_error)
    } else if params.focused {
        rgb(theme.accent)
    } else {
        rgb(theme.border_light)
    };

    let bg = if params.disabled {
        rgb(theme.surface_variant)
    } else {
        rgb(theme.surface)
    };

    let text_color = if params.disabled {
        rgb(theme.text_disabled)
    } else if params.value.is_empty() {
        rgb(theme.text_disabled)
    } else {
        rgb(theme.text_primary)
    };

    let display_text = if params.value.is_empty() {
        params.placeholder.to_string()
    } else if params.password {
        "\u{2022}".repeat(params.value.len())
    } else {
        params.value.to_string()
    };

    // Build the label element (or empty placeholder).
    let label_el: AnyElement = if let Some(label) = params.label {
        div()
            .text_size(px(12.0))
            .font_weight(FontWeight::MEDIUM)
            .text_color(rgb(theme.text_secondary))
            .child(label.to_string())
            .into_any_element()
    } else {
        div().into_any_element()
    };

    // Build the error message element (or empty placeholder).
    let error_el: AnyElement = if let Some(msg) = params.error_message {
        div()
            .text_size(px(11.0))
            .text_color(rgb(theme.status_error))
            .child(msg.to_string())
            .into_any_element()
    } else {
        div().into_any_element()
    };

    div()
        .flex()
        .flex_col()
        .gap(px(4.0))
        .child(label_el)
        .child(
            // Input container
            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .px(px(12.0))
                .py(px(8.0))
                .min_h(px(36.0))
                .rounded(px(8.0))
                .bg(bg)
                .border_1()
                .border_color(border_color)
                .child(
                    div()
                        .flex_1()
                        .text_size(px(14.0))
                        .text_color(text_color)
                        .child(display_text),
                ),
        )
        .child(error_el)
}
