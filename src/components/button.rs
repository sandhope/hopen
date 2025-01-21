use gpui::{
    div, prelude::*, rgb, SharedString, WindowContext
};

pub fn button(text: &str, on_click: impl Fn(&mut WindowContext) + 'static) -> impl IntoElement {
    div()
        .id(SharedString::from(text.to_string()))
        .flex_none()
        .px_2()
        .bg(rgb(0xf7f7f7))
        .active(|this| this.opacity(0.85))
        .border_1()
        .border_color(rgb(0xe0e0e0))
        .rounded_md()
        .cursor_pointer()
        .child(text.to_string())
        .on_click(move |_, cx| on_click(cx))
}