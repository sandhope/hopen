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

mod dashboard;
mod placeholders;
mod proxies;
pub mod search_input;
mod settings;
mod shared;

use gpui::*;

use crate::i18n::I18nStrings;
use crate::navigation::{Page, ToolsSubPage};
use crate::theme::Theme;
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
            Page::Profiles => placeholders::profiles_view(theme, &strings).into_any_element(),
            Page::Requests => placeholders::requests_view(theme, &strings).into_any_element(),
            Page::Connections => placeholders::connections_view(theme, &strings).into_any_element(),
            Page::Resources => placeholders::resources_view(theme, &strings).into_any_element(),
            Page::Logs => placeholders::logs_view(theme, &strings).into_any_element(),
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
