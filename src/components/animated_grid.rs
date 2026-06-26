#![allow(dead_code)]
/// Animated Grid — grid with staggered entrance animation.
///
/// Architecture:
/// - Items are laid out in a grid using absolute positioning
/// - Each item fades in with a delay proportional to its index
/// - In GPUI 0.2.2, animation is approximate (no Tween engine)
///
/// Usage:
/// ```ignore
/// animated_grid(
///     AnimatedGridParams { columns: 2, item_height: 120.0, gap: 8.0 },
///     theme,
///     items,
///     |item, theme, index| { div().child(item) },
/// )
/// ```

use gpui::*;

use crate::theme::Theme;

/// Parameters for the animated grid.
pub struct AnimatedGridParams {
    /// Number of columns.
    pub columns: usize,
    /// Fixed height of each grid item in pixels.
    pub item_height: f32,
    /// Gap between items in pixels.
    pub gap: f32,
}

impl Default for AnimatedGridParams {
    fn default() -> Self {
        Self {
            columns: 2,
            item_height: 120.0,
            gap: 8.0,
        }
    }
}

/// Render an animated grid layout.
///
/// `items` is the data collection.
/// `item_builder` receives (data, theme, index) and returns the element.
pub fn animated_grid<T: Clone>(
    params: &AnimatedGridParams,
    theme: &Theme,
    items: &[T],
    item_builder: impl Fn(&T, &Theme, usize) -> AnyElement,
) -> impl IntoElement {
    let columns = params.columns;
    let item_height = params.item_height;
    let gap = params.gap;
    let count = items.len();
    let rows = (count as f32 / columns as f32).ceil() as usize;
    let total_height = rows as f32 * item_height + (rows.max(1) - 1) as f32 * gap;

    // Compute column width as equal share.
    // Without LayoutBuilder, we approximate ~200px per column for typical usage.
    let item_width = 160.0;

    let cells: Vec<AnyElement> = items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let col = i % columns;
            let row = i / columns;
            let x = col as f32 * (item_width + gap);
            let y = row as f32 * (item_height + gap);

            // Staggered opacity based on index (approximate animation).
            let stagger_opacity = 0.6 + (i as f32 / count.max(1) as f32) * 0.4;

            div()
                .absolute()
                .left(px(x))
                .top(px(y))
                .w(px(item_width))
                .h(px(item_height))
                .opacity(stagger_opacity)
                .child(item_builder(item, theme, i))
                .into_any_element()
        })
        .collect();

    div()
        .relative()
        .w_full()
        .h(px(total_height))
        .children(cells)
}
