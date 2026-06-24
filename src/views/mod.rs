/// Page view skeletons for the Hopen GPUI client.
///
/// Each function returns a view for the corresponding page.
/// Theme-aware: all colours come from the `Theme` parameter.

use gpui::*;

use crate::navigation::Page;
use crate::theme::Theme;
use crate::{save_theme_mode, AppState};

/// Route to the correct page view based on the current navigation state.
///
/// `cx` is the AppView context, needed for interactive elements (e.g. theme toggle).
pub fn render_page(
    page: Page,
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
) -> impl IntoElement + use<> {
    let title = page.title();
    let description = page.description();

    let header = div()
        .flex()
        .flex_col()
        .gap(px(4.0))
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
        .child(
            div()
                .text_size(px(13.0))
                .text_color(rgb(theme.text_secondary))
                .child(description),
        );

    let content = match page {
        Page::Dashboard => div().child(dashboard_view(theme)),
        Page::Proxies => div().child(proxies_view(theme)),
        Page::Profiles => div().child(profiles_view(theme)),
        Page::Requests => div().child(requests_view(theme)),
        Page::Connections => div().child(connections_view(theme)),
        Page::Resources => div().child(resources_view(theme)),
        Page::Logs => div().child(logs_view(theme)),
        Page::Tools => div().child(tools_view(theme, cx)),
    };

    div()
        .flex()
        .flex_col()
        .size_full()
        .overflow_y_hidden()
        .child(header)
        .child(content)
}

// ─── Dashboard ─────────────────────────────────────────────────

fn dashboard_view(theme: &Theme) -> impl IntoElement {
    // Placeholder: grid of status cards
    let cards = div()
        .flex()
        .flex_wrap()
        .gap(px(12.0))
        .px(px(24.0))
        .child(status_card("Status", "Not Connected", theme.status_warning, theme))
        .child(status_card("Upload", "0 B/s", theme.status_info, theme))
        .child(status_card("Download", "0 B/s", theme.status_info, theme))
        .child(status_card(
            "Active Conns",
            "0",
            theme.text_secondary,
            theme,
        ));

    let quick_actions = div()
        .flex()
        .flex_col()
        .gap(px(8.0))
        .px(px(24.0))
        .pt(px(20.0))
        .child(
            div()
                .text_size(px(14.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme.text_primary))
                .child("Quick Actions"),
        )
        .child(
            div()
                .flex()
                .gap(px(8.0))
                .child(action_button("Start Core", theme))
                .child(action_button("System Proxy", theme))
                .child(action_button("TUN Mode", theme)),
        );

    div().flex().flex_col().child(cards).child(quick_actions)
}

// ─── Proxies ───────────────────────────────────────────────────

fn proxies_view(theme: &Theme) -> impl IntoElement {
    placeholder_section(
        "Proxy Groups",
        "Proxy groups will appear here when the core is connected.",
        theme,
    )
}

// ─── Profiles ──────────────────────────────────────────────────

fn profiles_view(theme: &Theme) -> impl IntoElement {
    placeholder_section(
        "Profiles",
        "Import or add subscription profiles to get started.",
        theme,
    )
}

// ─── Requests ──────────────────────────────────────────────────

fn requests_view(theme: &Theme) -> impl IntoElement {
    placeholder_section(
        "Request Timeline",
        "Real-time request tracking will appear here.",
        theme,
    )
}

// ─── Connections ───────────────────────────────────────────────

fn connections_view(theme: &Theme) -> impl IntoElement {
    placeholder_section(
        "Active Connections",
        "Active connections will be listed here.",
        theme,
    )
}

// ─── Resources ─────────────────────────────────────────────────

fn resources_view(theme: &Theme) -> impl IntoElement {
    placeholder_section(
        "Resources",
        "GeoIP, GeoSite, and other resource files will be managed here.",
        theme,
    )
}

// ─── Logs ──────────────────────────────────────────────────────

fn logs_view(theme: &Theme) -> impl IntoElement {
    placeholder_section(
        "Core Logs",
        "Logs from the proxy core will stream here.",
        theme,
    )
}

