/// Placeholder views for pages whose content is not yet implemented.

use gpui::*;

use crate::i18n::I18nStrings;
use crate::theme::Theme;

use super::shared::placeholder_section;

// ─── Profiles ──────────────────────────────────────────────────

pub(super) fn profiles_view(theme: &Theme, strings: &I18nStrings) -> impl IntoElement {
    placeholder_section(
        strings.placeholder_profiles_title,
        strings.placeholder_profiles_desc,
        theme,
    )
}

// ─── Connections ───────────────────────────────────────────────

pub(super) fn connections_view(theme: &Theme, strings: &I18nStrings) -> impl IntoElement {
    placeholder_section(
        strings.placeholder_connections_title,
        strings.placeholder_connections_desc,
        theme,
    )
}

// ─── Resources ─────────────────────────────────────────────────

pub(super) fn resources_view(theme: &Theme, strings: &I18nStrings) -> impl IntoElement {
    placeholder_section(
        strings.placeholder_resources_title,
        strings.placeholder_resources_desc,
        theme,
    )
}
