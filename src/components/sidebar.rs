/// Sidebar navigation component for the Hopen desktop client.
///
/// Renders a vertical navigation panel with page icons and labels.
/// Follows a Zed-inspired design: clean, minimal, theme-aware.

use std::path::PathBuf;

use gpui::*;

use crate::i18n::I18nStrings;
use crate::navigation::Page;
use crate::theme::Theme;

/// Render the sidebar. Called from `AppView::render` each frame.
///
/// - `current_page`: the currently active page (highlighted in the sidebar)
/// - `cx`: listener context from `AppView`, used to attach click handlers
/// - `theme`: the active colour palette
/// - `strings`: localised UI strings
/// - `width`: current sidebar width (pixels), adjustable via drag on the resize handle
pub fn render_sidebar(
    current_page: Page,
    cx: &mut Context<crate::app::AppView>,
    theme: &Theme,
    strings: &I18nStrings,
    width: f32,
) -> impl IntoElement + use<> {
    // ── Logo area ──────────────────────────────────────────────
    let logo = div()
        .flex()
        .items_center()
        //.justify_center()
        .ml(px(20.0))
        .mt(px(12.0))
        .h(px(56.0))
        .child(
            img(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources/app-icon.png"))
                .size(px(40.0)),
        );

    // ── Divider ────────────────────────────────────────────────
    let divider = div()
        .mx(px(12.0))
        .my(px(8.0))
        .h(px(1.0))
        .bg(rgb(theme.border_light));

    // ── Navigation items ───────────────────────────────────────
    let mut nav_items = div()
        .id("sidebar-nav")
        .flex()
        .flex_col()
        .flex_1()
        .overflow_y_scroll()
        .gap(px(2.0))
        .px(px(8.0));

    for page in Page::ALL {
        let is_active = *page == current_page;
        let icon_path = page.icon_path();
        let title = strings.page_title(*page);
        let id = page.title(); // keep English key as DOM id
        let icon_color = if is_active {
            rgb(theme.accent)
        } else {
            rgb(theme.text_secondary)
        };

        nav_items = nav_items.child(
            div()
                .id(id)
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

    // ── Sidebar container ──────────────────────────────────────
    div()
        .flex()
        .flex_col()
        .w(px(width))
        .h_full()
        .bg(rgb(theme.sidebar_bg))
        .border_r_1()
        .border_color(rgb(theme.border_light))
        .child(logo)
        .child(divider)
        .child(nav_items)
}