// ─── Tools / Settings ─────────────────────────────────────────

fn tools_view(
    theme: &Theme,
    cx: &mut Context<crate::app::AppView>,
) -> impl IntoElement {
    let settings_groups = div()
        .flex()
        .flex_col()
        .gap(px(4.0))
        .px(px(24.0))
        .child(theme_toggle_item(theme, cx))
        .child(settings_item("Basic Config", "Port, log level, mode", theme))
        .child(settings_item(
            "Advanced Config",
            "DNS, TUN, rules",
            theme,
        ))
        .child(settings_item("Hotkeys", "Keyboard shortcuts", theme))
        .child(settings_item("Backup & Restore", "WebDAV sync", theme))
        .child(settings_item("About", "Version and license info", theme));

    settings_groups
}

// ─── Theme Toggle (interactive) ────────────────────────────────

fn theme_toggle_item(
    theme: &Theme,
    _cx: &mut Context<crate::app::AppView>,
) -> impl IntoElement {
    let accent_color = rgb(theme.accent);
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
                    .child("Theme"),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(theme.text_secondary))
                    .child("Dark / Light — tap to switch appearance"),
            ),
        )
        .child(
            div()
                .text_size(px(14.0))
                .text_color(rgb(theme.text_disabled))
                .child("\u{203A}"), // ›
        )
}

// ─── Shared Helper Components ──────────────────────────────────

/// A reusable status card for the dashboard.
fn status_card(label: &str, value: &str, value_color: u32, theme: &Theme) -> impl IntoElement {
    let label = label.to_string();
    let value = value.to_string();
    div()
        .flex()
        .flex_col()
        .gap(px(6.0))
        .p(px(16.0))
        .w(px(180.0))
        .rounded(px(crate::theme::CARD_RADIUS))
        .bg(rgb(theme.surface))
        .border_1()
        .border_color(rgb(theme.border_light))
        .child(
            div()
                .text_size(px(12.0))
                .text_color(rgb(theme.text_secondary))
                .child(label),
        )
        .child(
            div()
                .text_size(px(18.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(value_color))
                .child(value),
        )
}

/// A placeholder button for quick actions.
fn action_button(label: &str, theme: &Theme) -> impl IntoElement {
    let label = label.to_string();
    div()
        .px(px(16.0))
        .py(px(8.0))
        .rounded(px(6.0))
        .bg(rgb(theme.surface))
        .border_1()
        .border_color(rgb(theme.border))
        .text_size(px(13.0))
        .text_color(rgb(theme.text_primary))
        .cursor_pointer()
        .hover(|s| s.bg(rgb(theme.surface_variant)))
        .child(label)
}

/// A reusable placeholder section with title and description.
fn placeholder_section(title: &str, description: &str, theme: &Theme) -> impl IntoElement {
    let title = title.to_string();
    let description = description.to_string();
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .py(px(60.0))
        .px(px(24.0))
        .gap(px(8.0))
        .child(
            div()
                .text_size(px(40.0))
                .text_color(rgb(theme.text_disabled))
                .child("\u{1F527}"),
        )
        .child(
            div()
                .text_size(px(16.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(theme.text_secondary))
                .child(title),
        )
        .child(
            div()
                .text_size(px(13.0))
                .text_color(rgb(theme.text_disabled))
                .child(description),
        )
}

/// A static settings list item with title and subtitle.
fn settings_item(title: &str, subtitle: &str, theme: &Theme) -> impl IntoElement {
    let title = title.to_string();
    let subtitle = subtitle.to_string();
    div()
        .flex()
        .items_center()
        .justify_between()
        .px(px(16.0))
        .py(px(14.0))
        .rounded(px(6.0))
        .cursor_pointer()
        .hover(|s| s.bg(rgb(theme.surface)))
        .child(
            div().flex().flex_col().gap(px(2.0)).child(
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
        )
        .child(
            div()
                .text_size(px(14.0))
                .text_color(rgb(theme.text_disabled))
                .child("\u{203A}"), // ›
        )
}
