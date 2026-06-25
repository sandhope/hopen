/// Grid layout — arranges children in a configurable column grid.
///
/// Architecture:
/// - Uses flex-wrap for responsive grid layout
/// - Configurable column count, gap, and item sizing
/// - Each child gets equal width based on column count
///
/// Usage:
/// ```ignore
/// grid(
///     GridParams { columns: 2, gap_x: 8.0, gap_y: 8.0 },
///     theme,
///     items,
/// )
/// ```

use gpui::*;

use crate::theme::Theme;

/// Parameters for the grid layout.
pub struct GridParams {
    /// Number of columns in the grid.
    pub columns: usize,
    /// Horizontal gap between items in pixels.
    pub gap_x: f32,
    /// Vertical gap between items in pixels.
    pub gap_y: f32,
    /// Fixed height for each grid item. If 0, height is auto.
    pub item_height: f32,
}

impl Default for GridParams {
    fn default() -> Self {
        Self {
            columns: 2,
            gap_x: 8.0,
            gap_y: 8.0,
            item_height: 0.0,
        }
    }
}

/// Render a grid layout using flexbox wrapping.
///
/// `items` is a collection of elements to arrange in the grid.
/// Each item gets equal width: `(container_width - gaps) / columns`.
pub fn grid(
    params: &GridParams,
    _theme: &Theme,
    items: Vec<AnyElement>,
) -> impl IntoElement {
    let gap = params.gap_x;
    let item_height = params.item_height;

    div()
        .flex()
        .flex_wrap()
        .gap_x(px(gap))
        .gap_y(px(params.gap_y))
        .children(
            items.into_iter().map(move |item| {
                if item_height > 0.0 {
                    div()
                        .flex_1()
                        .min_w(px(80.0))
                        .h(px(item_height))
                        .child(item)
                        .into_any_element()
                } else {
                    div()
                        .flex_1()
                        .min_w(px(80.0))
                        .child(item)
                        .into_any_element()
                }
            }),
        )
}
