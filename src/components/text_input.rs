/// TextInput — a self-contained text input entity for GPUI.
///
/// Manages its own text, cursor, focus, and keyboard input.
/// Uses a custom `CursorField` Element to render text with a blinking cursor
/// (standard GPUI approach — Element::paint + paint_quad).
///
/// Usage:
/// ```ignore
/// // In AppView::new:
/// let search = cx.new(|cx| TextInput::new("Search...", window, cx));
///
/// // In view function:
/// let text = search.read(cx).text().to_string(); // for filtering
/// div().child(search.read(cx).render(theme, cx))  // renders the input
/// ```

use std::time::Duration;

use gpui::*;

use crate::theme::{Theme, CARD_RADIUS};

// ─── Platform‑specific cursor width ──────────────────────────────

#[cfg(not(target_os = "macos"))]
const CURSOR_WIDTH_PX: f32 = 2.0;
#[cfg(target_os = "macos")]
const CURSOR_WIDTH_PX: f32 = 1.5;

const BLINK_INTERVAL: Duration = Duration::from_millis(500);
const BLINK_PAUSE: Duration = Duration::from_millis(300);


// ═══════════════════════════════════════════════════════════════════
//  BlinkCursor  –  blinking cursor state machine
// ═══════════════════════════════════════════════════════════════════

/// Manages the blinking visibility state of the text cursor.
///
/// - Interval: 500 ms toggle
/// - On user input: `pause()` → cursor stays visible 300 ms, then resumes
/// - `start()` / `stop()` on focus‑in / focus‑out
pub(crate) struct BlinkCursor {
    visible: bool,
    paused: bool,
    epoch: usize,
    _task: Task<()>,
}

impl BlinkCursor {
    pub fn new() -> Self {
        Self {
            visible: false,
            paused: false,
            epoch: 0,
            _task: Task::ready(()),
        }
    }

    /// Start blinking (called on focus).
    pub fn start(&mut self, cx: &mut Context<Self>) {
        self.visible = true;
        cx.notify();
        let epoch = self.epoch + 1;
        self.epoch = epoch;
        self.blink(epoch, cx);
    }

    /// Stop blinking (called on blur).
    pub fn stop(&mut self, _cx: &mut Context<Self>) {
        self.epoch = 0;
        self.visible = false;
    }

    /// Pause blinking for 300 ms after user input.
    pub fn pause(&mut self, cx: &mut Context<Self>) {
        self.paused = true;
        self.visible = true;
        cx.notify();
        let epoch = self.epoch + 1;
        self.epoch = epoch;
        self._task = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(BLINK_PAUSE).await;
            if let Some(this) = this.upgrade() {
                let _ = this.update(cx, |this, cx| {
                    this.paused = false;
                    this.blink(epoch, cx);
                });
            }
        });
    }

    /// Core blink loop.
    fn blink(&mut self, epoch: usize, cx: &mut Context<Self>) {
        if self.paused || epoch != self.epoch {
            self.visible = true;
            return;
        }
        self.visible = !self.visible;
        cx.notify();
        let next_epoch = self.epoch + 1;
        self.epoch = next_epoch;
        self._task = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(BLINK_INTERVAL).await;
            if let Some(this) = this.upgrade() {
                let _ = this.update(cx, |this, cx| this.blink(next_epoch, cx));
            }
        });
    }

    /// Is the cursor currently visible?
    pub fn visible(&self) -> bool {
        self.paused || self.visible
    }
}


// ═══════════════════════════════════════════════════════════════════
//  TextInput  –  self‑contained text input entity
// ═══════════════════════════════════════════════════════════════════

/// A self-contained text input that owns its text, cursor, focus, and keyboard
/// handling.
pub struct TextInput {
    text: String,
    /// Byte offset of the cursor within `text`.
    cursor_offset: usize,
    /// Tracked focus state (updated via focus subscriptions).
    focused: bool,
    focus_handle: FocusHandle,
    placeholder: String,
    /// Optional font family override (e.g. "monospace" for URL inputs).
    font_family: Option<&'static str>,
    blink_cursor: Entity<BlinkCursor>,
    /// Must store subscriptions so they live as long as the entity.
    _focus_in_sub: Subscription,
    _focus_out_sub: Subscription,
}

