/// Placeholder views for pages whose content is not yet implemented.

use gpui::*;

use crate::i18n::I18nStrings;
use crate::theme::Theme;

use super::shared::placeholder_section;

// ─── Resources ─────────────────────────────────────────────────

pub(super) fn resources_view(theme: &Theme, strings: &I18nStrings) -> impl IntoElement {
    placeholder_section(
        strings.placeholder_resources_title,
        strings.placeholder_resources_desc,
        theme,
    )
}
