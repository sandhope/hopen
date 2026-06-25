/// Side Sheet — a panel that slides in from the right edge.
///
/// Architecture:
/// - Backdrop overlay absorbs clicks to dismiss
/// - Panel slides in from the right with fixed width
/// - Header with title and close button
/// - Scrollable content area

use std::sync::Arc;

use gpui::*;

use crate::app::AppView;
use crate::theme::Theme;

/// Parameters controlling the side sheet appearance and behaviour.
pub struct SideSheetParams {
    /// Title shown in the sheet header.
    pub title: String,
    /// Whether the sheet is currently visible.
    pub visible: bool,
    /// Width of the side panel in pixels. Default 320.
    pub width: f32,
}

impl Default for SideSheetParams {
    fn default() -> Self {
        Self {
            title: String::new(),
            visible: false,
            width: 320.0,
        }
    }
}

/// Render a side sheet overlay that slides in from the right.
///
/// `content` is a closure that builds the sheet body.
/// `on_close` is invoked when the backdrop or close button is clicked.
pub fn side_sheet(
    params: &SideSheetParams,
    theme: &Theme,
    _entity: Entity<AppView>,
    content: impl FnOnce() -> AnyElement,
    on_close: Arc<dyn Fn(&ClickEvent, &mut Window, &mut App)>,
) -> AnyElement {
    if !params.visible {
        return div().into_any_element();
    }

    let surface = rgb(theme.surface);
    let title_text = params.title.clone();
    let sheet_width = params.width;

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
                .id("side-sheet-backdrop")
                .on_click(move |e, w, a| on_close_backdrop(e, w, a)),
        )
        .child(
            // Sheet panel — pinned to right edge.
            div()
                .absolute()
                .top_0()
                .right_0()
                .bottom_0()
                .w(px(sheet_width))
                .bg(surface)
                .flex()
                .flex_col()
                .child(
                    // ── Header ──
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .px(px(20.0))
                        .py(px(16.0))
                        .border_b_1()
                        .border_color(rgb(theme.border_light))
                        .child(
                            div()
                                .text_size(px(16.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(theme.text_primary))
                                .child(title_text),
                        )
                        .child(
                            div()
                                .id("side-sheet-close")
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
                        .py(px(16.0))
                        .child(content()),
                ),
        )
        .into_any_element()
}
