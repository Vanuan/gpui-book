# Chapter 11: Text Foundations: The Input You Have to Build Yourself

If you want to understand why building native UI is hard, look at text.

Here's a simple search bar. You capture a keystroke and append it to a string. In Rust, that string is UTF-8. Characters can be one to four bytes. The emoji "🦀" is four bytes. If your cursor lands anywhere inside those four bytes, Rust will panic.

```rust
let mut query = String::from("Hi 🦀");
// The user moves the cursor back one space and types "!"
query.insert(4, '!');
// PANIC: byte index 4 is not a char boundary; it is inside '🦀' (bytes 3..7)
```

String operations in Rust index by bytes. The emoji "🦀" spans bytes 3 through 7. Index 4 lands inside the emoji, and Rust refuses to split a character. This is the trap. Standard strings index by bytes. User interfaces index by characters (graphemes). Mix them, and your app crashes the moment someone types an emoji or a non-Latin character.

## What GPUI Gives You (And What It Doesn't)

GPUI is a rendering engine and a windowing system. It is not a text editor out of the box. Before you build an input field, understand exactly where the framework stops and your responsibility begins.

| Feature | Who Provides It | Notes |
|---------|----------------|-------|
| Keyboard Events | GPUI | `KeyDownEvent` delivers raw keystrokes |
| Focus Management | GPUI | `FocusHandle` routes events to your element |
| Clipboard Access | GPUI | Platform services read/write the system clipboard |
| Text Storage | **You** | GPUI has no built-in `InputState` or text buffer |
| Cursor Position | **You** | Track where the blinking cursor lives |
| Selection | **You** | Track highlighted ranges as byte offsets |
| Undo/Redo | **You** | Maintain a stack of `Change` operations |
| IME Composition | **You** | Handle multi-event input for Chinese, Japanese, Korean |

Look at that table. GPUI handles the low-level plumbing. You handle everything that makes text behave like text.

## The Rope Solution

Because standard `String` requires O(n) time to insert a character in the middle of a large document, and because byte-offset math is a minefield, the Rust text-editing ecosystem relies on a different data structure: the **rope**.

A rope is a tree of string slices. Inserting or deleting text anywhere in the document takes O(log n) time — exponentially faster than O(n) for large documents. More importantly for UI, the standard crate — `ropey` — operates on character indices.

You don't need to master `ropey`. You need a tiny fraction of its API:

```rust
use ropey::Rope;

let mut text = Rope::from_str("Hello");
text.insert_char(5, '!');    // Safe, O(log n), indexed by character
text.remove(0..1);            // Removes the 'H'
let len = text.len_chars();   // Length in characters, not bytes
```

**Version note:** Check the `ropey` version you're depending on. The 1.x and 2.x APIs differ on whether indices are character-based or byte-based. This book uses 1.x conventions.

## The Minimal Working Input

Let's wire `ropey` and GPUI together. A functioning text input needs three pieces: state, event handling, and rendering.

### 1. The State

Your component needs a `Rope` for the text buffer, a `usize` for the cursor position (tracked as a character offset, not a byte index), and a `FocusHandle` so GPUI routes keyboard events correctly.

```rust
use gpui::*;
use ropey::Rope;

pub struct TextInput {
    buffer: Rope,
    cursor_offset: usize,    // Character index, NOT byte offset
    focus_handle: FocusHandle,
}
```

Real-world inputs like `gpui-component`'s `InputState` add much more: selection ranges, undo history, display maps for line wrapping, scroll handles, IME marked ranges, validation rules, masking, and LSP integration. But this minimal state gets you typing.

### 2. Event Handling

When your element renders, attach the focus handle and listen for keystrokes. Intercept navigation and editing commands — Backspace, Left Arrow, Right Arrow — and pass normal characters directly into the `Rope`.

```rust
impl TextInput {
    fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
        let key = &event.keystroke.key;

        match key.as_str() {
            "backspace" => {
                if self.cursor_offset > 0 {
                    // ropey removes the character before the cursor safely
                    self.buffer.remove((self.cursor_offset - 1)..self.cursor_offset);
                    self.cursor_offset -= 1;
                }
            }
            "left" => {
                self.cursor_offset = self.cursor_offset.saturating_sub(1);
            }
            "right" => {
                self.cursor_offset = (self.cursor_offset + 1).min(self.buffer.len_chars());
            }
            _ => {
                // Single character? Insert it.
                if key.chars().count() == 1 {
                    let c = key.chars().next().unwrap();
                    self.buffer.insert_char(self.cursor_offset, c);
                    self.cursor_offset += 1;
                }
            }
        }
        cx.notify(); // Tell GPUI we changed state
    }
}
```

