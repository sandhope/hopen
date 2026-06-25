/// Page view skeletons for the Hopen GPUI client.
///
/// Each submodule provides the view builders for a logical group of pages.
/// Theme-aware: all colours come from the `Theme` parameter.
///
/// Module structure:
/// - `mod.rs`       — routing dispatch + page header helpers
/// - `shared.rs`    — reusable widgets (placeholder_section, settings_item, back_button)
/// - `dashboard.rs` — dashboard page with all sub-cards
/// - `proxies.rs`   — proxy groups, node cards, delay test, search, providers
/// - `settings.rs`  — tools/settings page, language selector, theme toggle
/// - `placeholders.rs` — stub views for unimplemented pages

mod connections;
mod dashboard;
mod logs;
mod placeholders;
pub mod profiles;
mod proxies;
mod requests;
pub mod resources;
pub mod search_input;
mod settings;
mod shared;

use gpui::*;

use crate::i18n::I18nStrings;
use crate::navigation::{Page, ToolsSubPage};
use crate::theme::Theme;
pub use logs::LogLevelFilter;
use search_input::SearchInput;

/// Route to the correct page view based on the current navigation state.
///
/// `cx` is the AppView context, needed for interactive elements (e.g. theme toggle).
/// `tools_sub_page` is the active drill-down sub-page within Settings, or `None`
/// for the main settings list.
pub fn render_page(
    page: Page,
    tools_sub_page: Option<ToolsSubPage>,
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    proxies_search_text: &str,
    proxies_expanded: &std::collections::HashMap<String, bool>,
    search_input_entity: &Entity<SearchInput>,
    requests_search_text: &str,
    requests_selected_index: Option<usize>,
    logs_search_text: &str,
    logs_filter_level: LogLevelFilter,
    connections_search_text: &str,
    connections_selected_index: Option<usize>,
    profiles_search_text: &str,
    profiles_selected_index: Option<usize>,
    profiles_show_add: bool,
    profiles_add_url: &str,
    profiles_detail_tab: profiles::DetailTab,
    profiles_overwrite_sub_tab: profiles::OverwriteSubTab,
    resources_state: &resources::ResourcesState,
) -> impl IntoElement {
    let strings = cx.global::<crate::i18n::I18nManager>().strings_arc();

    // Compute header and content, accounting for Settings sub-pages.
    // We collect both into AnyElement so the outer div has a single concrete return type.
    let (header_elem, content_elem): (AnyElement, AnyElement) = if page == Page::Tools {
        if let Some(sub) = tools_sub_page {
            let header = render_sub_page_header(sub, theme, cx, &strings).into_any_element();
            let body = render_sub_page_body(sub, theme, cx, &strings);
            (header, body)
        } else {
            let title = strings.page_title(page);
            (
                page_header(title, theme).into_any_element(),
                settings::tools_view(theme, cx, &strings).into_any_element(),
            )
        }
    } else {
        let title = strings.page_title(page);
        let content: AnyElement = match page {
            Page::Dashboard => dashboard::dashboard_view(theme, cx, &strings).into_any_element(),
            Page::Proxies => proxies::proxies_view(theme, cx, &strings, proxies_search_text, proxies_expanded, search_input_entity).into_any_element(),
            Page::Profiles => profiles::profiles_view(theme, cx, &strings, profiles_search_text, profiles_selected_index, profiles_show_add, profiles_add_url, profiles_detail_tab, profiles_overwrite_sub_tab).into_any_element(),
            Page::Requests => requests::requests_view(theme, cx, &strings, requests_search_text, requests_selected_index).into_any_element(),
            Page::Connections => connections::connections_view(theme, cx, &strings, connections_search_text, connections_selected_index).into_any_element(),
            Page::Resources => resources::resources_view(theme, cx, &strings, resources_state).into_any_element(),
            Page::Logs => logs::logs_view(theme, cx, &strings, logs_search_text, logs_filter_level).into_any_element(),
            Page::Tools => unreachable!(),
        };
        (page_header(title, theme).into_any_element(), content)
    };

    div()
        .flex()
        .flex_col()
        .size_full()
        .overflow_y_hidden()
        .child(header_elem)
        .child(content_elem)
}

/// Standard page title header (used by all non-sub pages).
fn page_header(title: &str, theme: &Theme) -> impl IntoElement {
    let title = title.to_string();
    div()
        .flex()
        .px(px(24.0))
        .pt(px(24.0))
        .pb(px(16.0))
        .child(
            div()
                .text_size(px(22.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(theme.text_primary))
                .child(title),
        )
}

/// Header with back button for Settings sub-pages.
fn render_sub_page_header(
    sub: ToolsSubPage,
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let title = match sub {
        ToolsSubPage::Language => strings.page_title_language,
        ToolsSubPage::Theme => strings.page_title_theme,
    };
    div()
        .flex()
        .items_center()
        .justify_between()
        .px(px(24.0))
        .pt(px(24.0))
        .pb(px(16.0))
        .child(
            div()
                .text_size(px(22.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(theme.text_primary))
                .child(title),
        )
        .child(shared::back_button(theme, cx))
}

/// Body content for Settings sub-pages.
fn render_sub_page_body(
    sub: ToolsSubPage,
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> AnyElement {
    match sub {
        ToolsSubPage::Language => settings::language_sub_page_body(theme, cx, strings).into_any_element(),
        ToolsSubPage::Theme => settings::theme_sub_page_body(theme, cx, strings).into_any_element(),
    }
}
