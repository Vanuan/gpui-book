# Appendix A: GPUI's Origins — The Zed Interview and What It Reveals

This appendix is optional. Chapter 8 gave you the practical takeaways. Here's the primary source material behind those claims — drawn from a conversation with Nathan Ball, co-founder of Zed Industries.

---

## The Origin

GPUI wasn't born as a general-purpose framework. It was extracted from Zed.

The original 2018 architecture had GPUI 1 spitting JSON at an Electron shell — Zed's first UI was an Electron app launching a Rust binary. From there, GPUI 1 went through phases: a Pathfinder renderer, a Flutter-inspired layout system (constraints down, sizes up), and a split between Rust code and JSON/TypeScript themes. The team later admitted that split worked poorly in practice.

By late 2023, the team was paralyzed. Every time they wanted to add collaboration features, the same question came up: how do you build this? The honest answer was that they didn't have a good workflow. Building UI had become frustrating and miserable.

The catalyst was a contract designer who couldn't make any progress in the old framework.

---

## The Rewrite

The rewrite took four months, from late October to December 2023. The team froze GPUI 1 and built GPUI 2 in parallel. They were never worried that the rewrite itself was a bad idea — the real risk was organizational, coordinating the switch without breaking everything.

The rewrite had three goals. First, unidirectional flow — model code should never be aware of view code. Second, a Flexbox-inspired DSL where you can just write div and child. Third, type safety — previously, pixels were just f32s with no type distinction, leading to silent rendering bugs.

The result solved the problem they set out to solve: building views went from painful to painless.

---

## The Open Source Decision

The team followed a strict sequence: rewrite, stabilize, then open source. Doing a major rewrite after open sourcing would have created too much chaos.

On quality: Nathan describes himself as an unapologetic perfectionist. Beauty matters. The code is now public, so it's part of what he's expressing to the world. It needs to be good.

---

## The License Split: Apache vs GPL

GPUI and Zed live in the same repository but under different licenses. This matters when you're deciding what code you can legally reuse.

**GPUI is Apache 2.0.** The framework — the `gpui` crate and its core primitives — uses Apache License, Version 2.0. This is a permissive license. You can use GPUI to build commercial desktop applications and distribute them under any license you choose. The Apache 2.0 license grants you a perpetual, worldwide, royalty-free copyright license to reproduce, prepare derivative works, and distribute the work.

**Zed the editor is GPL v3.** The editor's source code — features like the workspace, dock, pane, scrollbar, picker, and title bar — falls under GPL version 3. Server-side components use AGPL. The copyleft license ensures that any improvements to the editor itself must be shared back with the community.

**The boundary is enforced.** A developer once asked permission to extract Zed's dock, scrollbar, picker, and title bar components for use in a commercial GPUI project. The response from the Zed team was clear: taking those crates wholesale and relicensing them would violate the GPL, and they couldn't legally permit it. Those components are built specifically for Zed, not intended for reuse outside the editor.

**What this means for you.** You can freely use GPUI itself. Build your app, ship it, keep it closed source if you want. But if you copy code from Zed's GPL-licensed crates — anything from `workspace`, `ui`, or editor-specific components — your project becomes subject to the GPL. The team has plans for a more general-purpose component library under a permissive license someday, but right now, the focus is Zed.

The practical rule: stick to the `gpui` crate and your own code. Admire Zed's architecture, don't copy its GPL'd implementation.

---

## What This Reveals

Four truths about GPUI today:

**One — it's editor-first, not community-first.** The roadmap serves Zed. That's focus, not hostility, but it's a constraint you must accept if you build on upstream.

**Two — the rewrite was about workflow, not features.** GPUI 2 fixed painful UI development. It did not add custom shaders, transforms, or general-purpose graphics. Those gaps exist because Zed doesn't need them.

**Three — open source was an afterthought.** GPUI is an editor's internal framework that happens to be open source, not an open source project that happens to have an editor. This matters when you're betting on it.

**Four — quality is real.** Nathan's perfectionism means thoughtful architecture. It also means features that don't meet his standards — or don't serve Zed — may never arrive.

---

## The Missing Text Input

GPUI does not ship a text input component. This surprises everyone. Zed is a text editor. How can its own framework lack basic text input?

The answer reveals how GPUI works in practice.

GPUI provides the rendering and event primitives: keyboard routing, focus management, cursor rendering, and basic shapes. What it does not provide is the text storage, selection logic, IME handling, or undo/redo stack. Those live in Zed's `text` crate — Rope-based, with sum tree architecture for performance.

Inside Zed, the boundary between "framework" and "editor" is blurry. The text stack is editor code that happens to share a repository with framework code. It was never extracted, documented, or stabilized as a public API.

As a result, you have three options:

**One:** Copy from Zed. The `ui_input` crate contains `SingleLineInput` and `MultiLineInput`. The code is there. The team won't stop you. But it's not versioned, not documented, and may break. And remember the license — copying from Zed's GPL'd crates puts your project under GPL.

**Two:** Use `gpui-component` from the Longbridge trading platform. It provides `Input` with `InputState`, `InputMode::PlainText`, selection, history, and rendering. This is the most complete off-the-shelf solution, but it carries the rest of that library's opinions.

**Three:** Build from GPUI primitives. Keyboard events, focus, and cursor rendering are in the framework. Add `ropey` for text storage, wire selection and IME yourself. Chapter 11 walks through exactly this. Your code stays under your own license.

The absence isn't a technical gap. It's a priority gap. Zed doesn't need a standalone text input — it has an `Editor` that does syntax highlighting, multi-cursor, and LSP. Extracting the simple case is work with no direct benefit to the product. The editor-first constraint in action.

---

## What This Does Not Mean

No licensing hostility. The team chose Apache for GPUI specifically so others could build commercial applications. The GPL on Zed itself is standard for open source editors.

No abandonment. Active maintenance continues — witness the February 2026 PR reimplementing the Linux renderer with wgpu.

No universal warning label. GPUI is optimized for editor-shaped problems. If your project looks like an editor — data-heavy, text-heavy, performance-sensitive — you're in the right place. If it looks like a game or creative tool, you're not.

---

## How To Use This

When you hit a missing feature, you now understand *why*. That tells you whether to work around it, contribute upstream, or fork.

When you evaluate a long-term bet, you now understand the incentives. Upstream serves Zed first. That means faster, more reliable text and data — but no custom shaders or transforms anytime soon.

When you read GitHub discussions or contribute, you now understand the team's perspective. Contributions that help Zed are welcome. Contributions that don't are lower priority.

When you look at Zed's source for inspiration, you now understand the legal boundary. Learn from the architecture. Don't copy the GPL'd code.

