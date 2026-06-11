# Chapter 2: Add State

Our app from Chapter 1 looks real but remembers nothing. Click anywhere — nothing happens. To fix that we need to give `HelloWorld` some data and learn how GPUI thinks about ownership.

Let's build a click counter. It's the simplest possible stateful UI, and it introduces every pattern you'll use in every GPUI application you write.

---

**The Data**

Start by adding a field to the struct:

```rust
struct HelloWorld {
    counter: usize,
}
```

Simple enough. But the moment we try to mutate this field inside a running UI, we run into something worth understanding — because how GPUI solves it is the foundation of everything else in this book.

Think of your app's UI as a family tree of screens, panels, buttons, and text. Each piece potentially reacts to the same underlying data. A button label might change from "Start" to "Stop". A click handler might update a counter that three other components are displaying. A panel might appear or disappear based on state that something else just changed.

Rust wants to know exactly who is allowed to change what, and only one part of the program may mutate a piece of data at a time. A UI, on the other hand, naturally wants several parts of the code to look at and react to the same thing. That tension — Rust's strict ownership rules meeting a widget tree's overlapping interests in shared state — is why GUI programming in Rust has a reputation for friction. Not because Rust is wrong. But because the obvious way to write UI code is exactly the kind of thing Rust is designed to prevent.

GPUI sidesteps this entirely by changing who owns the data.

---

**Handing Ownership to the Runtime**

In GPUI, the application runtime owns your state — not you. Instead of passing mutable references around your UI tree, you surrender your struct to the framework when the window opens, using `cx.new()`:

```rust
fn main() {
    App::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_cx| HelloWorld { counter: 0 })
        });
    });
}
```

`cx.new()` takes your data, places it in GPUI's managed runtime, and hands you back an `Entity<HelloWorld>` — a strongly-typed handle to your state. The framework now owns the actual memory. It guarantees your state stays alive exactly as long as the UI needs it and cleans it up automatically when the window closes. No manual memory management, no lifetime annotations, no dangling pointers.

Two things to hold onto here:

- **`T`** — your raw struct, `HelloWorld`. Just plain data.
- **`Entity<T>`** — the handle GPUI gives back. Your reference into the runtime.

When a type implements `Render`, GPUI treats its entity as a view. The struct is both the data and the component — the state and the thing that knows how to draw itself.

---

**Making It Interactive**

Now update `Render` to draw a counter and a button. In GPUI there's no built-in `<Button>` primitive — interactivity is just behavior layered onto the same `div()` you already know:

```rust
use gpui::*;

struct HelloWorld {
    counter: usize,
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .justify_center()
            .items_center()
            .bg(rgb(0x1e1e1e))
            .text_color(rgb(0xffffff))
            .gap_4()
            .child(
                div()
                    .text_xl()
                    .child(format!("Click count: {}", self.counter))
            )
            .child(
                div()
                    .id("increment-button")
                    .p_2()
                    .px_4()
                    .bg(rgb(0x3b82f6))
                    .rounded_md()
                    .cursor_pointer()
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.counter += 1;
                        cx.notify();
                    }))
                    .child("Increment")
            )
    }
}

fn main() {
    App::new().run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| HelloWorld { counter: 0 })
        });
    });
}
```

Run `cargo run`. Click the button. The counter increments instantly — native pixels, no virtual DOM diff, no garbage collector deciding this is a good moment to pause.

Two small things before we move on:

`format!("Click count: {}", self.counter)` is Rust's string interpolation. The `!` marks it as a macro rather than a regular function call — something you'll see often in Rust. It works exactly like a template literal in JavaScript.

`cx.listener()` is worth a moment's attention. When you write an event handler in GPUI, you need access to both your component's data (`this`) and the framework context (`cx`) at the same time. `cx.listener()` is the bridge that wires them together safely — it produces a closure that GPUI knows how to call with both. You'll write it often enough that it becomes automatic, but now you know what it's doing.

---

**Two Things Worth Understanding Now**

**The Hitbox Rule**

Notice `.id("increment-button")` on the button div. In GPUI a plain `div()` is invisible to the mouse by default — GPUI's hit-testing skips it entirely. Assigning a unique `.id()` registers the element as a target for mouse events. Without it, clicks fall silently through to the background with no error, no warning, nothing. Any element that needs to respond to mouse interaction needs an id.

**The Golden Rule**

Inside the click handler, `this.counter += 1` mutates the data in memory. But changing data in memory does not update the screen. You must call `cx.notify()` explicitly.

If you're coming from React, this is your `setState`. It tells GPUI: the data has changed, mark this entity dirty, schedule a re-render for the next frame. This explicit step is what lets GPUI safely coordinate updates across the entire UI tree without watching every variable in your application. Forget it once and your UI will silently show stale values — which will happen exactly once before it becomes muscle memory.

---

**What's Next**

You built a stateful, interactive, native desktop app without writing a single lifetime annotation, without fighting the borrow checker, without thinking about memory at all. GPUI handled it.

But how? How does the runtime safely let a click handler, a render pass, and an async task all interact with your data without the compiler objecting?

That's the question Chapter 3 answers. You've earned it.
