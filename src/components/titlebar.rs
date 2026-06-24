/// Custom titlebar component for the Hopen desktop client.
///
/// Replaces the native OS titlebar when `appears_transparent` is set to `true`.
/// Provides a drag region with the application name and window control buttons
/// (minimize, maximize, close), styled to match the active theme.

use gpui::*;

use crate::theme::Theme;

/// Height of the custom titlebar in pixels.
pub const TITLEBAR_HEIGHT: f32 = 32.0;
/// Width of each window control button (minimize / maximize / close).
const CTRL_BTN_WIDTH: f32 = 46.0;

/// Render a custom titlebar with drag support and window control buttons.
///
/// - `theme`: the active colour palette
pub fn render_titlebar(theme: &Theme) -> impl IntoElement + use<> {
    let bg = rgb(theme.sidebar_bg);
    let border = rgb(theme.border_light);
    let text_muted = rgb(theme.text_secondary);

    div()
        .flex()
        .items_center()
        .h(px(TITLEBAR_HEIGHT))
        .bg(bg)
        .border_b_1()
        .border_color(border)
        // Make the entire titlebar draggable (platform-level drag region).
        // `.occlude()` is required so a hitbox gets generated, otherwise
        // `WindowControlArea::Drag` has no effect.
        .occlude()
        .window_control_area(WindowControlArea::Drag)
        // Title label (left side)
        .child(
            div()
                .flex()
                .items_center()
                .h_full()
                .flex_1()
                .pl(px(12.0))
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(text_muted)
                        .child("Hopen"),
                ),
        )
        // Window control buttons (right side)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .h_full()
                // ── Minimize ──
                .child(ctrl_button(
                    WindowControlArea::Min,
                    CTRL_BTN_WIDTH,
                    theme,
                    "─",
                ))
                // ── Maximize / Restore ──
                .child(ctrl_button(
                    WindowControlArea::Max,
                    CTRL_BTN_WIDTH,
                    theme,
                    "□",
                ))
                // ── Close ──
                .child(ctrl_button(
                    WindowControlArea::Close,
                    CTRL_BTN_WIDTH,
                    theme,
                    "✕",
                )),
        )
}

/// Helper: render a single window control button.
///
/// The `.occlude()` call forces a hitbox to be generated (even without
/// mouse listeners), which is required for `window_control_area` to work.
///
/// On Windows the platform handles click actions natively via the hit-test
/// callback; `.hover()` and `.active()` provide visual feedback.
fn ctrl_button(
    area: WindowControlArea,
    width: f32,
    theme: &Theme,
    label: &'static str,
) -> impl IntoElement + use<> {
    let font_size = if label == "✕" { 16.0 } else { 14.0 };
    let text_muted = rgb(theme.text_secondary);

    // Close button gets red hover background
    let (hover_bg, hover_fg) = match area {
        WindowControlArea::Close => (rgb(0xc4_2b_1c), rgb(0xff_ff_ff)),
        _ => (rgb(theme.sidebar_hover_bg), text_muted),
    };

    div()
        .flex()
        .items_center()
        .justify_center()
        .w(px(width))
        .h_full()
        .occlude()
        .hover(move |style| style.bg(hover_bg).text_color(hover_fg))
        .window_control_area(area)
        .child(
            div()
                .text_size(px(font_size))
                .text_color(text_muted)
                .child(label),
        )
}
