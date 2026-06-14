use gpui::*;
use gpui_platform;

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .child("Hello, GPUI!")
    }
}

fn main() {
    gpui_platform::application().run(|cx| {
        let _ = cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| HelloWorld)
        });
    });
}
