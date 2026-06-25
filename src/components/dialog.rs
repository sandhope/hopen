/// Dialog / Modal overlay component.
///
/// Provides a full-screen backdrop with a centered dialog card containing
/// a title, body text, and action buttons (confirm / cancel / danger).
///
/// Uses `Entity<AppView>` (which is `Copy`) instead of `&mut Context` so
/// that the element tree can be built without holding a mutable borrow.
///
/// Usage:
/// ```ignore
/// self.active_dialog = Some(DialogParams::confirm(
///     "Delete Profile",
///     "Are you sure?",
/// ));
/// ```

use gpui::*;
use gpui::prelude::*;

use crate::theme::Theme;

// ─── Dialog parameter types ────────────────────────────────────────

/// Which kind of dialog to show.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogKind {
    Info,
    Confirm,
    Danger,
}

/// Parameters for rendering a dialog overlay.
#[derive(Clone, Debug)]
pub struct DialogParams {
    pub kind: DialogKind,
    pub title: String,
    pub body: String,
    pub primary_label: Option<String>,
    pub secondary_label: Option<String>,
}

impl DialogParams {
    pub fn info(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            kind: DialogKind::Info,
            title: title.into(),
            body: body.into(),
            primary_label: None,
            secondary_label: None,
        }
    }

    pub fn confirm(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            kind: DialogKind::Confirm,
            title: title.into(),
            body: body.into(),
            primary_label: None,
            secondary_label: None,
        }
    }

    pub fn danger(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            kind: DialogKind::Danger,
            title: title.into(),
            body: body.into(),
            primary_label: None,
            secondary_label: None,
        }
    }

    pub fn primary_label(&self) -> &str {
        self.primary_label
            .as_deref()
            .unwrap_or(match self.kind {
                DialogKind::Info => "OK",
                DialogKind::Confirm => "Confirm",
                DialogKind::Danger => "Delete",
            })
    }

    pub fn secondary_label(&self) -> &str {
        self.secondary_label
            .as_deref()
            .unwrap_or(match self.kind {
                DialogKind::Info => "",
                DialogKind::Confirm => "Cancel",
                DialogKind::Danger => "Cancel",
            })
    }
}

// ─── Sizing constants ──────────────────────────────────────────────

const DIALOG_MAX_WIDTH: f32 = 400.0;
const DIALOG_PADDING: f32 = 24.0;
const DIALOG_RADIUS: f32 = 12.0;
const DIALOG_GAP: f32 = 12.0;
const BTN_HEIGHT: f32 = 36.0;

// ─── Public render function ────────────────────────────────────────

/// Render a modal dialog overlay.
///
/// `entity` is the `Entity<AppView>` (Copy) for updating state from
/// button callbacks. The caller is responsible for clearing
/// `active_dialog` when the user clicks an action or the backdrop.
pub fn render_dialog(
    params: &DialogParams,
    theme: &Theme,
    entity: Entity<crate::app::AppView>,
) -> impl IntoElement {
    let title = params.title.clone();
    let body_text = params.body.clone();
    let primary_label = params.primary_label().to_string();
    let secondary_label = params.secondary_label().to_string();
    let has_secondary = !secondary_label.is_empty();
    let kind = params.kind;

    // Build buttons upfront — each needs its own entity clone.
    let primary_btn = dialog_btn(
        theme,
        &primary_label,
        match kind {
            DialogKind::Danger => BtnStyle::Danger,
            _ => BtnStyle::Primary,
        },
        entity.clone(),
    );

    let secondary_btn = if has_secondary {
        Some(dialog_btn(theme, &secondary_label, BtnStyle::Secondary, entity.clone()))
    } else {
        None
    };

    // Full-screen backdrop.
    div()
        .absolute()
        .top_0()
        .left_0()
        .right_0()
        .bottom_0()
        .flex()
        .items_center()
        .justify_center()
        .bg(rgba(0x00000055))
        .id("dialog-overlay")
        .on_click({
            move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
                entity.update(cx, |this, _| {
                    this.active_dialog = None;
                });
                cx.refresh_windows();
            }
        })
        // Dialog card — styling before `.id()`, interaction after.
        .child(
            div()
                .max_w(px(DIALOG_MAX_WIDTH))
                .px(px(DIALOG_PADDING))
                .py(px(DIALOG_PADDING))
                .rounded(px(DIALOG_RADIUS))
                .bg(rgb(theme.surface))
                .border(px(1.0))
                .border_color(rgb(theme.border))
                .id("dialog-card")
                .on_click(|_: &ClickEvent, _: &mut Window, _: &mut App| {
                    // Absorb click — prevent backdrop dismiss.
                })
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(DIALOG_GAP))
                        .child(
                            div()
                                .text_size(px(16.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(theme.text_primary))
                                .child(title),
                        )
                        .child(
                            div()
                                .text_size(px(13.0))
                                .text_color(rgb(theme.text_secondary))
                                .line_height(px(18.0))
                                .child(body_text),
                        )
                        .child(div().h(px(4.0)))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .justify_end()
                                .gap(px(8.0))
                                .children(secondary_btn)
                                .child(primary_btn),
                        ),
                ),
        )
}

// ─── Helpers ───────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum BtnStyle {
    Primary,
    Secondary,
    Danger,
}

fn dialog_btn(
    theme: &Theme,
    label: &str,
    style: BtnStyle,
    entity: Entity<crate::app::AppView>,
) -> impl IntoElement {
    let label = label.to_string();
    div()
        .flex()
        .items_center()
        .justify_center()
        .px(px(20.0))
        .h(px(BTN_HEIGHT))
        .rounded(px(6.0))
        .text_size(px(13.0))
        .font_weight(FontWeight::SEMIBOLD)
        .cursor_pointer()
        .when(style == BtnStyle::Primary, |this| {
            this.bg(rgb(theme.accent))
                .text_color(rgb(0xffffff))
                .hover(|s| s.bg(rgb(theme.accent_hover)))
        })
        .when(style == BtnStyle::Secondary, |this| {
            this.bg(rgba(0x00000000))
                .text_color(rgb(theme.text_secondary))
                .hover(|s| s.bg(rgb(theme.surface_variant)))
        })
        .when(style == BtnStyle::Danger, |this| {
            this.bg(rgb(theme.status_error))
                .text_color(rgb(0xffffff))
                .hover(|s| s.bg(rgb(0xdc2626)))
        })
        .id(match style {
            BtnStyle::Primary => "dialog-btn-primary",
            BtnStyle::Secondary => "dialog-btn-secondary",
            BtnStyle::Danger => "dialog-btn-danger",
        })
        .on_click({
            move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
                entity.update(cx, |this, _| {
                    this.active_dialog = None;
                });
                cx.refresh_windows();
            }
        })
        .child(label)
}
