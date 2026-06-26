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
pub mod settings;
mod shared;

use gpui::*;

use crate::components::text_input::TextInput;
use crate::i18n::I18nStrings;
use crate::navigation::{Page, ToolsSubPage};
use crate::theme::Theme;
pub use logs::LogLevelFilter;

/// Route to the correct page view based on the current navigation state.
pub fn render_page(
    page: Page,
    tools_sub_page: Option<ToolsSubPage>,
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    proxies_expanded: &std::collections::HashMap<String, bool>,
    proxies_search: &Entity<TextInput>,
    requests_search: &Entity<TextInput>,
    requests_selected_index: Option<usize>,
    logs_search: &Entity<TextInput>,
    logs_filter_level: LogLevelFilter,
    connections_search: &Entity<TextInput>,
    connections_selected_index: Option<usize>,
    profiles_search: &Entity<TextInput>,
    profiles_selected_index: Option<usize>,
    profiles_show_add: bool,
    profiles_add_mode: profiles::AddMode,
    profiles_url_input: &Entity<TextInput>,
    profiles_detail_tab: profiles::DetailTab,
    profiles_overwrite_sub_tab: profiles::OverwriteSubTab,
    resources_state: &resources::ResourcesState,
    resources_edit_input: &Entity<TextInput>,
    settings_data: &settings::SettingsData,
) -> impl IntoElement {
    let strings = cx.global::<crate::i18n::I18nManager>().strings_arc();

    // Compute header and content, accounting for Settings sub-pages.
    let (header_elem, content_elem): (AnyElement, AnyElement) = if page == Page::Tools {
        if let Some(sub) = tools_sub_page {
            let header = render_sub_page_header(sub, theme, cx, &strings).into_any_element();
            let body = render_sub_page_body(sub, theme, cx, &strings, settings_data);
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
            Page::Proxies => proxies::proxies_view(theme, cx, &strings, proxies_expanded, proxies_search).into_any_element(),
            Page::Profiles => profiles::profiles_view(theme, cx, &strings, profiles_selected_index, profiles_show_add, profiles_add_mode, profiles_detail_tab, profiles_overwrite_sub_tab, profiles_search, profiles_url_input).into_any_element(),
            Page::Requests => requests::requests_view(theme, cx, &strings, requests_selected_index, requests_search).into_any_element(),
            Page::Connections => connections::connections_view(theme, cx, &strings, connections_selected_index, connections_search).into_any_element(),
            Page::Resources => resources::resources_view(theme, cx, &strings, resources_state, resources_edit_input).into_any_element(),
            Page::Logs => logs::logs_view(theme, cx, &strings, logs_filter_level, logs_search).into_any_element(),
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
        .child(
            div()
                .id("page-content-scroll")
                .flex().flex_col()
                .flex_1()
                .overflow_y_scroll()
                .child(content_elem)
        )
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
    let title = strings.tools_sub_page_title(sub);
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
    settings_data: &settings::SettingsData,
) -> AnyElement {
    let data = settings_data.clone();
    match sub {
        ToolsSubPage::Language => settings::language_sub_page_body(theme, cx, strings).into_any_element(),
        ToolsSubPage::Theme => settings::theme_sub_page_body(theme, cx, strings).into_any_element(),
        ToolsSubPage::BasicConfig => settings::basic_config_sub_page_body(theme, data, cx, strings).into_any_element(),
        ToolsSubPage::NetworkConfig => settings::network_config_sub_page_body(theme, data, cx, strings).into_any_element(),
        ToolsSubPage::DnsConfig => settings::dns_config_sub_page_body(theme, data, cx, strings).into_any_element(),
        ToolsSubPage::RulesConfig => settings::rules_config_sub_page_body(theme, data, cx, strings).into_any_element(),
        ToolsSubPage::ScriptsConfig => settings::scripts_config_sub_page_body(theme, data, cx, strings).into_any_element(),
        ToolsSubPage::AdvancedConfig => settings::advanced_config_sub_page_body(theme, data, cx, strings).into_any_element(),
        ToolsSubPage::OnDemand => settings::on_demand_sub_page_body(theme, data, cx, strings).into_any_element(),
        ToolsSubPage::Hotkeys => settings::hotkeys_sub_page_body(theme, data, cx, strings).into_any_element(),
        ToolsSubPage::BackupRestore => settings::backup_restore_sub_page_body(theme, data, cx, strings).into_any_element(),
        ToolsSubPage::Disclaimer => settings::disclaimer_sub_page_body(theme, data, cx, strings).into_any_element(),
        ToolsSubPage::About => settings::about_sub_page_body(theme, data, cx, strings).into_any_element(),
    }
}
