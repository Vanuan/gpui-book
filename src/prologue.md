# Prologue

You've heard about Rust.

Maybe a colleague wouldn't stop talking about it. Maybe you saw it top the "most loved language" survey for the eighth year running and wondered what the fuss was about. Maybe you've watched the systems programming world slowly tilt in its direction and filed it under "should probably look at that eventually."

Eventually keeps getting postponed.

Rust has a reputation: steep learning curve, cryptic compiler errors, ownership rules that feel academic until they suddenly click. There's always a more urgent deadline, a more familiar tool, a good enough solution in the language you already know.

This book is for the engineer who never quite got the activation energy.

If you're coming from C++ — you already understand memory, you're just tired of the preprocessor, the build system archaeology, the use-after-free that only manifests in release builds on one platform. You know the performance is worth it. You've quietly wondered if the ceremony has to be this expensive.

If you're coming from TypeScript and React — you understand components, reactivity, and utility-first styling. You've shipped products in Electron and watched a 200MB bundle download just to open a settings window. You've accepted that "native feel" was someone else's problem and explained to a non-technical founder why your desktop app needs 400MB of RAM at idle.

Both of you end up in the same place: wanting something better, not quite finding the reason to start.

GPUI is that reason. It's the UI framework the Zed editor was built with. It thinks in components. It styles with a fluent API that maps directly to your Tailwind intuitions. It runs on a language that makes the promises JavaScript never could — no garbage collector, no IPC bridge between main and renderer, no Chromium. Real cross-platform native desktop apps, small enough to email, fast enough to feel instant.

And agentic coding changes the calculus entirely. You don't have to hold the entire borrow checker in your head on day one. You have a collaborator that knows the rules, catches the compiler errors with you, and explains the why while you focus on building something real. The learning curve is still there — but it's no longer something you climb alone.

This book teaches Rust through GPUI. Every language concept arrives exactly when the framework demands it, motivated by something you've already felt as a limitation. By the end you'll have built and shipped a cross-platform native desktop application — and you'll understand why so many engineers who finally made the jump say they wish they'd done it sooner.

Let's start by opening a window.

