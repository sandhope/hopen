/// Root application view for Hopen.
///
/// `AppView` owns the navigation state and renders the sidebar + content layout.
/// This is the top-level entity created for the main window.

use gpui::*;

use crate::components::sidebar;
use crate::components::titlebar;
use crate::current_theme;
use crate::i18n::I18nManager;
use crate::navigation::{Page, ToolsSubPage};
use crate::views;

/// The root view that composes sidebar navigation with page content.
pub struct AppView {
    /// Currently active page — drives both sidebar highlight and content rendering.
    pub current_page: Page,
    /// Active sub-page within Settings (Tools). `None` means show the main settings list.
    pub tools_sub_page: Option<ToolsSubPage>,
}

impl Render for AppView {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = current_theme(cx);
        let strings = cx.global::<I18nManager>().strings_arc();

        // Build sidebar first (consumes cx borrow), then content.
        let sidebar = sidebar::render_sidebar(self.current_page, cx, &theme, &strings);
        let content = views::render_page(self.current_page, self.tools_sub_page, &theme, cx);
        let titlebar = titlebar::render_titlebar(&theme, window, &strings);

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(theme.content_bg))
            .text_color(rgb(theme.text_primary))
            // Custom titlebar
            .child(titlebar)
            // Body: sidebar + content — offset by titlebar height
            .child(
                div()
                    .flex()
                    .flex_1()
                    .pt(px(titlebar::TITLEBAR_HEIGHT))
                    .overflow_hidden()
                    .child(sidebar)
                    .child(div().flex().flex_col().flex_1().overflow_hidden().child(content)),
            )
    }
}
