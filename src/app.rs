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
use crate::views::LogLevelFilter;
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
    /// Search text for the Requests page filter.
    pub requests_search_text: String,
    /// Currently selected request index in the Requests page.
    pub requests_selected_index: Option<usize>,
    /// Search text for the Logs page filter.
    pub logs_search_text: String,
    /// Active level filter for the Logs page.
    pub logs_filter_level: LogLevelFilter,
    /// Search text for the Connections page filter.
    pub connections_search_text: String,
    /// Currently selected connection index in the Connections page.
    pub connections_selected_index: Option<usize>,
    /// Current sidebar width (pixels), adjustable via drag.
    pub sidebar_width: f32,
    /// Whether the user is currently dragging the sidebar resize handle.
    sidebar_resizing: bool,
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
            requests_search_text: String::new(),
            requests_selected_index: None,
            logs_search_text: String::new(),
            logs_filter_level: LogLevelFilter::All,
            connections_search_text: String::new(),
            connections_selected_index: None,
            sidebar_width: crate::theme::SIDEBAR_WIDTH,
            sidebar_resizing: false,
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

        // Capture entity before mutable cx borrows (for mouse event closures).
        let entity = cx.entity();

        // Build sidebar first (consumes cx borrow), then content.
        let sidebar_width = self.sidebar_width;
        let sidebar = sidebar::render_sidebar(
            self.current_page, cx, &theme, &strings, sidebar_width,
        );
        let content = views::render_page(
            self.current_page,
            self.tools_sub_page,
            &theme,
            cx,
            &self.proxies_search_text,
            &self.proxies_expanded,
            &self.search_input_entity,
            &self.requests_search_text,
            self.requests_selected_index,
            &self.logs_search_text,
            self.logs_filter_level,
            &self.connections_search_text,
            self.connections_selected_index,
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
            // Body: sidebar + resize-handle + content — offset by titlebar height.
            //
            // Resize logic uses raw mouse events (gpui 0.2.2 lacks `on_drag`
            // on `Div`):
            //   1. on_any_mouse_down on handle → set sidebar_resizing = true
            //   2. on_mouse_move on body → if resizing, update sidebar_width
            //      from mouse x coordinate
            //   3. capture_any_mouse_up on body → set sidebar_resizing = false
            .child(
                div()
                    .flex()
                    .flex_1()
                    .pt(px(titlebar::TITLEBAR_HEIGHT))
                    .overflow_hidden()
                    .on_mouse_move({
                        let entity = entity.clone();
                        move |event: &MouseMoveEvent, _window, app| {
                            entity.update(app, |this, _| {
                                if this.sidebar_resizing {
                                    let current_x: f32 = event.position.x.into();
                                    this.sidebar_width =
                                        current_x.clamp(160.0, 360.0);
                                }
                            });
                        }
                    })
                    .capture_any_mouse_up({
                        let entity = entity.clone();
                        move |_event: &MouseUpEvent, _window, app| {
                            entity.update(app, |this, _| {
                                this.sidebar_resizing = false;
                            });
                        }
                    })
                    .child(sidebar)
                    .child(
                        // Resize handle: drag left/right to adjust sidebar width.
                        div()
                            .w(px(4.0))
                            .h_full()
                            .cursor(CursorStyle::ResizeLeftRight)
                            .hover(|s| s.bg(rgba(0x00000010)))
                            .on_any_mouse_down({
                                let entity = entity.clone();
                                move |_event: &MouseDownEvent, _window, app| {
                                    entity.update(app, |this, _| {
                                        this.sidebar_resizing = true;
                                    });
                                }
                            }),
                    )
                    .child(div().flex().flex_col().flex_1().overflow_hidden().child(content)),
            )
    }
}
