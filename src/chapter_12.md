## Chapter 12: Structuring a Real Application

You've built components, wrestled with text input, and navigated the ecosystem. Now the question is mechanical: how do you organize code so your business logic doesn't tangle itself in GPUI's event loop, and how do you test that logic without opening a window?

This chapter answers that with a specific structure — the concrete workspace layout and crate boundaries that make a GPUI application maintainable as it grows.

---

**The Four-Layer Architecture**

A robust GPUI application separates concerns through four distinct layers:

**The Rendering Layer** is the absolute bottom. GPUI manages the windowing system, the event loop, the GPU pipeline, and flexbox layout. You rarely interact with this directly beyond initializing the `App`.

**The Primitives Layer** is the declarative API layer. Elements like `div()`, `text()`, and `svg()` are stateless building blocks. They describe what something looks like but hold no application state.

**The Widgets Layer** is where primitives become reusable, interactive components. Widgets are stateful but strictly product-blind. A button knows how to animate when clicked. It does not know what that click achieves.

**The Product Layer** is the orchestrator. This layer catches widget interactions and binds them to your business logic.

The rule that makes this model work: **your domain logic should never import GPUI.**

---

**The Pure Core Pattern**

Put your business logic in a separate crate with no dependencies on `gpui`, `winit`, or any UI framework. That crate should compile to native Rust and nothing else. No `div`, no `cx.spawn()`, no focus handles.

The structure is straightforward:

```text
myapp/
├── Cargo.toml           # Workspace root
├── crates/
│   ├── domain/          # Pure business logic
│   │   ├── Cargo.toml   # No GPUI dependency
│   │   └── src/
│   │       └── lib.rs
│   ├── ui/              # GPUI application
│   │   ├── Cargo.toml   # Depends on domain + gpui
│   │   └── src/
│   │       └── main.rs
│   └── cli/             # Headless CLI harness
│       ├── Cargo.toml   # Depends on domain only
│       └── src/
│           └── main.rs
```

---

**Cargo Workspaces**

A Cargo workspace manages multiple related crates in a single repository:

```toml
# Root Cargo.toml
[workspace]
members = [
    "crates/domain",
    "crates/ui",
    "crates/cli",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2024"

[workspace.dependencies]
gpui = { git = "https://github.com/zed-industries/zed", package = "gpui" }
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
```

The domain crate's `Cargo.toml` stays deliberately lean:

```toml
[package]
name = "myapp-domain"
version.workspace = true
edition.workspace = true

[dependencies]
serde.workspace = true
thiserror.workspace = true
# No gpui here — enforced by Cargo
```

Cargo enforces the boundary. You cannot accidentally import GPUI into your domain crate — it simply isn't in the dependency graph.

---

> **Tip:** As your workspace grows and you add community crates beyond `serde` and `thiserror`, you'll encounter Rust's `0.x` versioning conventions — which work differently than you might expect from `npm` or `pip`. Appendix B covers the two rules worth knowing before they bite you.

---

**The Domain: An Example**

Let's build a markdown previewer. The domain crate:

```rust
// crates/domain/src/lib.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkdownDoc {
    pub raw_text: String,
    pub word_count: usize,
}

impl MarkdownDoc {
    pub fn new(text: String) -> Self {
        let word_count = text.split_whitespace().count();
        Self { raw_text: text, word_count }
    }

    pub fn update_text(&mut self, new_text: String) {
        let new_word_count = new_text.split_whitespace().count();
        self.raw_text = new_text;
        self.word_count = new_word_count;
    }

    pub fn export_html(&self) -> String {
        format!("<h1>Preview</h1>\n<p>{}</p>", self.raw_text)
    }
}

pub struct DocumentStore {
    documents: Vec<MarkdownDoc>,
    current_index: usize,
}

impl DocumentStore {
    pub fn new() -> Self {
        Self {
            documents: vec![MarkdownDoc::new(String::new())],
            current_index: 0,
        }
    }

    pub fn current_document(&self) -> &MarkdownDoc {
        &self.documents[self.current_index]
    }

    pub fn update_current_document(&mut self, new_text: String) {
        self.documents[self.current_index].update_text(new_text);
    }
}
```

No `gpui`. No `cx`. Just Rust. You can run `cargo test` on this crate without a window server, a GPU, or an event loop.

---

**The UI Layer: Thin and Deliberate**

In GPUI, state is owned by whoever creates it via `cx.new()`. The resulting `Entity<T>` is your handle. Everything flows from where you store that handle.