impl TextInput {
    /// Create a new text input with the given placeholder.
    pub fn new(
        placeholder: impl Into<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let blink_cursor = cx.new(|_cx| BlinkCursor::new());
        let focus_handle = cx.focus_handle();

        // Subscribe to focus‑in → start blink
        let fh = focus_handle.clone();
        let blink = blink_cursor.clone();
        let focus_in_sub = cx.on_focus_in(&fh, window, move |this, _window, cx| {
            this.focused = true;
            blink.update(cx, |b, cx| b.start(cx));
            cx.notify();
        });

        // Subscribe to blur → stop blink
        let fh = focus_handle.clone();
        let blink = blink_cursor.clone();
        let focus_out_sub = cx.on_blur(&fh, window, move |this, _window, cx| {
            this.focused = false;
            blink.update(cx, |b, cx| b.stop(cx));
            cx.notify();
        });

        Self {
            text: String::new(),
            cursor_offset: 0,
            focused: false,
            focus_handle,
            placeholder: placeholder.into(),
            font_family: None,
            blink_cursor,
            _focus_in_sub: focus_in_sub,
            _focus_out_sub: focus_out_sub,
        }
    }

    /// Set a custom font family (e.g. "monospace").
    pub fn set_font_family(&mut self, family: &'static str) {
        self.font_family = Some(family);
    }

    /// Get the current text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set the text content, moving cursor to the end.
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.cursor_offset = self.text.len();
    }

    /// Clear the text and reset cursor.
    pub fn clear(&mut self) {
        self.text.clear();
        self.cursor_offset = 0;
    }

    /// Focus this input on the given window.
    pub fn focus(&self, window: &mut Window, _cx: &mut Context<Self>) {
        self.focus_handle.focus(window);
    }

    /// Render the interactive input element with search icon.
    pub fn render(&self, theme: &Theme, cx: &mut Context<Self>) -> AnyElement {
        self.build_element(theme, cx, true)
    }

    /// Render without the search icon (for URL inputs, etc.).
    pub fn render_plain(&self, theme: &Theme, cx: &mut Context<Self>) -> AnyElement {
        self.build_element(theme, cx, false)
    }

    /// Internal: build the input element.
    fn build_element(
        &self,
        theme: &Theme,
        cx: &mut Context<Self>,
        show_icon: bool,
    ) -> AnyElement {
        let focus_handle = self.focus_handle.clone();
        let has_text = !self.text.is_empty();

        let display = self.text.clone();
        let placeholder = self.placeholder.clone();
        let cursor_offset = self.cursor_offset;
        let focused = self.focused;

        let blink_visible = self.blink_cursor.read(cx).visible();
        // Cursor shows when focused AND blink says visible (even with empty text).
        let show_cursor = focused && blink_visible;

        let text_color = if has_text {
            Hsla::from(rgb(theme.text_primary))
        } else {
            Hsla::from(rgb(theme.text_disabled))
        };

        // ── CursorField: custom Element that renders text + blinking cursor ──
        let cursor_field = CursorField {
            text: if has_text { display } else { placeholder },
            is_placeholder: !has_text,
            cursor_offset: if has_text { cursor_offset } else { 0 },
            show_cursor,
            text_color,
            font_size: px(13.0),
        };

        let mut element = div()
            .flex()
            .items_center()
            .gap(px(8.0))
            .flex_1()
            .px(px(12.0))
            .py(px(8.0))
            .rounded(px(CARD_RADIUS))
            .bg(rgb(theme.surface))
            .border_1()
            .border_color(rgb(theme.border_light))
            .in_focus(|style| style.border_color(rgb(theme.accent)))
            .cursor_pointer()
            .track_focus(&focus_handle)
            .on_key_down(cx.listener(
                move |this, event: &KeyDownEvent, window, cx| {
                    this.focus_handle.focus(window);
                    if let Some(ref ch) = event.keystroke.key_char {
                        this.text.push_str(ch);
                    } else {
                        match event.keystroke.key.as_str() {
                            "backspace" => {
                                if this.cursor_offset > 0 {
                                    this.text.pop();
                                }
                            }
                            "space" => {
                                this.text.push(' ');
                            }
                            _ => {}
                        }
                    }
                    // Always place cursor at end (simple single‑line behaviour)
                    this.cursor_offset = this.text.len();
                    // Pause blink after every keystroke
                    this.blink_cursor.update(cx, |blink, cx| blink.pause(cx));
                    cx.notify();
                },
            ));

        if show_icon {
            element = element.child(
                div()
                    .text_size(px(14.0))
                    .text_color(rgb(theme.text_disabled))
                    .flex_shrink_0()
                    .child("\u{1F50D}"),
            );
        }

        element.flex_1().child(cursor_field).into_any_element()
    }
}

