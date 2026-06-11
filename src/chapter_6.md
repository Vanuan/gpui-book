## Chapter 6: Focus and Actions

You can now draw anything to the screen. The next question is how your application responds to the keyboard — and this is where GPUI introduces a distinction that feels unfamiliar at first but turns out to be one of its best ideas.

In GPUI, mouse events and keyboard events are handled through entirely different systems, and deliberately so.

---

**The Semantic Divide**

Mouse events are positional. When you click, GPUI finds the hitbox registered during prepaint and triggers the corresponding `.on_click()` handler. The location of the cursor determines everything.

Keyboard events are contextual. When a user presses `Cmd+S`, the cursor position is irrelevant. What matters is which part of the application currently has the user's attention. Is the user working in a text editor? Save the file. Are they in a settings panel? Save the preferences. The same keypress, two completely different meanings depending on context.

GPUI models this context through a focus system and a routing layer called Actions. Together they replace the pattern of attaching `onKeyDown` listeners to elements and checking which mode the app is currently in.

---

**Focus Management**

To receive keyboard-driven input, an element must be part of the focus tree. GPUI manages this with `FocusHandle` — a token representing a specific element's position in that tree.

You create focus handles through the context and attach them to elements with `.track_focus()`:

```rust
use gpui::*;

struct SplitPane {
    left_focus: FocusHandle,
    right_focus: FocusHandle,
}

impl SplitPane {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            left_focus: cx.focus_handle(),
            right_focus: cx.focus_handle(),
        }
    }
}

impl Render for SplitPane {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .size_full()
            .child(
                div()
                    .track_focus(&self.left_focus)
                    .on_focus(cx.listener(|_, _, _, cx| println!("Left pane focused")))
                    .w_1_2()
                    .bg(rgb(0x1e1e1e))
            )
            .child(
                div()
                    .track_focus(&self.right_focus)
                    .w_1_2()
                    .bg(rgb(0x2d3748))
            )
    }
}
```

`.track_focus()` registers the element with GPUI's focus tree. To move focus programmatically — after a task completes, after a modal closes — call `self.right_focus.focus(window)`. The rest of the routing system follows automatically.

---

**The Action System**

With focus established, the question is how keypresses get connected to behavior. GPUI's answer is Actions — pure semantic descriptions of intent defined with the `actions!` macro:

```rust
actions!(workspace, [SaveFile, ClosePanel]);
```

These structs carry no logic. `SaveFile` just means "the user intends to save." What saving actually does depends on which element catches the action and what context it's in.

Keystrokes are mapped to actions in a keymap configuration separate from your application code. `Cmd+S` maps to `workspace::SaveFile`. If a user wants to remap it, they change the keymap — nothing in your Rust code changes. The action name is the stable contract between input and behavior.

---

**Action Routing: The Bubble**

When the user presses a mapped keystroke, GPUI creates the corresponding action struct and injects it at whichever element currently holds focus. From there it bubbles up the element tree. Every element in the path has a chance to catch it with `.on_action()`:

```rust
div()
    .track_focus(&self.left_focus)
    .on_action(cx.listener(|this, _action: &SaveFile, _window, cx| {
        // Only reached if this pane or one of its children has focus
        this.save(cx);
        cx.notify();
    }))
```

If no element in the active focus path handles the action, it drops silently at the root.

This silent drop is a feature. Consider a `Delete` action mapped to `Backspace`. When the user is typing in a text input, the input catches `Delete`, removes a character, and stops the bubble. When the user clicks into a file explorer and presses `Backspace`, the text input is no longer in the focus path — it never sees the action. The file explorer catches it instead and moves a file to trash.

No global mode tracking. No `if currentFocus === 'editor'` conditionals. The structure of the focus tree is the routing logic.

---

**What's Next**

You can draw anything and route user intent precisely where it belongs. The remaining question is time — what happens when handling an action triggers work that takes a while? Saving to a slow disk, fetching from a network, parsing a large file. Doing that work inside an action handler will freeze the UI.

Chapter 7 covers GPUI's async engine: how to move heavy work off the UI thread, how to get results back safely, and how the `WeakEntity` pattern from Chapter 3 becomes essential in practice.
