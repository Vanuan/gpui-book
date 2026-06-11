# Chapter 4: Layout and Style 

In the first part of this book, you built a working, stateful application. You learned how to pass data to GPUI and let the runtime borrow checker keep it safe. But so far, our UI has been brutally simple.

Now it is time to make it look like a professional desktop application. 

If you are coming from web development, you might be looking around for the CSS files or a stylesheet engine. You won't find one. GPUI completely eliminates the split between markup and styling. Instead, styling is applied directly via a fluent Rust API that maps perfectly to the mental model of Tailwind CSS.

## The `Styled` Trait and the Fluent API
In GPUI, every base element (like `div()`, `button()`, and `text()`) implements a trait called `Styled`. Because these elements implement this trait, procedural macros inside the framework generate hundreds of chainable styling methods for them. 

When you write `.w_full()`, `.p_4()`, or `.bg()`, you are calling these generated methods directly on the element.

```rust
div()
    .flex()
    .w_full()    // width: 100%
    .h_12()      // height: 3rem (48px)
    .p_4()       // padding: 1rem (16px)
    .bg(rgb(0x1e1e1e))
```

Because this is pure Rust, there are no string typos or missing CSS classes. If you type `.w_fll()` instead of `.w_full()`, your application simply won't compile.

#### Flexbox and Spacing
GPUI's layout engine uses Flexbox. There is no CSS Grid, no floats, and no table layouts. Flexbox is the single layout primitive you must master.

To turn any `div()` into a flex container, you simply call `.flex()`. From there, you control the layout exactly as you would in the web world:

```rust
div()
    .flex()
    .flex_col()           // Flex direction: column
    .justify_between()    // Space elements apart
    .items_center()       // Center elements along the cross-axis
    .gap_4()              // Apply a 16px gap between children
```

For exact pixel measurements, GPUI provides the `px()` helper. You can use this for specific gaps (`.gap(px(10.0))`) or custom sizing when the predefined Tailwind-like steps don't fit your needs.

#### Colors, Typography, and Borders
You apply colors using the `rgb()` or `rgba()` macros. These can be chained for backgrounds, text, and borders:

```rust
div()
    .bg(rgb(0x2d3748))
    .text_color(rgb(0xffffff))
    .border_1()           // 1px border
    .border_color(rgb(0x4a5568))
    .rounded_md()         // Medium border radius
    .shadow_sm()          // Small drop shadow
```
*Note on Text:* While GPUI supports rich text formatting, the basic `.text_color()` and size APIs applied to a `div()` will cascade to the plain text children inside it. However, GPUI lacks some of the hyper-granular text styling found in CSS, such as arbitrary `.line_height()`.

#### Conditional Styling with `when()`
One of the biggest friction points in standard React/Tailwind development is dynamically changing classes based on state (e.g., `className={isActive ? "bg-blue-500" : "bg-gray-500"}`).

GPUI solves this elegantly with the `.when()` method. It allows you to conditionally apply styles without breaking your fluent builder chain:

```rust
div()
    .flex()
    .p_2()
    .rounded_md()
    // If self.is_active is true, apply the blue background.
    // Otherwise, do nothing.
    .when(self.is_active, |this| this.bg(rgb(0x3b82f6)))
    // You can also use it for conditional text colors or borders
    .when(!self.is_active, |this| this.text_color(rgb(0xa0aec0)))
    .child("Dashboard")
```

#### Absolute Positioning and Overlays

Flexbox handles most of your application's structure, but some things need to step outside the normal flow entirely. Notification badges, tooltips, dropdown menus — these sit on top of other content rather than alongside it.

GPUI provides `.absolute()`, paired with `.top()`, `.bottom()`, `.left()`, and `.right()`. An absolute element is positioned relative to its closest ancestor marked `.relative()`, exactly as in CSS:

```rust
div()
    .relative()          // Establishes the positioning boundary
    .p_2()
    .bg(rgb(0x3b82f6))
    .rounded_md()
    .child("Inbox")
    .child(
        div()
            .absolute()          // Pulls the badge out of the flex flow
            .top(px(-4.0))
            .right(px(-4.0))
            .w_3()
            .h_3()
            .bg(rgb(0xef4444))
            .rounded_full()
    )
```

#### Paint Order Instead of Z-Index

When you start using `.absolute()`, you'll instinctively reach for `.z_index()` to control which element sits on top. There isn't one.

GPUI renders directly to the GPU and deliberately avoids the performance cost of calculating CSS stacking contexts. Instead, elements are painted in exactly the order they appear in your code. If you need a badge to overlap the content below it, declare it after that content. The last child paints on top.

For overlays that need to break out of their parent's clipping bounds entirely — a command palette, a context menu — GPUI provides a `deferred()` rendering system. That's Chapter 8.

#### What GPUI Does NOT Have (The Constraints)
Because GPUI renders everything natively to the GPU for maximum performance, it does not implement the entire CSS specification. To avoid frustration, you must understand what is deliberately missing:

1. **No Z-Index:** There is no `.z_index()` method. Elements are drawn in the exact order they appear in the tree. To put a popup over a text box, you either render it later in the child list or use absolute positioning with GPUI's `.deferred()` rendering system (which we will cover in Chapter 8).
2. **No Animations or Transforms:** GPUI has no `.transition()`, `.animation()`, `.rotate()`, or `.scale()` methods. If you want something to move or fade, you must manually drive the animation state using a timer, which we will cover in Chapter 7. 
3. **Limited Overflow:** There is no `.overflow_scroll()`, `.overflow_auto()`, or `.overflow_visible()`. GPUI only supports `.overflow_hidden()` to clip children to the parent's bounds.
4. **No Gradients or Filters:** There are no built-in linear gradients, radial gradients, or backdrop blur filters.

#### The Next Step
By combining `div()`, Flexbox, and `.when()`, you can build 95% of a professional application interface. But what about the other 5%? What happens when you need to draw a custom shape, a complex chart, or something GPUI simply doesn't have a built-in method for?

In Chapter 5, we are going to step off the "fluent styling" path and drop down to the foundational `Element` trait, showing you how to draw your own custom primitives directly to the screen.

