use gpui::*;
use gpui_platform;

struct HelloWorld;

impl Render for HelloWorld {
    // ANCHOR: render_impl
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x1e1e1e))
            .child(
                div()
                    .w_full()
                    .p_4()
                    .bg(rgb(0x3b82f6))
                    .text_color(rgb(0xffffff))
                    .text_xl()
                    .font_weight(FontWeight::BOLD)
                    .child("My Native App")
            )
            .child(
                div()
                    .flex_1()
                    .p_6()
                    .text_color(rgb(0xa0aec0))
                    .child("Rendering on GPU")
            )
    }
    // ANCHOR_END: render_impl
}

fn main() {
    gpui_platform::application().run(|cx| {
        let _ = cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| HelloWorld)
        });
    });
}
