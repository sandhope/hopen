/// Chart components built with GPUI native elements.
///
/// GPUI 0.2.2 lacks Canvas 2D / Path drawing, so all charts are rendered
/// using positioned divs, borders, and shape tricks rather than vector
/// graphics. The result is slightly less precise than Flutter's CustomPaint
/// but is functional and themable.

use gpui::*;

use crate::theme::Theme;

// ═══════════════════════════════════════════════════════════════════
//  Bar Chart
// ═══════════════════════════════════════════════════════════════════

/// A single bar in the bar chart.
#[derive(Clone, Debug)]
pub struct BarChartData {
    pub value: f32,
    pub label: String,
    pub color: u32,
}

/// Render a horizontal bar chart.
///
/// Bars are rendered as coloured rectangles whose widths are proportional
/// to the maximum value in the data set. Each bar has a label on the left
/// and a value badge on the right.
pub fn bar_chart(data: &[BarChartData], theme: &Theme) -> impl IntoElement {
    let max_val = data
        .iter()
        .fold(0.0_f32, |m, d| m.max(d.value))
        .max(1.0);

    // Pre-compute bar elements.
    let bars: Vec<AnyElement> = data
        .iter()
        .map(|d| {
            let pct = (d.value / max_val) * 100.0;
            let label = d.label.clone();
            let value_str = format_value(d.value);
            let color = d.color;
            let bar_w = pct.max(2.0);

            div()
                .flex()
                .items_center()
                .gap(px(8.0))
                .h(px(28.0))
                .child(
                    div()
                        .w(px(60.0))
                        .text_size(px(11.0))
                        .text_color(rgb(theme.text_secondary))
                        .truncate()
                        .child(label),
                )
                .child(
                    div()
                        .flex_1()
                        .h(px(16.0))
                        .rounded(px(4.0))
                        .bg(rgb(theme.surface_variant))
                        .child(
                            div()
                                .h_full()
                                .rounded(px(4.0))
                                .bg(rgb(color))
                                // Approximate px from percentage (flex_1() ≈ 200px).
                                .w(px((bar_w / 100.0) * 200.0)),
                        ),
                )
                .child(
                    div()
                        .w(px(48.0))
                        .text_size(px(11.0))
                        .text_color(rgb(theme.text_primary))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_align(TextAlign::Right)
                        .child(value_str),
                )
                .into_any_element()
        })
        .collect();

    div().flex().flex_col().gap(px(6.0)).children(bars)
}

// ═══════════════════════════════════════════════════════════════════
//  Donut Chart
// ═══════════════════════════════════════════════════════════════════

/// A segment/slice in the donut chart.
#[derive(Clone, Debug)]
pub struct DonutChartData {
    pub value: f32,
    pub color: u32,
    pub label: String,
}

/// Render a donut-like summary using a circular progress ring and a
/// colour-coded legend. This is a simplified approximation of the
/// Flutter `DonutChart` (which uses Canvas arcs).
pub fn donut_chart(data: &[DonutChartData], theme: &Theme) -> impl IntoElement {
    let total: f32 = data.iter().map(|d| d.value).sum();
    let ring_size = 120.0;
    let stroke = 12.0;

    // Build legend items.
    let legend: Vec<_> = data
        .iter()
        .map(|d| {
            let label = d.label.clone();
            let pct = if total > 0.0 {
                format!("{:.1}%", (d.value / total) * 100.0)
            } else {
                "0%".to_string()
            };
            let color = d.color;

            div()
                .flex()
                .items_center()
                .gap(px(6.0))
                .child(div().w(px(8.0)).h(px(8.0)).rounded(px(2.0)).bg(rgb(color)))
                .child(
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(theme.text_secondary))
                        .flex_1()
                        .child(label),
                )
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(theme.text_primary))
                        .child(pct),
                )
        })
        .collect();

    // Build a simplified ring: just a coloured circle with border.
    // Use the first data item's colour as the ring colour, or accent.
    let ring_color = data.first().map(|d| d.color).unwrap_or(theme.accent);

    div()
        .flex()
        .items_center()
        .gap(px(20.0))
        .child(
            // Circular ring indicator.
            div()
                .flex()
                .items_center()
                .justify_center()
                .w(px(ring_size))
                .h(px(ring_size))
                .rounded_full()
                .border(px(stroke))
                .border_color(rgb(ring_color))
                .child(
                    div()
                        .text_size(px(20.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(theme.text_primary))
                        .child(format_value(total)),
                ),
        )
        .child(
            // Legend column.
            div().flex().flex_col().gap(px(6.0)).children(legend),
        )
}

// ═══════════════════════════════════════════════════════════════════
//  Line Chart
// ═══════════════════════════════════════════════════════════════════

/// A data point for the line chart.
#[derive(Clone, Debug)]
pub struct ChartPoint {
    pub x: f32,
    pub y: f32,
}

