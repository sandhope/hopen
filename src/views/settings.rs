/// Settings / Tools page view and sub-page views (language, theme).

use gpui::*;
use gpui::prelude::*;

use crate::i18n::{I18nStrings, language_display_name};
use crate::navigation::ToolsSubPage;
use crate::theme::Theme;
use crate::{
    save_theme_mode, save_language_id, AppState,
};

use super::shared::settings_item;

// ─── Tools / Settings ─────────────────────────────────────────

pub(super) fn tools_view(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let settings_groups = div()
        .flex()
        .flex_col()
        .gap(px(4.0))
        .px(px(24.0))
        .child(language_entry_item(theme, cx, strings))
        .child(theme_toggle_item(theme, cx, strings))
        .child(settings_item(strings.settings_basic_config, strings.settings_basic_config_subtitle, theme))
        .child(settings_item(strings.settings_advanced_config, strings.settings_advanced_config_subtitle, theme))
        .child(settings_item(strings.settings_hotkeys, strings.settings_hotkeys_subtitle, theme))
        .child(settings_item(strings.settings_backup_restore, strings.settings_backup_restore_subtitle, theme))
        .child(settings_item(strings.settings_about, strings.settings_about_subtitle, theme));

    settings_groups
}

// ─── Language Entry (Settings Row → Sub-page) ─────────────────

/// A clickable settings row that navigates to the language selection sub-page.
fn language_entry_item(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let current_lang = cx.global::<crate::i18n::I18nManager>().current_language_id.clone();
    let display = language_display_name(&current_lang);

    div()
        .id("settings-language-entry")
        .flex()
        .items_center()
        .justify_between()
        .px(px(16.0))
        .py(px(14.0))
        .rounded(px(6.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgb(theme.surface)))
        .on_click(cx.listener(|this, _, _, cx| {
            this.tools_sub_page = Some(ToolsSubPage::Language);
            cx.notify();
        }))
        .child(
            div().flex().flex_col().gap(px(2.0)).child(
                div()
                    .text_size(px(14.0))
                    .text_color(rgb(theme.text_primary))
                    .child(strings.settings_language),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(theme.text_secondary))
                    .child(display),
            ),
        )
        .child(
            div()
                .text_size(px(14.0))
                .text_color(rgb(theme.text_disabled))
                .child("\u{203A}"), // ›
        )
}

// ─── Language Sub-page Body ───────────────────────────────────

pub(super) fn language_sub_page_body(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let current_lang = cx.global::<crate::i18n::I18nManager>().current_language_id.clone();

    div()
        .flex()
        .flex_col()
        .gap(px(2.0))
        .px(px(24.0))
        .child(
            div()
                .px(px(16.0))
                .py(px(8.0))
                .text_size(px(12.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme.text_secondary))
                .child(strings.settings_language),
        )
        .child(lang_option_row(theme, "en-US", "English", &current_lang))
        .child(lang_option_row(theme, "zh-CN", "简体中文", &current_lang))
        .child(lang_option_row(theme, "ja-JP", "日本語", &current_lang))
        .child(lang_option_row(theme, "ko-KR", "한국어", &current_lang))
        .child(lang_option_row(theme, "de-DE", "Deutsch", &current_lang))
        .child(lang_option_row(theme, "fr-FR", "Français", &current_lang))
        .child(lang_option_row(theme, "es-ES", "Español", &current_lang))
        .child(lang_option_row(theme, "pt-BR", "Português", &current_lang))
}

/// Single language option row for the sub-page.
fn lang_option_row(
    theme: &Theme,
    lang_id: &'static str,
    lang_name: &'static str,
    current_lang: &str,
) -> impl IntoElement {
    let is_active = lang_id == current_lang;
    let lang_id_owned = lang_id.to_string();
    let lang_name = lang_name.to_string();

    div()
        .id(lang_id)
        .flex()
        .items_center()
        .gap(px(10.0))
        .px(px(16.0))
        .py(px(12.0))
        .rounded(px(6.0))
        .cursor_pointer()
        .hover(|s| {
            s.bg(rgb(theme.surface))
        })
        .on_click(move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
            crate::i18n::I18nManager::init_with_language_id(cx, &lang_id_owned);
            save_language_id(&lang_id_owned);
            cx.refresh_windows();
        })
        .child(
            div()
                .w(px(18.0))
                .h(px(18.0))
                .rounded(px(9.0))
                .border_1()
                .border_color(if is_active {
                    Hsla::from(rgb(theme.accent))
                } else {
                    Hsla::from(rgb(theme.border))
                })
                .bg(if is_active {
                    Hsla::from(rgb(theme.accent))
                } else {
                    Hsla::from(rgba(0x00000000))
                })
                .flex()
                .items_center()
                .justify_center()
                .when(is_active, |s| {
                    s.child(div().w(px(6.0)).h(px(6.0)).rounded(px(3.0)).bg(rgb(0xffffff)))
                }),
        )
        .child(
            div()
                .text_size(px(14.0))
                .text_color(if is_active {
                    Hsla::from(rgb(theme.accent))
                } else {
                    Hsla::from(rgb(theme.text_primary))
                })
                .font_weight(if is_active {
                    FontWeight::SEMIBOLD
                } else {
                    FontWeight::NORMAL
                })
                .child(lang_name),
        )
}

// ─── Theme Toggle (interactive) ────────────────────────────────

fn theme_toggle_item(
    theme: &Theme,
    _cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let accent_color = rgb(theme.accent);
    let title = strings.settings_theme;
    let subtitle = strings.settings_theme_subtitle;
    div()
        .id("theme-toggle")
        .flex()
        .items_center()
        .justify_between()
        .px(px(16.0))
        .py(px(14.0))
        .rounded(px(6.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgb(theme.surface)))
        .on_click(|_: &ClickEvent, _: &mut Window, cx: &mut App| {
            cx.update_global::<AppState, _>(|state, _cx| {
                state.theme_mode = state.theme_mode.toggle();
                save_theme_mode(state.theme_mode);
            });
            cx.refresh_windows();
        })
        .child(
            div().flex().flex_col().gap(px(2.0)).child(
                div()
                    .text_size(px(14.0))
                    .text_color(accent_color)
                    .child(title),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(theme.text_secondary))
                    .child(subtitle),
            ),
        )
        .child(
            div()
                .text_size(px(14.0))
                .text_color(rgb(theme.text_disabled))
                .child("\u{203A}"), // ›
        )
}
