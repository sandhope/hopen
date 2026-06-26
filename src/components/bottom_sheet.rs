#![allow(dead_code)]
/// Bottom Sheet — a panel that slides up from the bottom of the screen.
///
/// Architecture:
/// - Backdrop overlay absorbs clicks to dismiss
/// - Panel is absolutely positioned at the bottom
/// - Rounded top corners + drag handle indicator
/// - Content area with optional scrolling

use std::sync::Arc;

use gpui::*;

use crate::app::AppView;
use crate::theme::Theme;

/// Parameters controlling the bottom sheet appearance and behaviour.
pub struct BottomSheetParams {
    /// Title shown in the sheet header.
    pub title: String,
    /// Whether the sheet is currently visible.
    pub visible: bool,
    /// Sheet height in pixels. Default 420.
    pub height: f32,
}

impl Default for BottomSheetParams {
    fn default() -> Self {
        Self {
            title: String::new(),
            visible: false,
            height: 420.0,
        }
    }
}

/// Render a bottom sheet overlay.
///
/// `content` is a closure that builds the sheet body.
/// `on_close` is invoked when the backdrop or close button is clicked.
pub fn bottom_sheet(
    params: &BottomSheetParams,
    theme: &Theme,
    _entity: Entity<AppView>,
    content: impl FnOnce() -> AnyElement,
    on_close: Arc<dyn Fn(&ClickEvent, &mut Window, &mut App)>,
) -> AnyElement {
    if !params.visible {
        return div().into_any_element();
    }

    let handle_color = rgb(theme.text_disabled);
    let surface = rgb(theme.surface);
    let title_text = params.title.clone();
    let sheet_h = params.height;

    let on_close_backdrop = on_close.clone();
    let on_close_btn = on_close.clone();

    div()
        .absolute()
        .top_0()
        .left_0()
        .right_0()
        .bottom_0()
        .child(
            // Semi-transparent backdrop.
            div()
                .absolute()
                .top_0()
                .left_0()
                .right_0()
                .bottom_0()
                .bg(rgba(0x00000080))
                .id("bottom-sheet-backdrop")
                .on_click(move |e, w, a| on_close_backdrop(e, w, a)),
        )
        .child(
            // Sheet panel — pinned to bottom.
            div()
                .absolute()
                .bottom_0()
                .left_0()
                .right_0()
                .h(px(sheet_h))
                .rounded_t(px(20.0))
                .bg(surface)
                .flex()
                .flex_col()
                .child(
                    // ── Drag Handle ──
                    div()
                        .flex()
                        .justify_center()
                        .py(px(8.0))
                        .child(
                            div()
                                .w(px(32.0))
                                .h(px(4.0))
                                .rounded(px(2.0))
                                .bg(handle_color),
                        ),
                )
                .child(
                    // ── Header ──
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .px(px(20.0))
                        .py(px(8.0))
                        .child(
                            div()
                                .text_size(px(16.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(theme.text_primary))
                                .child(title_text),
                        )
                        .child(
                            div()
                                .id("bottom-sheet-close")
                                .w(px(28.0))
                                .h(px(28.0))
                                .flex()
                                .items_center()
                                .justify_center()
                                .rounded(px(14.0))
                                .hover(|s| s.bg(rgb(theme.surface_variant)))
                                .cursor_pointer()
                                .on_click(move |e, w, a| on_close_btn(e, w, a))
                                .child(
                                    div()
                                        .text_size(px(18.0))
                                        .text_color(rgb(theme.text_secondary))
                                        .child("\u{2715}"), // ✕
                                ),
                        ),
                )
                .child(
                    // ── Content ──
                    div()
                        .flex_1()
                        .overflow_hidden()
                        .px(px(20.0))
                        .pb(px(20.0))
                        .child(content()),
                ),
        )
        .into_any_element()
}
