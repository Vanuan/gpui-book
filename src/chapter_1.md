# Chapter 1: Open a Window

Let's skip the theory. Before we explain anything, we're going to put a native window on your screen.

First, install Rust. If you haven't already:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This installs `rustc` (the compiler) and `cargo` (the package manager — think npm, but for Rust). One command, no separate installs, no PATH archaeology. When it's done, create your first project:

```bash
# A tool to create starter project

cargo install cargo-generate --lock

# cargo new gpui_hello

cd gpui_hello
cargo add gpui
```

> `cargo add gpui` pulls in the GPUI framework and everything it depends on. (Curious what to expect from Cargo's dependency model as your project grows? See Appendix B: The Cargo Expectation Gap.)

`cargo new` is your `npm init`. `cargo add gpui` pulls in the GPUI framework and everything it depends on. Open `src/main.rs`, delete what's there, and paste this:


```rust
use gpui::*;

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .justify_center()
            .items_center()
            .bg(rgb(0x2d3748))
            .text_color(rgb(0xffffff))
            .child("Hello, GPUI!")
    }
}

fn main() {
    App::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| HelloWorld)
        });
    });
}
```

Run it:

```bash
cargo run
```

The first build takes a minute — Rust is compiling GPUI and its dependencies from source, something that happens once and then caches. After that, a native window appears on your screen with "Hello, GPUI!" centered on a dark background. No Electron wrapper. No local web server. No Chromium. Native pixels, drawn directly by your GPU.

---

**What Just Happened**

If you're coming from the web world, those twenty lines will feel surprisingly familiar.

`App::new().run()` is your `ReactDOM.render()`. It initializes the application, hooks into your operating system's native event loop, and hands control to the closure — the `|cx: &mut App| { ... }` block — where you set up your windows before the loop starts.

`cx.open_window()` asks the operating system for a literal window frame. The `cx` here is a *context object* — you'll see it everywhere in GPUI. It's your handle into the framework's runtime, the thing you talk to when you want GPUI to do something on your behalf.

`HelloWorld` is your root component. Right now it's an empty struct — just a named type with no data — but it's about to become the home for your application state.

`impl Render for HelloWorld` is your component's render method. Whenever the screen needs to update, GPUI calls this function and asks: what should I draw right now? You return a description of the UI — not pixels directly, but a blueprint — and GPUI handles the rest.

`div()` is where it starts feeling like Tailwind. GPUI's layout engine is built on Flexbox, the same model powering every modern browser. The method chain reads almost like a class list: make it a flex container, fill the full size, center horizontally, center vertically, set the background color, set the text color, add a child. If you've written `.flex .h-full .w-full .justify-center .items-center` before, your hands already know the shape of this.

---

**Making It Look Like an App**

Static text centered on a background isn't very interesting. Let's make it look like something you'd actually ship. Replace the `Render` implementation:

```rust
impl Render for HelloWorld {
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
                    .child("Rendering at 120fps. No garbage collection pauses.")
            )
    }
}
```

Run it again. Try resizing the window. Notice how instantly the layout recalculates — no jank, no reflow flicker. You have a responsive flex layout with a header bar and a content area, zero CSS, zero HTML.

The nesting model is direct: `.child()` takes anything that can be rendered and places it inside the current element. Layouts compose by nesting, exactly like JSX — except it's all just Rust function calls returning descriptions that GPUI resolves into pixels.

---

**What's Missing**

The app looks real but it's completely static. Click anywhere — nothing happens. There's no state, no interactivity, no memory of anything the user has done.

To fix that we need to give `HelloWorld` some data, and we need to learn how GPUI thinks about ownership. That's Chapter 2 — and it's where Rust starts to feel less like a foreign language and more like a better version of something you already know.