In production inputs like `gpui-component`'s `InputState`, the event handler is far more sophisticated. It tracks selection state, handles clipboard operations, manages undo stack entries, and coordinates with the IME system.

### 3. Rendering

Finally, draw the component. Slice the `Rope` into a standard string for GPUI's text renderer, and show a cursor.

```rust
impl Render for TextInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let display_text = self.buffer.to_string();

        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(Self::handle_key_down))
            .p_2()
            .bg(rgb(0x1e1e1e))
            .border_1()
            .border_color(if self.focus_handle.is_focused(cx) {
                rgb(0x3b82f6)
            } else {
                rgb(0x4a5568)
            })
            .child(display_text)
    }
}
```

A production input would also compute the exact pixel position of the cursor for precise placement. `gpui-component`'s internal layout caching handles this, but that implementation is specific to the library.

## The Missing Pieces: IME

The code above works for English text. It fails for Chinese, Japanese, and Korean.

Input Method Editors (IMEs) allow users to type characters not on their keyboard. Japanese users type "nihongo" and get a candidate window to select 「日本語」. During composition, keystrokes *do not* produce final characters immediately. If you insert them directly into your buffer, the IME breaks.

GPUI provides IME events, but you must handle them correctly. The key is detecting when a keystroke is part of active IME composition:

```rust
// Check if keydown event is part of IME composition
if event.is_composing() {
    // IME is composing — don't insert into buffer
    return;
}
```

**Technical note:** The code above uses `event.is_composing()` as the correct GPUI native event check. (If you see references to `key_code == 229` in web-focused examples, that's a browser convention, not a native desktop one.)

On Linux, pressing Enter while composing with Chinese IME should not send a message — it should commit the composition. The fix required checking the compose status before dispatching `KeyDown` events.

On macOS, IME handling is even more complex. When a user types a dead key (like ` on a Brazilian layout) or a special character with Cmd held (like Cmd+ç on Spanish layout), the IME consumes the event before GPUI sees it. Recent fixes in GPUI bypass the IME for Cmd+key events while preserving composition state.

A complete input implementation must track an "IME marked range" — temporary text that the IME is composing — and only commit it to the `Rope` when composition ends. IME composition produces multiple events per character; you must buffer the composition string and only commit to the rope when composition ends. `gpui-component`'s `InputState` does exactly this, storing `ime_marked_range: Option<Selection>` alongside the main text buffer.

## The Missing Pieces: Selection and Undo

A usable text input also needs selection and undo.

Selection is a range of byte offsets in the buffer. A collapsed range (start == end) represents a cursor position. You need to handle:
- Shift + arrow keys to extend selection
- Double-click to select words
- Triple-click to select lines
- Drag to select with the mouse

`gpui-component` implements word selection by checking each character's `CharType` (Word, Whitespace, Newline, Other) and expanding the range to boundaries.

Undo requires a stack of `Change` operations. Each text insertion or deletion pushes a `Change` onto the history. Ctrl+Z pops the last change and reverses it. The history stack should have a maximum size to prevent memory growth. `gpui-component`'s `InputState` includes `history: History<Change>` as a core field.

## Which Path to Choose?

If you're building a specialized internal tool or a command palette, the minimal rope-backed input we just built is sufficient. You need the three pieces — state, events, rendering — and you can stop there.

But if text editing is a primary feature of your application, do not build from scratch. As outlined in Appendix A, you have better options:

- **Copy from Zed**: The `ui_input` crate contains `SingleLineInput` and `MultiLineInput`. The code is there, but it's GPL-licensed. Copying it puts your project under GPL. See Appendix A for the full licensing boundary.

- **Use `gpui-component`**: Provides `InputState` with rope manipulation, selection, undo history, IME composition handling, line measurement, validation rules, and LSP integration — all under Apache 2.0.

For most commercial applications, `gpui-component` is the right answer. It implements everything this chapter outlined — and dozens of details we skipped — in production-tested code.

In the next chapter, we'll move from text foundations to application structure: how to organize a real GPUI project so your business logic stays clean and testable.

