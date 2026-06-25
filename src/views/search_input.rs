/// Search input entity — a minimal entity that owns a FocusHandle
/// so that keyboard focus is managed independently from the parent view.
///
/// Keyboard input is handled in the parent view's `on_key_down` listener.

use gpui::*;

// ─── Entity ───────────────────────────────────────────────────────

pub struct SearchInput {
    focus_handle: FocusHandle,
}

impl SearchInput {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self { focus_handle }
    }

    /// Focus this input on the given window.
    pub fn focus(&self, window: &mut Window, _cx: &mut Context<Self>) {
        self.focus_handle.focus(window);
    }

    /// Get the focus handle for this input.
    pub fn focus_handle_raw(&self) -> &FocusHandle {
        &self.focus_handle
    }
}
