mod components;

use crate::components::button;
use gpui::{div, prelude::*, rgb, SharedString, ViewContext};

pub struct MainView {
    pub text: SharedString,
}

impl Render for MainView {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0xffffff))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0x000000))
            .child(format!("{}", &self.text))
            .child(button("Click Me", |_cx| {
                println!("Button clicked");
            }))
            .children(vec![format!("{}", "children1"), format!("{}", "children2")])
    }
}
