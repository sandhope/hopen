#![allow(dead_code)]
/// Toast notification overlay component.
///
/// Renders a stack of notification bars at the bottom-right of the window.
/// Toasts auto-dismiss after a timeout (frame-based) or can be manually
/// dismissed via the close button.
///
/// Uses `Entity<AppView>` (Copy) to avoid mutable borrow conflicts
/// during element tree construction.

use gpui::*;

use crate::theme::Theme;

// ─── Toast types ───────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastKind {
    Success,
    Error,
    Warning,
    Info,
}

#[derive(Clone, Debug)]
pub struct ToastData {
    pub kind: ToastKind,
    pub message: String,
}

impl ToastData {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            kind: ToastKind::Success,
            message: message.into(),
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            kind: ToastKind::Error,
            message: message.into(),
        }
    }

    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            kind: ToastKind::Warning,
            message: message.into(),
        }
    }

    pub fn info(message: impl Into<String>) -> Self {
        Self {
            kind: ToastKind::Info,
            message: message.into(),
        }
    }
}

// ─── Constants ─────────────────────────────────────────────────────

const MAX_VISIBLE_TOASTS: usize = 5;
const TOAST_MAX_WIDTH: f32 = 360.0;
const TOAST_GAP: f32 = 8.0;
const TOAST_BOTTOM_OFFSET: f32 = 24.0;
const TOAST_RADIUS: f32 = 8.0;

// ─── Public functions ──────────────────────────────────────────────

/// Render the stack of active toasts. Returns `None` if the list is empty.
pub fn render_toasts(
    toasts: &[ToastData],
    theme: &Theme,
    entity: Entity<crate::app::AppView>,
) -> Option<impl IntoElement> {
    if toasts.is_empty() {
        return None;
    }

    let theme = *theme;

    // Show only the most recent N toasts, newest first.
    let visible: Vec<ToastItem> = toasts
        .iter()
        .rev()
        .take(MAX_VISIBLE_TOASTS)
        .map(|t| ToastItem {
            message: t.message.clone(),
            kind: t.kind,
        })
        .collect();

    // Build cards — each needs its own entity clone for the close button.
    let mut cards: Vec<AnyElement> = Vec::new();
    for item in visible {
        cards.push(toast_card(item, &theme, entity.clone()).into_any_element());
    }

    Some(
        div()
            .absolute()
            .bottom(px(TOAST_BOTTOM_OFFSET))
            .right(px(TOAST_BOTTOM_OFFSET))
            .flex()
            .flex_col()
            .gap(px(TOAST_GAP))
            .max_w(px(TOAST_MAX_WIDTH))
            .id("toast-container")
            .children(cards),
    )
}

/// Remove a specific toast by message text.
pub fn remove_toast(toasts: &mut Vec<ToastData>, message: &str) {
    toasts.retain(|t| t.message != message);
}

// ─── Internal types ────────────────────────────────────────────────

struct ToastItem {
    message: String,
    kind: ToastKind,
}

// ─── Helpers ───────────────────────────────────────────────────────

fn toast_card(
    item: ToastItem,
    theme: &Theme,
    entity: Entity<crate::app::AppView>,
) -> impl IntoElement {
    let kind = item.kind;
    let message = item.message;

    let (icon, border_color, bg_color, text_color) = match kind {
        ToastKind::Success => ("\u{2705}", theme.status_success, 0xdcfce7, 0x052e16),
        ToastKind::Error => ("\u{274C}", theme.status_error, 0xfee2e2, 0x3b1515),
        ToastKind::Warning => ("\u{26A0}\u{FE0F}", theme.status_warning, 0xfef9c3, 0x2e2408),
        ToastKind::Info => ("\u{2139}\u{FE0F}", theme.status_info, 0xdbeafe, 0x162447),
    };

    div()
        .flex()
        .items_center()
        .gap(px(10.0))
        .px(px(16.0))
        .py(px(12.0))
        .rounded(px(TOAST_RADIUS))
        .bg(rgb(bg_color))
        .text_color(rgb(text_color))
        .border(px(1.0))
        .border_color(rgb(border_color))
        .id("toast-card")
        .child(
            div().text_size(px(16.0)).flex_shrink_0().child(icon),
        )
        .child(
            div().text_size(px(13.0)).overflow_hidden().child(message.clone()),
        )
        .child(
            div()
                .ml_auto()
                .rounded(px(4.0))
                .flex()
                .items_center()
                .justify_center()
                .w(px(20.0))
                .h(px(20.0))
                .cursor_pointer()
                .hover(|s| s.bg(rgba(0x00000015)))
                .id("toast-close")
                .on_click({
                    let entity = entity;
                    let msg = message;
                    move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
                        entity.update(cx, |this, _| {
                            this.toasts.retain(|t| t.message != msg);
                        });
                        cx.refresh_windows();
                    }
                })
                .child(div().text_size(px(14.0)).child("\u{2715}")),
        )
}
