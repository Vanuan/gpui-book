# Chapter 1: Open a Window

Let's skip the theory. Before we explain anything, we're going to put a native window on your screen.

## Install Rust and the project generator

If you don't already have Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This installs `rustc` (the compiler) and `cargo` (the package manager — think npm, but for Rust). One command, no separate installs, no PATH archaeology.

Now install the project generator and create your first app:

```bash
# Install a tool to create starter projects
cargo install cargo-gpui --locked

# Create new starter project in the current directory (choose backend/fork when prompted)
cargo gpui new gpui_hello

cd gpui_hello
cargo run
```

`cargo gpui new` is your `npm init`. It puts all the GPUI libraries into `Cargo.toml` manifest, and writes a `main.rs` suitable for whichever fork you picked.

Everything from here on assumes the `gpui-ce` fork; if your
generated `main.rs` looks different, that's expected — it means the
generator already adapted this chapter's code to your fork, and you should
follow your generated file.

> `cargo install cargo-gpui --locked` install the binary cargo-gpui, so that it gives you `cargo gpui` subcommand. `cargo gpui new` will generate `Cargo.toml` and other template files. `cargo run` will install the dependencies in `Cargo.toml`, execute `cargo build` and run the resulting command. The result is the selected gpui fork with a recommended version installed and a starter app running. (Curious what to expect from Cargo's dependency model as your project grows? See Appendix B: The Cargo Expectation Gap.)


## Run it

```bash
cargo run
```

The first build takes a minute — Rust is compiling GPUI and its dependencies from source, something that happens once and then caches. After that, a native window appears on your screen with "Hello, GPUI!" centered on a dark background. No Electron wrapper. No local web server. No Chromium. Native pixels, drawn directly by your GPU.

Here's what cargo `gpui new` just wrote for you:

```rust
{{#include ../examples/01_hello_window/src/main.rs}}
```

## What just happened

Open the `main.rs` in the editor.

If you're coming from the web world or done some desktop application development, some lines will feel familiar.

`application().run()` is your `ReactDOM.render()`. It initializes the application, hooks into your operating system's event loop, and hands control to the closure — the `|cx: &mut App| { ... }` block — where you set up your windows before the loop starts.

> Depending on the fork you selected and the version you chose, there might be some differences in API. But here's the major difference: `gpui_platform` contains `application()` factory that creates the app context. Earlier versions or other forks use `Application::new() or App::new()`.

`cx.open_window()` asks the operating system (compositor) for a window frame/surface. The `cx` here is a context object — you'll see it everywhere in GPUI. It's your handle into the framework's runtime, the thing you talk to when you want GPUI to do something for you.

`HelloWorld` is your root component. Right now it's an empty struct — just a named type with no data — but it's about to become the home for your application state.

`impl Render for HelloWorld` is your component's render method. Whenever the screen needs to update, GPUI calls this function and asks: what should I draw right now? You return a description of the UI — not pixels directly, but a high-level declaration — and GPUI handles the rest.

`div()` is where it starts feeling like Tailwind. GPUI's layout engine is built on Flexbox, the same model powering every modern browser. The method chain reads almost like a CSS class list: make it a flex container, fill the full size, center horizontally, center vertically, set the background color, set the text color, add a child. If you've written `.flex .h-full .w-full .justify-center .items-center` before, you already know some of GPUI.

## Making It Look Like an App

Static text centered on a background isn't very interesting. Let's make it a bit more complex. Open `src/main.rs` and replace the `Render` implementation:

```rust
{{#include ../examples/02_app_shell/src/main.rs:render_impl}}
```

Run it again. Try resizing the window. Notice how instantly the layout recalculates — no jank, no reflow flicker. You have a responsive flex layout with a header bar and a content area, zero CSS, zero HTML.

> **Linux readers**, a heads-up: "try resizing the window" assumes you can
find an edge to grab. On GNOME — the default desktop for Ubuntu, Fedora,
and others — running under Wayland, you might not have one. GPUI's
default window decorations aren't always honored by GNOME's compositor,
which can leave you with a frameless rectangle: no title bar, no resize
borders, no close button. Nothing's wrong with the code above — see
[Appendix C: The Linux Desktop Reality](appendix_c.md)
for why this happens and how to handle it before you ship.

The nesting model is direct: `.child()` takes anything that can be rendered and places it inside the current element. Layouts compose by nesting, exactly like JSX — except it's all just Rust function calls returning descriptions that GPUI resolves into pixels.

## Try it yourself

Before moving on, try changing a few things and re-running:

- Change `rgb(0x3b82f6)` to a different hex color and watch the header
repaint.
- Add a second `.child(...)` to the content area with different text.
- Delete the header `div()` entirely. What happens to the layout?

Don't be afraid to break something. The point is to get a feel for how changes
to the Render method map onto what gets drawn, before Chapter 2 introduces
state and the ownership model that makes this interactive.

## What's missing

The app looks real but it's completely static. Click anywhere — nothing happens. There's no state, no interactivity, no memory of anything the user has done.

To fix that we need to give `HelloWorld` some data, and we need to learn how GPUI thinks about ownership. That's Chapter 2 — and it's where Rust starts to feel less like a foreign language and more like a better version of something you already know.

