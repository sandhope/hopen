/// Settings / Tools page view and sub-page views (language, theme).

use gpui::*;
use gpui::prelude::*;

use crate::i18n::{I18nStrings, language_display_name};
use crate::navigation::ToolsSubPage;
use crate::theme::{AccentColor, Theme, ThemeMode};
use crate::{
    save_theme_mode, save_language_id, save_accent_color, AppState,
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
        .child(theme_entry_item(theme, cx, strings))
        .child(settings_item("svg/basic-config.svg", strings.settings_basic_config, strings.settings_basic_config_subtitle, theme))
        .child(settings_item("svg/advanced-config.svg", strings.settings_advanced_config, strings.settings_advanced_config_subtitle, theme))
        .child(settings_item("svg/hotkeys.svg", strings.settings_hotkeys, strings.settings_hotkeys_subtitle, theme))
        .child(settings_item("svg/backup-restore.svg", strings.settings_backup_restore, strings.settings_backup_restore_subtitle, theme))
        .child(settings_item("svg/about.svg", strings.settings_about, strings.settings_about_subtitle, theme));

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

    settings_nav_row(
        "settings-language-entry",
        "svg/language.svg",
        strings.settings_language,
        display,
        theme,
        cx.listener(|this, _, _, cx| {
            this.tools_sub_page = Some(ToolsSubPage::Language);
            cx.notify();
        }),
    )
}

// ─── Theme Entry (Settings Row → Sub-page) ───────────────────

/// A clickable settings row that navigates to the theme sub-page.
fn theme_entry_item(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let state = cx.global::<AppState>();
    let mode_label = match state.theme_mode {
        ThemeMode::Dark => strings.theme_dark,
        ThemeMode::Light => strings.theme_light,
        ThemeMode::System => strings.theme_system,
    };

    settings_nav_row(
        "settings-theme-entry",
        "svg/theme.svg",
        strings.settings_theme,
        mode_label,
        theme,
        cx.listener(|this, _, _, cx| {
            this.tools_sub_page = Some(ToolsSubPage::Theme);
            cx.notify();
        }),
    )
}

/// Reusable settings row with icon, title, subtitle, and chevron.
fn settings_nav_row(
    id: &'static str,
    icon: &'static str,
    title: &str,
    subtitle: &str,
    theme: &Theme,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let title = title.to_string();
    let subtitle = subtitle.to_string();

    div()
        .id(id)
        .flex()
        .items_center()
        .justify_between()
        .px(px(16.0))
        .py(px(14.0))
        .rounded(px(6.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgb(theme.surface)))
        .on_click(on_click)
        .child(
            div().flex().items_center().gap(px(12.0))
                .child(
                    svg()
                        .path(icon)
                        .size(px(18.0))
                        .text_color(rgb(theme.text_secondary)),
                )
                .child(
                    div().flex().flex_col().gap(px(2.0))
                        .child(
                            div()
                                .text_size(px(14.0))
                                .text_color(rgb(theme.text_primary))
                                .child(title),
                        )
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(theme.text_secondary))
                                .child(subtitle),
                        ),
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
                .flex()
                .items_center()
                .gap(px(6.0))
                .px(px(16.0))
                .py(px(8.0))
                .text_size(px(13.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme.text_secondary))
                .child(
                    svg()
                        .path("svg/language.svg")
                        .size(px(16.0))
                        .text_color(rgb(theme.text_secondary)),
                )
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
        .child(radio_dot(theme, is_active))
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

/// Small radio-dot indicator used by language/theme option rows.
fn radio_dot(theme: &Theme, is_active: bool) -> impl IntoElement {
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
        })
}

// ─── Theme Sub-page Body ──────────────────────────────────────

pub(super) fn theme_sub_page_body(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
    strings: &I18nStrings,
) -> impl IntoElement {
    let state = cx.global::<AppState>();
    let current_mode = state.theme_mode;
    let current_accent = state.accent_color;

    div()
        .flex()
        .flex_col()
        .gap(px(12.0))
        .px(px(24.0))
        // ── Section: Theme Mode ───────────────────────────
        .child(section_label(theme, "svg/theme.svg", strings.theme_mode_label))
        .child(theme_mode_row(
            theme,
            strings.theme_system,
            current_mode == ThemeMode::System,
            ThemeMode::System,
        ))
        .child(theme_mode_row(
            theme,
            strings.theme_light,
            current_mode == ThemeMode::Light,
            ThemeMode::Light,
        ))
        .child(theme_mode_row(
            theme,
            strings.theme_dark,
            current_mode == ThemeMode::Dark,
            ThemeMode::Dark,
        ))
        // ── Section: Accent Color ───────────────────────────
        .child(div().h(px(8.0)))
        .child(section_label(theme, "svg/palette.svg", strings.accent_color_label))
        .child(accent_color_grid(theme, current_accent))
}

/// Section header label with optional icon (muted, bold, small).
fn section_label(theme: &Theme, icon: &'static str, label: &str) -> impl IntoElement {
    let label = label.to_string();
    div()
        .flex()
        .items_center()
        .gap(px(6.0))
        .px(px(16.0))
        .py(px(8.0))
        .text_size(px(13.0))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(rgb(theme.text_secondary))
        .child(
            svg()
                .path(icon)
                .size(px(16.0))
                .text_color(rgb(theme.text_secondary)),
        )
        .child(label)
}

/// A selectable row for theme mode (Light / Dark).
fn theme_mode_row(
    theme: &Theme,
    label: &str,
    is_active: bool,
    mode: ThemeMode,
) -> impl IntoElement {
    let label = label.to_string();
    let id = match mode {
        ThemeMode::Light => "theme-mode-light",
        ThemeMode::Dark => "theme-mode-dark",
        ThemeMode::System => "theme-mode-system",
    };
    div()
        .id(id)
        .flex()
        .items_center()
        .gap(px(10.0))
        .px(px(16.0))
        .py(px(12.0))
        .rounded(px(6.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgb(theme.surface)))
        .on_click(move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
            cx.update_global::<AppState, _>(|state, _cx| {
                state.theme_mode = mode;
                save_theme_mode(mode);
            });
            cx.refresh_windows();
        })
        .child(radio_dot(theme, is_active))
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
                .child(label),
        )
}

/// Grid of accent colour swatches with the active one highlighted.
fn accent_color_grid(theme: &Theme, current: AccentColor) -> impl IntoElement {
    div()
        .flex()
        .flex_wrap()
        .gap(px(10.0))
        .px(px(16.0))
        .children(AccentColor::ALL.iter().map(move |&accent| {
            accent_swatch(theme, accent, accent == current)
        }))
}

/// A clickable colour circle for one accent preset.
fn accent_swatch(
    theme: &Theme,
    accent: AccentColor,
    is_active: bool,
) -> impl IntoElement {
    let swatch_color = accent.swatch();
    let size = if is_active { 36.0 } else { 32.0 };
    let border_color = if is_active {
        Hsla::from(rgb(theme.text_primary))
    } else {
        Hsla::from(rgba(0x00000000))
    };

    let id = accent.label();

    div()
        .id(id)
        .w(px(40.0))
        .h(px(40.0))
        .flex()
        .items_center()
        .justify_center()
        .rounded(px(8.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgb(theme.surface)))
        .on_click(move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
            cx.update_global::<AppState, _>(|state, _cx| {
                state.accent_color = accent;
                save_accent_color(accent);
            });
            cx.refresh_windows();
        })
        .child(
            div()
                .w(px(size))
                .h(px(size))
                .rounded(px(size / 2.0))
                .bg(rgb(swatch_color))
                .border_2()
                .border_color(border_color),
        )
}
