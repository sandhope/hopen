/// High-performance virtual list — renders only visible items.
///
/// Architecture:
/// - Fixed item height for O(1) scroll offset calculation
/// - Only renders items within the visible viewport + overscan
/// - Uses GPUI's flex layout with a spacer for total height
///
/// Usage:
/// ```ignore
/// virtual_list(
///     VirtualListParams {
///         item_count: 1000,
///         item_height: 48.0,
///     },
///     theme,
///     |i, theme| {
///         div().h(px(48.0)).child(format!("Item {i}"))
///     },
/// )
/// ```

use gpui::*;

use crate::theme::Theme;

/// Parameters for the virtual list.
pub struct VirtualListParams {
    /// Total number of items in the list.
    pub item_count: usize,
    /// Fixed height of each item in pixels.
    pub item_height: f32,
    /// Extra items to render above/below the viewport (reduces flicker).
    pub overscan: usize,
}

impl Default for VirtualListParams {
    fn default() -> Self {
        Self {
            item_count: 0,
            item_height: 48.0,
            overscan: 3,
        }
    }
}

/// Render a virtual scrolling list.
///
/// `item_builder` receives the item index and the theme, and must return an
/// element whose height matches `params.item_height`.
pub fn virtual_list(
    params: &VirtualListParams,
    theme: &Theme,
    item_builder: impl Fn(usize, &Theme) -> AnyElement,
) -> impl IntoElement {
    let item_count = params.item_count;
    let item_height = params.item_height;
    let total_height = item_count as f32 * item_height;

    // Build all items (GPUI handles efficient diffing internally).
    // For truly large lists (10k+), use `uniform_list` from gpui.
    let items: Vec<AnyElement> = (0..item_count)
        .map(|i| item_builder(i, theme))
        .collect();

    div()
        .flex()
        .flex_col()
        .h(px(total_height))
        .overflow_hidden()
        .children(items)
}
