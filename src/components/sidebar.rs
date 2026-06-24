/// Sidebar navigation component for the Hopen desktop client.
///
/// Renders a vertical navigation panel with page icons and labels.
/// Follows a Zed-inspired design: clean, minimal, theme-aware.

use gpui::*;

use crate::navigation::Page;
use crate::theme::Theme;

/// Render the sidebar. Called from `AppView::render` each frame.
///
/// - `current_page`: the currently active page (highlighted in the sidebar)
/// - `cx`: listener context from `AppView`, used to attach click handlers
/// - `theme`: the active colour palette
pub fn render_sidebar(
    current_page: Page,
    cx: &mut Context<crate::app::AppView>,
    theme: &Theme,
) -> impl IntoElement + use<> {
    // ── Logo area ──────────────────────────────────────────────
    let logo = div()
        .flex()
        .items_center()
        .px(px(16.0))
        .h(px(56.0))
        .child(
            div()
                .text_size(px(18.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(theme.accent))
                .child("Hopen"),
        );

    // ── Divider ────────────────────────────────────────────────
    let divider = div()
        .mx(px(12.0))
        .my(px(8.0))
        .h(px(1.0))
        .bg(rgb(theme.border_light));

    // ── Navigation items ───────────────────────────────────────
    let mut nav_items = div().flex().flex_col().gap(px(2.0)).px(px(8.0));

    for page in Page::ALL {
        let is_active = *page == current_page;
        let icon_path = page.icon_path();
        let title = page.title();
        let icon_color = if is_active {
            rgb(theme.accent)
        } else {
            rgb(theme.text_secondary)
        };

        nav_items = nav_items.child(
            div()
                .id(page.title())
                .flex()
                .items_center()
                .gap(px(10.0))
                .px(px(12.0))
                .h(px(crate::theme::NAV_ITEM_HEIGHT))
                .rounded(px(6.0))
                .bg(if is_active {
                    rgb(theme.sidebar_active_bg)
                } else {
                    rgba(0x00000000)
                })
                .text_color(if is_active {
                    rgb(theme.accent)
                } else {
                    rgb(theme.text_secondary)
                })
                .cursor_pointer()
                .hover(|s| {
                    if !is_active {
                        s.bg(rgb(theme.sidebar_hover_bg))
                            .text_color(rgb(theme.text_primary))
                    } else {
                        s
                    }
                })
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.current_page = *page;
                    cx.notify();
                }))
                .child(
                    svg()
                        .path(icon_path)
                        .size(px(18.0))
                        .text_color(icon_color),
                )
                .child(div().text_size(px(13.0)).child(title)),
        );
    }

    // ── Spacer ─────────────────────────────────────────────────
    let spacer = div().flex_1();

    // ── Footer (version) ───────────────────────────────────────
    let footer = div()
        .px(px(16.0))
        .pb(px(12.0))
        .child(
            div()
                .text_size(px(11.0))
                .text_color(rgb(theme.text_disabled))
                .child(format!("v{}", env!("CARGO_PKG_VERSION"))),
        );

    // ── Sidebar container ──────────────────────────────────────
    div()
        .flex()
        .flex_col()
        .w(px(crate::theme::SIDEBAR_WIDTH))
        .h_full()
        .bg(rgb(theme.sidebar_bg))
        .border_r_1()
        .border_color(rgb(theme.border_light))
        .child(logo)
        .child(divider)
        .child(nav_items)
        .child(spacer)
        .child(footer)
}
