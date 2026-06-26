/// Root application view for Hopen.
///
/// `AppView` owns the navigation state and renders the sidebar + content layout.
/// This is the top-level entity created for the main window.

use gpui::*;

use crate::components::dialog::{self, DialogParams};
use crate::components::sidebar;
use crate::components::titlebar;
use crate::components::toast::{self, ToastData};
use crate::current_theme;
use crate::i18n::I18nManager;
use crate::navigation::{Page, ToolsSubPage};
use crate::components::text_input::TextInput;
use crate::views;
use crate::views::LogLevelFilter;
use crate::views::resources::ResourcesState;
use crate::views::settings::SettingsData;

/// The root view that composes sidebar navigation with page content.
pub struct AppView {
    /// Currently active page — drives both sidebar highlight and content rendering.
    pub current_page: Page,
    /// Active sub-page within Settings (Tools). `None` means show the main settings list.
    pub tools_sub_page: Option<ToolsSubPage>,
    /// Which proxy groups are expanded: group-name → expanded.
    pub proxies_expanded: std::collections::HashMap<String, bool>,
    /// Search input entity for the Proxies page.
    pub proxies_search: Entity<TextInput>,
    /// Search input entity for the Requests page.
    pub requests_search: Entity<TextInput>,
    /// Currently selected request index in the Requests page.
    pub requests_selected_index: Option<usize>,
    /// Search input entity for the Logs page.
    pub logs_search: Entity<TextInput>,
    /// Active level filter for the Logs page.
    pub logs_filter_level: LogLevelFilter,
    /// Search input entity for the Connections page.
    pub connections_search: Entity<TextInput>,
    /// Currently selected connection index in the Connections page.
    pub connections_selected_index: Option<usize>,
    /// Search input entity for the Profiles page.
    pub profiles_search: Entity<TextInput>,
    /// Currently selected profile index in the Profiles page.
    pub profiles_selected_index: Option<usize>,
    /// Whether the Add Subscription panel is shown.
    pub profiles_show_add: bool,
    /// URL input entity for the Add Subscription panel.
    pub profiles_url_input: Entity<TextInput>,
    /// Active tab in the profiles detail panel.
    pub profiles_detail_tab: Option<crate::views::profiles::DetailTab>,
    /// Active sub-tab in the profiles overwrite section.
    pub profiles_overwrite_sub_tab: Option<crate::views::profiles::OverwriteSubTab>,
    /// Resources page state (GeoIP, GeoSite, MMDB, ASN management).
    pub resources_state: ResourcesState,
    /// Text input entity for editing resource URLs.
    pub resources_edit_input: Entity<TextInput>,
    /// Settings page state (all editable settings values).
    pub settings_data: SettingsData,
    /// Current sidebar width (pixels), adjustable via drag.
    pub sidebar_width: f32,
    /// Whether the user is currently dragging the sidebar resize handle.
    sidebar_resizing: bool,
    /// Active dialog overlay. When `Some`, a modal dialog is rendered on top.
    pub active_dialog: Option<DialogParams>,
    /// Active toast notifications. Dismissed manually via close button.
    pub toasts: Vec<ToastData>,
    /// Whether we've already auto-focused the search input on this page visit.
    search_focused: bool,
}