The application root creates the `DocumentStore` entity and hands it to the view:

```rust
// crates/ui/src/main.rs
use gpui::*;
use myapp_domain::DocumentStore;

struct EditorApp {
    document: Entity<DocumentStore>,
}

impl EditorApp {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            document: cx.new(|_cx| DocumentStore::new()),
        }
    }
}
```

The view reads from the entity and renders it:

```rust
// crates/ui/src/document_view.rs
use gpui::*;
use myapp_domain::DocumentStore;

pub struct DocumentView {
    document: Entity<DocumentStore>,
    focus_handle: FocusHandle,
}

impl Render for DocumentView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let doc = self.document.read(cx).current_document().clone();

        div()
            .track_focus(&self.focus_handle)
            .flex()
            .flex_col()
            .size_full()
            .child(
                div()
                    .text_sm()
                    .text_color(rgb(0x6b7280))
                    .child(format!("Word count: {}", doc.word_count))
            )
            .child(
                div()
                    .mt_4()
                    .p_4()
                    .bg(rgb(0x1e1e1e))
                    .child(doc.raw_text)
            )
    }
}
```

The component reads the current document from the entity, renders it using primitives, and knows nothing about how the document got its content.

---

**Four Architectural Lessons**

**Lesson one: drop UI metadata at the boundary.**

When a user clicks, GPUI generates events containing screen coordinates and pixel bounds. Your domain logic should never see these. The Product layer catches the raw event, strips the framework-specific metadata, and hands the core exactly what it needs — a clean string or a semantic command.

**Lesson two: ownership boundaries are enforced by where you store the handle.**

`Entity<T>` is GPUI's universal handle. Whoever holds it, owns the lifetime. If you store an `Entity<DocumentStore>` in a component that recreates documents frequently, the old entities won't be dropped until the component itself is dropped — which may never happen. Use `WeakEntity` for references that should not extend lifetimes:

```rust
// Store a weak reference when you observe but don't own
let weak_doc = self.document.downgrade();
```

The question to ask at every boundary: should this component own this state, or just observe it?

**Lesson three: use subscriptions for cross-cutting concerns.**

Rather than manually pushing updates to every component, broadcast changes through GPUI's subscription system. One component updates the entity. Every subscribed view receives a notification and re-renders:

```rust
// In a component that observes but doesn't own the document
cx.subscribe(&document_entity, |this, _entity, _event, cx| {
    cx.notify(); // Re-render when the document changes
}).detach();
```

The domain publishes events. The UI subscribes. Components re-render when they need to, not when you manually tell them to.

**Lesson four: lifecycle discipline prevents silent leaks.**

Subscriptions and tasks follow RAII — drop the handle, cancel the subscription. Store subscriptions in your struct if you want them to live as long as the component. Drop them if you want to stop listening. The same applies to async tasks. If a component is removed from the view tree while a background task is running, `WeakEntity::update()` returns `Err` and exits cleanly. No orphaned operations, no silent memory growth.

---

**The CLI Harness**

The final crate proves the pure core actually works without a UI:

```toml
# crates/cli/Cargo.toml
[package]
name = "myapp-cli"
version.workspace = true
edition.workspace = true

[dependencies]
myapp-domain = { path = "../domain" }
# No gpui
```

```rust
// crates/cli/src/main.rs
use myapp_domain::DocumentStore;

fn main() {
    let mut store = DocumentStore::new();
    store.update_current_document("# Hello\n\nThis is a test.".into());
    let doc = store.current_document();
    println!("Words: {}", doc.word_count);
    println!("HTML: {}", doc.export_html());
}
```

Run with `cargo run --bin myapp-cli`. No window, no GPU, no event loop. The domain logic runs in complete isolation:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_count_updates_on_edit() {
        let mut store = DocumentStore::new();
        store.update_current_document("hello world".into());
        assert_eq!(store.current_document().word_count, 2);
    }

    #[test]
    fn export_html_wraps_content() {
        let doc = MarkdownDoc::new("test content".into());
        assert!(doc.export_html().contains("<p>test content</p>"));
    }
}
```

`cargo test` on the domain crate runs in milliseconds. When GPUI shifts under you — a new version, a fork migration — your domain logic doesn't feel a thing. The separation isn't just good practice. It's what makes your application portable.

---

**What's Next**

The application is structured. The domain is clean. Now it needs to interact with the platform it's running on — file dialogs, clipboard, system tray, and the places where macOS, Windows, and Linux behave differently than you'd expect.

Chapter 13: Platform Features and Cross-Platform Concerns.
