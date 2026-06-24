/// Custom titlebar component for the Hopen desktop client.
///
/// Replaces the native OS titlebar when `appears_transparent` is set to `true`.
/// Provides a drag region with the application name and window control buttons
/// (minimize, maximize/restore, close), styled to match the active theme.
///
/// Inspired by the Velotype markdown editor's window chrome.

use gpui::*;

use crate::theme::Theme;

/// Height of the custom titlebar in pixels.
pub const TITLEBAR_HEIGHT: f32 = 32.0;
/// Width of each window control button (minimize / maximize / close).
const CTRL_BTN_WIDTH: f32 = 46.0;
/// Size (width & height) of the SVG titlebar icons.
const CTRL_ICON_SIZE: f32 = 12.0;

// ─── SVG asset paths ────────────────────────────────────────────

const ICON_MINIMIZE: &str = "icon/titlebar/chrome-minimize.svg";
const ICON_MAXIMIZE: &str = "icon/titlebar/chrome-maximize.svg";
const ICON_RESTORE: &str = "icon/titlebar/chrome-restore.svg";
const ICON_CLOSE: &str = "icon/titlebar/chrome-close.svg";

// ─── Public API ──────────────────────────────────────────────────

/// Render a custom titlebar with drag support and SVG window control buttons.
pub fn render_titlebar(
    theme: &Theme,
    window: &mut Window,
) -> impl IntoElement + use<> {
    let bg = rgb(theme.titlebar_bg);
    let border = rgb(theme.titlebar_border);
    let icon_color = Hsla::from(rgb(theme.titlebar_icon));
    let is_maximized = window.is_maximized() || window.is_fullscreen();

    div()
        .absolute()
        .top_0()
        .left_0()
        .right_0()
        .h(px(TITLEBAR_HEIGHT))
        .occlude()
        .flex()
        .items_center()
        .bg(bg)
        .border_b(px(1.0))
        .border_color(border)
        // ── Drag region + window title (center) ──
        .child(
            div()
                .id("titlebar-drag")
                .flex()
                .items_center()
                .h_full()
                .flex_1()
                .min_w(px(0.0))
                .px(px(12.0))
                .window_control_area(WindowControlArea::Drag)
                .on_click(|event, window, _cx| {
                    if event.is_right_click() {
                        window.show_window_menu(event.position());
                    }
                })
                .child(
                    div()
                        .min_w(px(0.0))
                        .truncate()
                        .text_size(px(12.0))
                        .text_color(rgb(theme.titlebar_text))
                        .child("Hopen"),
                ),
        )
        // ── Window control buttons (right) ──
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .h_full()
                .flex_shrink_0()
                // Minimize
                .child(win_button(
                    WindowControlArea::Min,
                    ICON_MINIMIZE,
                    icon_color,
                    theme,
                    |_, window, _cx| {
                        window.minimize_window();
                    },
                ))
                // Maximize / Restore
                .child(win_button(
                    WindowControlArea::Max,
                    if is_maximized { ICON_RESTORE } else { ICON_MAXIMIZE },
                    icon_color,
                    theme,
                    |_, window, _cx| {
                        window.zoom_window();
                    },
                ))
                // Close
                .child(win_button(
                    WindowControlArea::Close,
                    ICON_CLOSE,
                    icon_color,
                    theme,
                    |_, window, _cx| {
                        window.remove_window();
                    },
                )),
        )
}

// ─── Helpers ─────────────────────────────────────────────────────

/// Render a single window control button with SVG icon, hover effect, and click handler.
fn win_button(
    area: WindowControlArea,
    icon_path: &'static str,
    icon_color: Hsla,
    theme: &Theme,
    on_click: fn(&ClickEvent, &mut Window, &mut App),
) -> impl IntoElement + use<> {
    let (hover_bg, hover_fg): (Hsla, Hsla) = match area {
        WindowControlArea::Close => (
            rgb(theme.titlebar_close_hover_bg).into(),
            rgb(theme.titlebar_close_hover_text).into(),
        ),
        _ => (rgb(theme.titlebar_button_hover_bg).into(), icon_color),
    };

    div()
        .id(match area {
            WindowControlArea::Min => "titlebar-minimize",
            WindowControlArea::Max => "titlebar-maximize",
            WindowControlArea::Close => "titlebar-close",
            _ => "titlebar-button",
        })
        .w(px(CTRL_BTN_WIDTH))
        .h_full()
        .flex()
        .items_center()
        .justify_center()
        .occlude()
        .window_control_area(area)
        .hover(move |style| style.bg(hover_bg).text_color(hover_fg))
        .cursor_pointer()
        .child(
            svg()
                .path(icon_path)
                .size(px(CTRL_ICON_SIZE))
                .text_color(icon_color),
        )
        .on_click(move |event, window, cx| {
            if event.standard_click() {
                on_click(event, window, cx);
            }
        })
}
