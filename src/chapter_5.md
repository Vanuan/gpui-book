## Chapter 5: Custom Elements

The fluent API from Chapter 4 will take you a long way. Most application interfaces — navigation bars, settings panels, content lists — can be built entirely from composed `div()` elements with Flexbox and conditional styling.

Some things genuinely can't. A data visualization, a custom code minimap, a waveform display, a specialized input control with non-rectangular hit areas — these require drawing primitives the framework doesn't provide. In GPUI this isn't a dead end. Because the renderer is native, you can step below the fluent API and push drawing commands directly to the GPU.

---

**`Render` vs `Element`**

Two traits govern how GPUI turns your code into pixels.

`Render` is the high-level view contract you've been using since Chapter 1. It returns a blueprint composed of existing pieces — anything that implements `IntoElement`. GPUI resolves that blueprint into a tree and handles the rest.

`Element` is the low-level primitive. When you implement `Element`, you stop composing existing blocks and start defining three things directly: how much space your element needs, where it receives mouse input, and exactly what gets drawn to the screen.

Every built-in primitive — every `div()`, every `.bg()`, every `.rounded_md()` — is implemented using this same `Element` trait underneath. You're not accessing a back door. You're using the same interface the framework uses for itself.

---

**The Three-Phase Lifecycle**

GPUI evaluates every element in three phases: Layout, Prepaint, and Paint.

The best way to understand them is to build something real. Here's a complete custom `ProgressBar` — a fixed-height fill that renders a background track and an active fill proportional to a progress value:

```rust
use gpui::*;

pub struct ProgressBar {
    progress: f32, // 0.0 to 1.0
}

pub fn progress_bar(progress: f32) -> ProgressBar {
    ProgressBar { progress }
}

impl IntoElement for ProgressBar {
    type Element = Self;
    fn into_element(self) -> Self::Element { self }
}

impl Element for ProgressBar {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.0).into();
        style.size.height = px(8.0).into();
        let layout_id = cx.request_layout(style, None);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        ()
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        cx.paint_quad(fill(bounds, rgb(0x374151)));

        let mut fill_bounds = bounds;
        fill_bounds.size.width *= self.progress;
        cx.paint_quad(fill(fill_bounds, rgb(0x3b82f6)));
    }
}
```

You can now drop `progress_bar(0.75)` into any `Render` method exactly like a `div()`. GPUI treats it identically to its own built-in primitives.

---

**What Each Phase Does**

**Layout** — `request_layout` runs first. You define how much space your element needs by constructing a `Style` and returning a `LayoutId`. The flexbox engine uses this to position everything around you. Our progress bar requests full width and a fixed 8px height.

**Prepaint** — Once your position and size are resolved, `prepaint` runs. This is where you register hitboxes — the regions that receive mouse events. Our progress bar is display-only, so prepaint does nothing. If this were a custom slider, you would push an opaque hitbox here matching the element's bounds, so GPUI's event router knows to deliver mouse clicks and drags to this element. Without a registered hitbox, the router treats the element as invisible to the mouse regardless of what gets painted.

This is the mechanic behind the Hitbox Rule from Chapter 2. A `div()` without `.id()` skips hitbox registration entirely. Now you can see exactly why clicks fall through.

**Paint** — Finally, you push drawing commands to the GPU. `cx.paint_quad()` draws a filled rectangle. We draw the track first, then calculate the fill width by multiplying the full bounds width by the progress value, and draw the fill on top. Paint order is depth order — later calls paint over earlier ones.

---

**A Note on Animations**

GPUI has no `.transition()` or `.animate()` modifier. If you want a progress bar to fill smoothly rather than jump, you drive the animation yourself — incrementing the progress value on a timer and calling `cx.notify()` each frame. Chapter 7 covers exactly this using the async `Timer` system.

---

**What's Next**

You can now draw anything. The next question is how users interact with what you've built — specifically, how GPUI thinks about the difference between a mouse click and a keyboard shortcut, and why that distinction matters more than it might seem.

Chapter 6: Focus and Actions.

