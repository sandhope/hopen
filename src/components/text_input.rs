/// TextInput — a self-contained text input entity for GPUI.
///
/// Manages its own text, focus handle, and keyboard input.
/// Call `render()` to get a styled, interactive input element.
///
/// Usage:
/// ```ignore
/// // In AppView::new:
/// let search = cx.new(|cx| TextInput::new("Search...", window, cx));
///
/// // In view function:
/// let text = search.read(cx).text().to_string(); // for filtering
/// div().child(search.read(cx).render(theme, cx))  // renders the input
/// ```

use gpui::*;

use crate::theme::{Theme, CARD_RADIUS};

/// A self-contained text input that owns its text, focus, and keyboard handling.
pub struct TextInput {
    text: String,
    focus_handle: FocusHandle,
    placeholder: String,
    /// Optional font family override (e.g. "monospace" for URL inputs).
    font_family: Option<&'static str>,
}

impl TextInput {
    /// Create a new text input with the given placeholder.
    pub fn new(placeholder: impl Into<String>, _window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self {
            text: String::new(),
            focus_handle,
            placeholder: placeholder.into(),
            font_family: None,
        }
    }

    /// Set a custom font family (e.g. "monospace").
    pub fn set_font_family(&mut self, family: &'static str) {
        self.font_family = Some(family);
    }

    /// Get the current text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set the text content.
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    /// Clear the text.
    pub fn clear(&mut self) {
        self.text.clear();
    }

    /// Focus this input on the given window.
    pub fn focus(&self, window: &mut Window, _cx: &mut Context<Self>) {
        self.focus_handle.focus(window);
    }

    /// Render the interactive input element with search icon.
    ///
    /// Returns a styled div with focus tracking, keyboard handling,
    /// search icon, and text/placeholder display.
    pub fn render(&self, theme: &Theme, cx: &mut Context<Self>) -> AnyElement {
        self.build_element(theme, cx, true)
    }

    /// Render without the search icon (for URL inputs, etc.).
    pub fn render_plain(&self, theme: &Theme, cx: &mut Context<Self>) -> AnyElement {
        self.build_element(theme, cx, false)
    }

    /// Internal: build the input element.
    fn build_element(&self, theme: &Theme, cx: &mut Context<Self>, show_icon: bool) -> AnyElement {
        let focus_handle = self.focus_handle.clone();
        let has_text = !self.text.is_empty();
        let display = self.text.clone();
        let placeholder = self.placeholder.clone();
        let font_family = self.font_family;

        let text_color = if has_text {
            rgb(theme.text_primary)
        } else {
            rgb(theme.text_disabled)
        };

        let mut text_div = div()
            .flex_1()
            .text_size(px(13.0))
            .text_color(text_color);

        if let Some(ff) = font_family {
            text_div = text_div.font_family(ff);
        }

        let text_elem = if has_text {
            text_div.child(display)
        } else {
            text_div.child(placeholder)
        };

        let mut element = div()
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
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                // Keep focus on this input.
                this.focus_handle.focus(window);
                if let Some(ref ch) = event.keystroke.key_char {
                    this.text.push_str(ch);
                } else {
                    match event.keystroke.key.as_str() {
                        "backspace" => { this.text.pop(); }
                        "space" => { this.text.push(' '); }
                        _ => {}
                    }
                }
                cx.notify();
            }));

        if show_icon {
            element = element.child(
                div()
                    .text_size(px(14.0))
                    .text_color(rgb(theme.text_disabled))
                    .flex_shrink_0()
                    .child("\u{1F50D}"),
            );
        }

        element.child(text_elem).into_any_element()
    }
}
