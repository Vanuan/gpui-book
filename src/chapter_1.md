# Chapter 1: Open a Window

We're going to put a native window on your screen.

## Install Rust and the project generator

If you don't already have Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This installs `rustc` (the compiler) and `cargo` (the package manager). `rustup` is one command that installs the whole Rust toolchain.

Now install the project generator and create your first app:

```bash
# Install a tool to create starter projects
cargo install cargo-gpui --locked

# Create new starter project in the current directory (choose backend/fork when prompted)
cargo gpui new gpui_hello

# When prompted:
#   - Choose "gpui-unofficial" (or your preferred backend)
#   - Choose "hello-world" starter template


cd gpui_hello
cargo run
```

`cargo gpui new` works much like `npm init` for the Rust ecosystem: it adds the GPUI libraries to your `Cargo.toml` manifest and writes a `main.rs` suitable for whichever fork you picked.

> 📝 **Note**: When you run cargo gpui new, the generator will ask you two questions:

> - **Which GPUI backend?** → Select gpui-unofficial (or your preferred fork)
> - **Which example/starter?** → Select hello-world

> The rest of this chapter assumes you've made these choices. If you picked something different, your generated `main.rs` might look different. Follow what was generated rather than the code examples here.

> `cargo install cargo-gpui --locked` installs the `cargo-gpui` binary, which provides the `cargo gpui` subcommand. `cargo gpui new` generates `Cargo.toml` and other template files. `cargo run` resolves and builds dependencies, compiles the project, and runs the resulting binary. The result is the selected GPUI fork with a recommended version installed and a starter app running. (For more on Cargo's dependency model, see Appendix B: The Cargo Expectation Gap.)


## Run it

```bash
cargo run
```

The first build takes a minute — Rust compiles GPUI and its dependencies from source, then caches the result. After that, a native window appears on your screen with "Hello, GPUI!" centered on a dark background. There's no Electron wrapper, local web server, or Chromium — it renders native pixels directly.

Here's what `cargo gpui new` just wrote for you:

```rust
{{#include ../examples/01_hello_window/src/main.rs}}
```

## What just happened

Open `main.rs` in your editor.

`application().run()` plays a role similar to `ReactDOM.render()` in React or `app.exec()` in Qt: it initializes the application, hooks into your operating system's event loop, and hands control to the closure (the `|cx: &mut App| { ... }` block) where you set up your windows before the loop starts.

> Depending on the fork you selected and the version you chose, there might be some differences in API. The major difference: `gpui_platform` contains the `application()` factory that creates the app context. Earlier versions or other forks use `Application::new()` or `App::new()`.

`cx.open_window()` asks the operating system (compositor) for a window frame or surface. The `cx` here is a context object that provides access to the framework's runtime, the primary interface for asking GPUI to perform actions.

`HelloWorld` is your root component. Right now it's an empty struct but it will soon hold your application state.

`impl Render for HelloWorld` is your component's render method. Whenever the screen needs to update, GPUI calls this function and asks: what should I draw right now? You return a description of the UI (not pixels directly, but a high-level declaration) and GPUI handles the rest.

`div()` is like `<div/>`. GPUI's layout engine is built on Flexbox, one of the core layout models used in modern browsers. The method chain reads much like a CSS class list: make it a flex container, fill the full size, center horizontally, center vertically, set the background color, set the text color, add a child. GPUI's fluent API follows Tailwind's naming conventions. The method chain: `.flex()`, `.w_full()`, `.justify_center()`, `.items_center()` corresponds directly to `.flex`, `.w-full`, `.justify-center`, `.items-center` in this CSS framework.

## Making It Look Like an App

Static text centered on a background isn't very interesting. Let's make it a bit more complex. Open `src/main.rs` and replace the `Render` implementation:

```rust
{{#include ../examples/02_app_shell/src/main.rs:render_impl}}
```

Run it again. Try resizing the window, so that the layout recalculates. You have a responsive flex layout with a header bar and a content area. The layout is defined entirely in Rust.

> **Linux readers**, a heads-up: "try resizing the window" assumes you can find an edge to grab. On GNOME — the default desktop for Ubuntu, Fedora, and others running under Wayland, you might not have one. GPUI's default window decorations aren't always honored by GNOME's compositor, which can leave you with a frameless rectangle. See [Appendix C: The Linux Desktop Reality](appendix_c.md) for why this happens and how to handle it before you ship.

The nesting model is direct: `.child()` takes anything that can be rendered and places it inside the current element. Layouts compose by nesting, similar to how JSX nests components, except it's all Rust function calls returning descriptions that GPUI resolves into pixels.

## Try it yourself

Before moving on, try changing a few things and re-running:

- Change `rgb(0x3b82f6)` to a different hex color and watch the header repaint.
- Add a second `.child(...)` to the content area with different text.
- Delete the header `div()` entirely. What happens to the layout?

Experiment freely, the compiler will catch anything broken. These exercises show how changes to the Render method map directly to what gets displayed.

## What's missing

The app looks real but it's completely static. Click anywhere — nothing happens. There's no state, no interactivity, no memory of anything the user has done.

To fix that we need to give `HelloWorld` some data, and we need to learn how GPUI thinks about ownership. That's Chapter 2 — where we introduce state and GPUI's ownership model.