/// Render a line chart using absolute-positioned dots and connecting
/// div lines. Data points are normalised to fit the chart area.
pub fn line_chart(
    points: &[ChartPoint],
    theme: &Theme,
    color: u32,
    show_area: bool,
) -> AnyElement {
    let chart_w = 300.0;
    let chart_h = 120.0;
    let dot_radius = 3.0;

    if points.is_empty() {
        return div().h(px(chart_h)).into_any_element();
    }

    // Normalise to [0, 1] range.
    let (min_x, max_x) = points.iter().fold((f32::MAX, f32::MIN), |(mn, mx), p| {
        (mn.min(p.x), mx.max(p.x))
    });
    let (min_y, max_y) = points.iter().fold((f32::MAX, f32::MIN), |(mn, mx), p| {
        (mn.min(p.y), mx.max(p.y))
    });

    let dx = if (max_x - min_x).abs() < 0.0001 { 1.0 } else { max_x - min_x };
    let dy = if (max_y - min_y).abs() < 0.0001 { 1.0 } else { max_y - min_y };

    let normalized: Vec<(f32, f32)> = points
        .iter()
        .map(|p| ((p.x - min_x) / dx, (p.y - min_y) / dy))
        .collect();

    // Generate line segments and dots.
    let mut segments: Vec<AnyElement> = Vec::new();
    let mut dots: Vec<AnyElement> = Vec::new();
    let mut area_segments: Vec<AnyElement> = Vec::new();

    let pad = dot_radius + 2.0;
    let usable_w = chart_w - pad * 2.0;
    let usable_h = chart_h - pad * 2.0;

    for i in 0..normalized.len() {
        let (nx, ny) = normalized[i];
        let x = pad + nx * usable_w;
        let y = pad + (1.0 - ny) * usable_h; // flip Y (0 = bottom)

        // Dot at each point.
        dots.push(
            div()
                .absolute()
                .left(px(x - dot_radius))
                .top(px(y - dot_radius))
                .w(px(dot_radius * 2.0))
                .h(px(dot_radius * 2.0))
                .rounded_full()
                .bg(rgb(color))
                .into_any_element(),
        );

        // Line segment to next point.
        if i + 1 < normalized.len() {
            let (nx2, ny2) = normalized[i + 1];
            let x2 = pad + nx2 * usable_w;
            let y2 = pad + (1.0 - ny2) * usable_h;

            let seg_dx = x2 - x;
            let seg_dy = y2 - y;
            let length = (seg_dx * seg_dx + seg_dy * seg_dy).sqrt();
            let _angle = seg_dy.atan2(seg_dx);

            if length > 0.5 {
                segments.push(
                    div()
                        .absolute()
                        .left(px(x))
                        .top(px(y))
                        .w(px(length))
                        .h(px(2.0))
                        .bg(rgb(color))
                        .rounded(px(1.0))
                        // Approximate rotation via a slight visual shift.
                        // (GPUI 0.2.2 has no transform/rotate — we accept
                        //  the line may not perfectly connect diagonal points.)
                        .into_any_element(),
                );
            }

            // Area fill: coloured strip below each point.
            if show_area {
                area_segments.push(
                    div()
                        .absolute()
                        .left(px(x))
                        .top(px(y))
                        .w(px(x2 - x + dot_radius))
                        .h(px(chart_h - y))
                        .bg(rgba((color >> 8) | 0x00000019)) // ~10% opacity
                        .into_any_element(),
                );
            }
        }
    }

    div()
        .flex()
        .flex_col()
        .child(
            // Chart area.
            div()
                .relative()
                .w(px(chart_w))
                .h(px(chart_h))
                .rounded(px(8.0))
                .bg(rgb(theme.surface))
                .children(area_segments)
                .children(segments)
                .children(dots),
        )
        .into_any_element()
}

// ═══════════════════════════════════════════════════════════════════
//  Wave View
// ═══════════════════════════════════════════════════════════════════

/// Render a simplified wave form using a stack of overlapping circles
/// to approximate a sine-wave pattern.
pub fn wave_view(theme: &Theme, color: u32, amplitude: f32) -> impl IntoElement {
    let h = 80.0;
    let w = 300.0;
    let wave_count = 8;
    let step = w / wave_count as f32;
    let amp = amplitude.clamp(10.0, 40.0);

    // Build a sine-wave approximation with overlapping circles.
    let mut circles: Vec<AnyElement> = Vec::new();
    for i in 0..=wave_count {
        let x = i as f32 * step;
        let phase = i as f32 / wave_count as f32 * std::f32::consts::PI * 2.0;
        let y = h * 0.5 + amp * phase.sin();

        circles.push(
            div()
                .absolute()
                .left(px(x - amp))
                .top(px(y - amp))
                .w(px(amp * 2.0))
                .h(px(h + amp))
                .rounded_full()
                .bg(rgba((color >> 8) | 0x00000030))
                .into_any_element(),
        );
    }

    div()
        .relative()
        .w(px(w))
        .h(px(h))
        .overflow_hidden()
        .rounded(px(8.0))
        .bg(rgb(theme.surface))
        .children(circles)
}

// ═══════════════════════════════════════════════════════════════════
//  Helpers
// ═══════════════════════════════════════════════════════════════════

/// Format a numeric value for display (B, KB, MB, GB).
fn format_value(val: f32) -> String {
    if val >= 1_000_000_000.0 {
        format!("{:.2} GB", val / 1_000_000_000.0)
    } else if val >= 1_000_000.0 {
        format!("{:.2} MB", val / 1_000_000.0)
    } else if val >= 1_000.0 {
        format!("{:.2} KB", val / 1_000.0)
    } else {
        format!("{:.0} B", val)
    }
}
