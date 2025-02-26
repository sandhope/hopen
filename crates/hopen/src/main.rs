// Disable command line from opening on release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gpui::{prelude::*, App, AppContext, WindowOptions};
use hopen::MainView;

fn main() {
    App::new().run(|cx: &mut AppContext| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| MainView {
                text: "Hopen!".into(),
            })
        })
        .unwrap();
    });
}