impl AppView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let i18n = cx.global::<I18nManager>();
        let s = i18n.strings_arc();

        let proxies_search = cx.new(|cx| TextInput::new(s.proxy_search_placeholder, window, cx));
        let requests_search = cx.new(|cx| TextInput::new(s.requests_search_placeholder, window, cx));
        let logs_search = cx.new(|cx| TextInput::new(s.logs_search_placeholder, window, cx));
        let connections_search = cx.new(|cx| TextInput::new(s.connections_search_placeholder, window, cx));
        let profiles_search = cx.new(|cx| TextInput::new(s.profiles_search_placeholder, window, cx));
        let profiles_url_input = cx.new(|cx| TextInput::new(s.profiles_url_placeholder, window, cx));
        let resources_edit_input = cx.new(|cx| {
            let mut ti = TextInput::new(s.resources_url_placeholder, window, cx);
            ti.set_font_family("monospace");
            ti
        });

        Self {
            current_page: Page::Dashboard,
            tools_sub_page: None,
            proxies_expanded: std::collections::HashMap::new(),
            proxies_search,
            requests_search,
            requests_selected_index: None,
            logs_search,
            logs_filter_level: LogLevelFilter::All,
            connections_search,
            connections_selected_index: None,
            profiles_search,
            profiles_selected_index: None,
            profiles_show_add: false,
            profiles_url_input,
            profiles_detail_tab: None,
            profiles_overwrite_sub_tab: None,
            resources_state: ResourcesState::new(),
            resources_edit_input,
            settings_data: SettingsData::default(),
            sidebar_width: crate::theme::SIDEBAR_WIDTH,
            sidebar_resizing: false,
            active_dialog: None,
            toasts: Vec::new(),
            search_focused: false,
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

        // Auto-focus the search input when navigating to pages with search.
        let should_focus_search = matches!(
            self.current_page,
            Page::Proxies | Page::Requests | Page::Connections | Page::Logs | Page::Profiles
        );
        if should_focus_search && !self.search_focused {
            self.search_focused = true;
            let search_entity = match self.current_page {
                Page::Proxies => Some(&self.proxies_search),
                Page::Requests => Some(&self.requests_search),
                Page::Connections => Some(&self.connections_search),
                Page::Logs => Some(&self.logs_search),
                Page::Profiles => Some(&self.profiles_search),
                _ => None,
            };
            if let Some(entity) = search_entity {
                entity.update(cx, |input, cx| {
                    input.focus(window, cx);
                });
            }
        }
        if !should_focus_search {
            self.search_focused = false;
        }

        // Initialize resources edit input when opening the edit dialog.
        if self.resources_state.editing_index.is_some()
            && self.resources_edit_input.read(cx).text().is_empty()
        {
            let url = self
                .resources_state
                .items
                .get(self.resources_state.editing_index.unwrap())
                .map(|r| r.url.to_string())
                .unwrap_or_default();
            self.resources_edit_input.update(cx, |input, _| {
                input.set_text(url);
            });
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
            &self.proxies_expanded,
            &self.proxies_search,
            &self.requests_search,
            self.requests_selected_index,
            &self.logs_search,
            self.logs_filter_level,
            &self.connections_search,
            self.connections_selected_index,
            &self.profiles_search,
            self.profiles_selected_index,
            self.profiles_show_add,
            &self.profiles_url_input,
            self.profiles_detail_tab.unwrap_or(views::profiles::DetailTab::Info),
            self.profiles_overwrite_sub_tab.unwrap_or(views::profiles::OverwriteSubTab::Standard),
            &self.resources_state,
            &self.resources_edit_input,
            &self.settings_data,
        );
        let titlebar = titlebar::render_titlebar(&theme, window, &strings);

        // Build overlay elements (dialog + toasts).
        // `Entity` is not Copy, so clone it for each usage.
        let dialog_overlay: Option<AnyElement> = self.active_dialog.as_ref().map(|params| {
            let params = dialog::DialogParams {
                kind: params.kind,
                title: params.title.clone(),
                body: params.body.clone(),
                primary_label: params.primary_label.clone(),
                secondary_label: params.secondary_label.clone(),
            };
            dialog::render_dialog(&params, &theme, entity.clone()).into_any_element()
        });

        let toast_overlay: Option<AnyElement> =
            toast::render_toasts(&self.toasts, &theme, entity.clone())
                .map(|el| el.into_any_element());

        // Collect all direct children of the root div.
        let mut children: Vec<AnyElement> = Vec::new();
        children.push(titlebar.into_any_element());
        // Body: sidebar + resize-handle + content — offset by titlebar height.
        //
        // Resize logic uses raw mouse events (gpui 0.2.2 lacks `on_drag`
        // on `Div`):
        //   1. on_any_mouse_down on handle → set sidebar_resizing = true
        //   2. on_mouse_move on body → if resizing, update sidebar_width
        //      from mouse x coordinate
        //   3. capture_any_mouse_up on body → set sidebar_resizing = false
        children.push(
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
                .child(div().flex().flex_col().flex_1().overflow_hidden().child(content))
                .into_any_element(),
        );

        // Append overlay children (dialog → toast stack).
        if let Some(dlg) = dialog_overlay {
            children.push(dlg);
        }
        if let Some(toast) = toast_overlay {
            children.push(toast);
        }

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(theme.content_bg))
            .text_color(rgb(theme.text_primary))
            .children(children)
    }
}
