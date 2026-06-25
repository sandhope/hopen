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
use crate::views::search_input::SearchInput;

/// The root view that composes sidebar navigation with page content.
pub struct AppView {
    /// Currently active page — drives both sidebar highlight and content rendering.
    pub current_page: Page,
    /// Active sub-page within Settings (Tools). `None` means show the main settings list.
    pub tools_sub_page: Option<ToolsSubPage>,
    /// Search text for the Proxies page filter.
    pub proxies_search_text: String,
    /// Which proxy groups are expanded: group-name → expanded.
    pub proxies_expanded: std::collections::HashMap<String, bool>,
    /// Dedicated search input entity with its own focus handle.
    pub search_input_entity: Entity<SearchInput>,
    /// Whether we've already auto-focused the search input on this page visit.
    pub proxies_search_focused: bool,
}

impl AppView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_input_entity = cx.new(|cx| SearchInput::new(window, cx));

        Self {
            current_page: Page::Dashboard,
            tools_sub_page: None,
            proxies_search_text: String::new(),
            proxies_expanded: std::collections::HashMap::new(),
            search_input_entity,
            proxies_search_focused: false,
        }
    }
}

impl Render for AppView {
    fn render(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = current_theme(cx);
        let strings = cx.global::<I18nManager>().strings_arc();

        // Auto-focus the search input when navigating to the Proxies page.
        if self.current_page == Page::Proxies && !self.proxies_search_focused {
            self.proxies_search_focused = true;
            self.search_input_entity.update(cx, |input, cx| {
                input.focus(window, cx);
            });
        }
        // Reset the flag when leaving the Proxies page.
        if self.current_page != Page::Proxies {
            self.proxies_search_focused = false;
        }

        // Build sidebar first (consumes cx borrow), then content.
        let sidebar = sidebar::render_sidebar(self.current_page, cx, &theme, &strings);
        let content = views::render_page(
            self.current_page,
            self.tools_sub_page,
            &theme,
            cx,
            &self.proxies_search_text,
            &self.proxies_expanded,
            &self.search_input_entity,
        );
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