impl Focusable for TextInput {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}


// ═══════════════════════════════════════════════════════════════════
//  CursorField  –  custom GPUI Element (text + blinking cursor)
// ═══════════════════════════════════════════════════════════════════
//
//  Lifecycle:
//   1. request_layout → single‑line height
//   2. prepaint       → shape text via text_system, compute cursor bounds
//   3. paint          → paint the shaped line, then paint cursor as a quad
//
//  This is the *standard* GPUI approach — same as gpui‑component's
//  `TextElement`.

struct CursorField {
    text: String,
    is_placeholder: bool,
    cursor_offset: usize,
    show_cursor: bool,
    text_color: Hsla,
    font_size: Pixels,
}

/// Pre‑computed render data for `CursorField`.
struct CursorFieldPrepaint {
    /// The fully shaped text line (used to paint the text glyphs).
    shaped_line: Option<ShapedLine>,
    /// Absolute pixel bounds of the blinking cursor (empty = hidden).
    cursor_bounds: Option<Bounds<Pixels>>,
    line_height: Pixels,
    /// The element's layout bounds (origin + size).
    bounds: Bounds<Pixels>,
}

// ── IntoElement boilerplate ──────────────────────────────────────

impl IntoElement for CursorField {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

// ── Element trait ────────────────────────────────────────────────

impl Element for CursorField {
    type RequestLayoutState = ();
    type PrepaintState = CursorFieldPrepaint;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let line_height = window.line_height();
        let mut style = Style::default();
        // Fill available width; fixed height = one line.
        style.size.width = relative(1.).into();
        style.size.height = line_height.into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
        let line_height = window.line_height();
        let text_style = window.text_style();
        let font = text_style.font();

        let mut shaped_line = None;
        let mut cursor_bounds = None;

        if !self.text.is_empty() {
            let run = TextRun {
                len: self.text.len(),
                font,
                color: self.text_color,
                background_color: None,
                underline: None,
                strikethrough: None,
            };
            let runs = vec![run];

            let text: SharedString = self.text.clone().into();

            // Shape the line so we can measure cursor position.
            let line = window.text_system().shape_line(
                text,
                self.font_size,
                &runs,
                None, // single‑line → no forced width
            );

            // Compute cursor X from byte offset.
            if self.show_cursor && self.cursor_offset <= self.text.len() {
                let cursor_x = line.x_for_index(self.cursor_offset);
                cursor_bounds = Some(Bounds::new(
                    point(
                        bounds.origin.x + cursor_x,
                        bounds.origin.y,
                    ),
                    size(px(CURSOR_WIDTH_PX), line_height),
                ));
            }
            shaped_line = Some(line);
        } else if self.show_cursor {
            // Empty text: cursor sits at the left edge.
            cursor_bounds = Some(Bounds::new(
                point(bounds.origin.x, bounds.origin.y),
                size(px(CURSOR_WIDTH_PX), line_height),
            ));
        }

        CursorFieldPrepaint {
            shaped_line,
            cursor_bounds,
            line_height,
            bounds,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector: Option<&gpui::InspectorElementId>,
        _input_bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let origin = prepaint.bounds.origin;

        // 1. Paint the text glyphs
        if let Some(ref line) = prepaint.shaped_line {
            let _ = line.paint(origin, prepaint.line_height, window, cx);
        }

        // 2. Paint the blinking cursor as a filled rectangle
        if let Some(bounds) = prepaint.cursor_bounds {
            window.paint_quad(fill(bounds, self.text_color));
        }
    }
}
