/// Card component — a reusable surface container with optional header/body/footer.
///
/// Architecture:
/// - Standard card surface with border-radius, background, border
/// - Optional header section (title + icon + actions)
/// - Required body section (flexible content area)
/// - Optional footer section
///
/// Usage:
/// ```ignore
/// card(
///     &CardParams { title: Some("Settings"), .. },
///     theme,
///     |_theme| div().child("Content"),
/// )
/// ```

use gpui::*;

use crate::theme::Theme;

/// Parameters for rendering a card.
pub struct CardParams<'a> {
    /// Optional title in the card header.
    pub title: Option<&'a str>,
    /// Optional subtitle below the title.
    pub subtitle: Option<&'a str>,
    /// Optional leading icon text (emoji or unicode character).
    pub icon: Option<&'a str>,
    /// Whether the card uses a compact padding style.
    pub compact: bool,
    /// Whether the card has a bottom border.
    pub bordered: bool,
}

impl<'a> Default for CardParams<'a> {
    fn default() -> Self {
        Self {
            title: None,
            subtitle: None,
            icon: None,
            compact: false,
            bordered: true,
        }
    }
}

/// Render a standard card container.
///
/// `content` receives the theme reference and must return the card body.
pub fn card(
    params: &CardParams,
    theme: &Theme,
    content: impl FnOnce(&Theme) -> AnyElement,
) -> impl IntoElement {
    let padding = if params.compact {
        px(16.0)
    } else {
        px(20.0)
    };

    let gap = if params.compact {
        px(8.0)
    } else {
        px(12.0)
    };

    let title_size = if params.compact {
        px(13.0)
    } else {
        px(14.0)
    };

    // Build header (if title is provided).
    let header: Option<AnyElement> = params.title.map(|title| {
        let subtitle_el: Option<AnyElement> = params.subtitle.map(|sub| {
            div()
                .text_size(px(11.0))
                .text_color(rgb(theme.text_secondary))
                .child(sub.to_string())
                .into_any_element()
        });

        let mut header_row = div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .pb(padding);

        if let Some(icon) = params.icon {
            header_row = header_row.child(
                div()
                    .text_size(px(16.0))
                    .text_color(rgb(theme.text_secondary))
                    .child(icon.to_string()),
            );
        }

        header_row = header_row.child(
            div().flex().flex_col().flex_1().child(
                div()
                    .text_size(title_size)
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(theme.text_primary))
                    .child(title.to_string()),
            ),
        );

        if let Some(sub) = subtitle_el {
            header_row = header_row.child(sub);
        }

        header_row.into_any_element()
    });

    let mut card = div()
        .flex()
        .flex_col()
        .gap(gap)
        .p(padding)
        .rounded(px(12.0))
        .bg(rgb(theme.surface));

    if params.bordered {
        card = card.border_1().border_color(rgb(theme.border_light));
    }

    if let Some(h) = header {
        card = card.child(h);
    }

    card.child(content(theme))
}
