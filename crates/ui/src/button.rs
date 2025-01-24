use gpui::{div, prelude::*, rgb, SharedString, WindowContext};

#[derive(IntoElement)]
pub struct Button {
    text: String,
    on_click: Box<dyn Fn(&mut WindowContext)>,
}

impl Button {
    pub fn new(text: &str, on_click: impl Fn(&mut WindowContext) + 'static) -> Self {
        Self {
            text: text.to_string(),
            on_click: Box::new(on_click),
        }
    }
}

impl RenderOnce for Button {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let on_click = self.on_click;
        div()
            .id(SharedString::from(self.text.clone()))
            .flex_none()
            .px_2()
            .bg(rgb(0xf7f7f7))
            .active(|this| this.opacity(0.85))
            .border_1()
            .border_color(rgb(0xe0e0e0))
            .rounded_md()
            .cursor_pointer()
            .child(self.text.clone())
            .on_click(move |_, cx| on_click(cx))
    }
}