## Chapter 13: Platform Features and Cross-Platform Concerns

Up until now, we have lived entirely inside the GPUI window. We have drawn pixels, managed state, and handled keyboard focus. But a real desktop application needs to copy text, save files, open external browsers, and communicate with the host operating system.

GPUI abstracts macOS, Windows, and Linux behind a unified `Platform` trait. However, crossing the boundary from your UI into the host operating system introduces async complexity, testing headaches, and strict component library boundaries.

### File Dialogs and System Integration

If you want a user to select a file, you should not build a custom file tree in GPUI. You should use the native OS file picker. GPUI provides direct access to these native dialogs via `prompt_for_paths()` and `prompt_for_new_path()`.

Because waiting for a user to navigate their hard drive could take minutes, these platform APIs return futures. You spawn a task with `cx.spawn()` and await the user's selection without blocking the UI thread:

```rust
cx.spawn(|this, mut cx| async move {
    let paths = this.update(&mut cx, |this, cx| {
        cx.prompt_for_paths(PromptOptions::default())
    }).await?;
    // Handle the selected paths
}).detach();
```

Alongside file dialogs, the platform services provide cross-platform hooks to hand operations off to the OS:

- `reveal_path()`: Opens the native file explorer (Finder on macOS, Explorer on Windows) and highlights the file.
- `open_with_system()`: Opens a file using the user's default registered application.
- `open_url()`: Launches the system's default web browser.

### Titlebar and Window Decoration

GPUI gives you control over the window titlebar, but the level of control depends on your platform and fork choice.

On macOS, you can enable native titlebar styling with `window.set_titlebar_appears_transparent()` or hide it entirely with `window.set_titlebar_hidden()`. When hidden, you become responsible for draggable regions — use `.on_drag()` on your custom titlebar element to allow window movement.

On Windows and Linux, titlebar behavior is more constrained. Upstream GPUI provides basic titlebar configuration, but deep customization often requires platform-specific code or a fork. WGPUI, for example, offers more consistent titlebar handling across all three platforms due to its unified winit backend.

The practical rule: for custom titlebars on macOS, use GPUI's native APIs. For cross-platform custom titlebars, test each target or consider a fork like WGPUI that abstracts the differences.

### The Clipboard and The Fork Boundary

To support copying and pasting, GPUI provides clipboard read/write capabilities. The exact API shape is subject to change, but the pattern is consistent: you access clipboard services through the context object `cx` and interact with them asynchronously.

Integrating the clipboard highlights a critical architectural boundary if you are using third-party libraries. Recall from Chapter 10 that `gpui-component` provides an excellent, pre-built `InputState` for text editing. But `gpui-component` is strictly a UI library. It provides the text buffer manipulation, cursor positioning, and event handling, but **clipboard APIs remain strictly at the raw GPUI framework level**.

If you want to implement a "Copy to Clipboard" button inside a custom `gpui-component` editor, you must bridge the gap yourself. Here's the pattern, assuming the current API shape:

```rust
// Bridge gpui-component's text state to GPUI's platform clipboard
let text = self.editor_state.value();  // Extract from gpui-component's InputState
cx.write_to_clipboard(ClipboardItem::new_text(text));
```

The same pattern applies in reverse for paste operations: read from the clipboard and insert the text into `gpui-component`'s `InputState`.

*(Note: GPUI's clipboard API is actively evolving. Consult the latest `gpui` documentation for the exact method names.)*

### System Tray and Native Menus (The Missing Pieces)

Upstream GPUI does not provide system tray access. If your application relies on background tray icons, you have two options. First, use external Rust crates that implement platform-specific tray functionality. Second, use a fork like Kael, which ships system tray support as a first-class feature built directly into the framework.

GPUI also does not natively invoke the host operating system's context menus. Applications like Zed build their own cross-platform menus using the `deferred()` rendering hatch and explicit screen coordinate math to prevent edge overflow. You are responsible for drawing these menus using GPUI elements rather than relying on native OS menu widgets.

### Headless Testing

When you touch the file system or native OS dialogs, your code becomes vulnerable to platform-specific divergence. Paths behave differently on Windows than on macOS, and file locking semantics vary wildly.

To test platform-dependent code without opening a window, GPUI provides the `#[gpui::test]` macro and `TestAppContext`. Here's a minimal example:

```rust
#[gpui::test]
async fn test_clipboard_interaction(cx: &mut TestAppContext) {
    let text = "Hello, world!".to_string();
    cx.write_to_clipboard(ClipboardItem::new_text(text.clone()));
    let read_back = cx.read_from_clipboard().await.unwrap();
    assert_eq!(read_back.text(), text);
}
```

This test runs completely headless — no window server, no GPU, no user interaction. It's fast enough to run in CI on every commit.

For file system operations, apply the "Pure Core" pattern from Chapter 12. Inject a `MockFileSystem` during testing rather than calling GPUI's platform APIs directly. Decouple command execution from GPUI's platform layer, and you can simulate file selection and dialog responses without ever touching the real file system.

### The Bridge Pattern Summary

| Platform Feature | GPUI Provides | Alternative |
|-----------------|---------------|-------------|
| File dialogs | ✓ `prompt_for_paths()` | — |
| Open with system | ✓ `open_with_system()` | — |
| Titlebar (macOS) | ✓ `set_titlebar_hidden()`, `.on_drag()` | — |
| Titlebar (cross-platform) | Basic | WGPUI fork |
| Clipboard | ✓ Platform services | — |
| System tray | — | External crates or Kael fork |
| Native context menus | — | Build with GPUI elements + `deferred()` |
| Headless testing | ✓ `#[gpui::test]` + `TestAppContext` | Mock filesystem |

The pattern is consistent: GPUI gives you the low-level platform hooks. For features it doesn't provide, you either bring an external crate, switch to a fork, or build the UI yourself in GPUI. The bridge code between these layers is minimal but critical — and now you have the pattern.

---

In the final chapter, we'll put it all together: distribution, binary size, and the final trade-offs of shipping a GPUI application.
