## Chapter 7: Async and Memory Lifetimes

Routing a `SaveFile` action through the focus tree is clean and fast. But saving a file — actually writing bytes to disk, or fetching data over a network — can take time. Do that work directly inside an `.on_action()` handler and the UI thread stalls. The window freezes, the OS considers the app unresponsive, and the user reaches for force quit.

GPUI's async engine is how you keep the interface running while real work happens in the background.

---

**Foreground vs Background Tasks**

GPUI provides two spawning contexts:

`cx.spawn()` runs on the main UI thread. It's right for lightweight async control flow — waiting on a timer, chaining short UI updates, sequencing operations that touch view state. It won't block rendering between yield points, but heavy synchronous work inside it will still stall the thread.

`cx.background_spawn()` moves execution onto GPUI's background thread pool, completely off the UI thread. Disk reads, network requests, large file parsing — anything that takes real time belongs here.

Because background tasks run on a separate thread, they have no direct access to your UI state. To get results back safely, you need the `WeakEntity` pattern.

---

**The `WeakEntity` Pattern in Practice**

Chapter 3 introduced `WeakEntity` as theory. Here it becomes a daily pattern.

Before leaving the UI thread, downgrade your entity to a weak reference. When the background work finishes, use that reference to jump back and update state:

```rust
let weak_state = entity.downgrade();

cx.background_spawn(async move {
    // Runs on the background thread pool.
    // The UI keeps rendering while this executes.
    let content = std::fs::read_to_string("large_file.txt")
        .unwrap_or_default();

    // Jump back to the UI thread to apply the result.
    let result = weak_state.update(&mut cx, |this, cx| {
        this.content = content;
        cx.notify();
    });

    if result.is_err() {
        // The window closed while we were loading. Nothing to do.
    }
}).detach();
```

`update()` returns a `Result`. If the user closes the window while the file is loading, the entity is dropped, and `update()` returns `Err` instead of panicking. Your background task exits cleanly. No crash, no undefined behavior, no complex cancellation logic.

---

**RAII: Lifetimes You Control**

GPUI uses Rust's RAII model to manage two things that are easy to get wrong silently.

**Tasks**

`cx.spawn()` and `cx.background_spawn()` both return a `Task<T>`. If you drop that value without calling `.detach()`, the task is immediately cancelled — cooperatively, on its next yield point. Not eventually. Immediately.

This is deliberate. It means you can cancel an in-flight operation simply by dropping its task handle. More usefully, it means storing a task in your struct and reassigning it automatically cancels the previous one:

```rust
struct SearchView {
    search_task: Option<Task<()>>,
}
```

Every new keystroke in a search input can replace `search_task` with a new task, cancelling the previous network request automatically. No explicit cancellation API, no tracking in-flight operations manually. The lifetime of the task is the lifetime of the handle.

If you want a task to keep running — a background sync, an animation loop — you must store it. A task you don't store will silently never complete.

**Subscriptions**

Event subscriptions from `cx.subscribe()` follow the same pattern. The subscription handle keeps the listener alive. Drop the handle and the listener is immediately unregistered. If your event handlers stop firing unexpectedly, the first thing to check is whether the subscription handle is still in scope.

```rust
struct MyView {
    _subscription: Option<Subscription>,
}
```

Both patterns share the same mental model: in GPUI, if you own the handle, you own the lifetime.

---

**Manual Animations**

GPUI has no `.transition()` or `.animate()` modifier. Animations are driven explicitly — update state on a timer, call `cx.notify()`, let the renderer do its job.

Here's a 60-frame animation that fills a progress bar over one second:

```rust
let weak_state = entity.downgrade();

// Store this in your struct — dropping it cancels the animation
self.animation_task = Some(cx.spawn(|mut cx| async move {
    for frame in 0..60 {
        Timer::after(Duration::from_millis(16)).await;

        let _ = weak_state.update(&mut cx, |this, cx| {
            this.progress = frame as f32 / 60.0;
            cx.notify();
        });
    }
}));
```

The same `WeakEntity` pattern applies. If the view is removed mid-animation, `update()` returns `Err` and the loop exits. No dangling references, no orphaned timers.

---

**What's Next**

You now have the full foundational toolkit: layouts, custom drawing, keyboard routing, and async work without memory hazards.

Part II is complete. Part III steps back from the framework itself and looks at the broader ecosystem — what the community has built on top of GPUI, what you can borrow, and what you'll likely need to build yourself.

We start with the application that stress-tests everything: Zed.
