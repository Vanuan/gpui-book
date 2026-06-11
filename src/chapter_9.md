## Chapter 9: The Fork Landscape

Chapter 8 gave you the honest truth about upstream GPUI: it's editor-first, gaps exist, and the team prioritizes Zed's needs over general UI features. So what do you do when you hit a wall — when you need custom shaders, or a unified renderer, or a component library that doesn't require building everything from scratch?

You have options. This chapter maps them.

Why do forks exist at all? By early 2026, the Zed team made it clear that GPUI development would prioritize only features directly related to the editor's use case, pushing off anything outside that scope. PRs for features like custom shaders have been rejected upstream, with the explanation that work not directly usable in Zed isn't being merged.

That's the context. None of this is hostile — it's focus. But focus creates gaps, and gaps create forks.

Here are the major ones you should know about.

---

**WGPUI**

WGPUI solves a specific technical problem: GPUI's platform-specific backends. Upstream GPUI uses Metal on macOS, Blade on Linux, and Direct3D on Windows. This works for Zed, but it means behavior can differ across platforms and adding new rendering features requires implementing them three times.

WGPUI replaces all of that with a single wgpu backend plus winit for window management. The result is one cross-platform renderer, eliminating accidental platform divergence. Text rendering uses cosmic-text and font-kit. The fork has also added features requested by its community: controllable text gradients, blurred UI elements, blurred element content, and smooth scrolling support.

Who uses WGPUI today? Several projects, including Futureboard¹ and the maintainers' own applications. The primary value proposition is unified rendering — if you care deeply about cross-platform consistency and wgpu's architecture, this is your fork.

---

**gpui-ce — Community Edition**

gpui-ce is the most direct response to upstream's narrowing focus. It's maintained by a former early Zed employee along with several community contributors. The goal is explicitly consolidation — keeping a version of GPUI that welcomes features Zed doesn't need.

The fork already has a custom shaders pull request in progress. The maintainers have stated their willingness to compromise on a lot in order to have a consolidated effort around GPUI, which they believe could be many times larger than it is now. People have stuck with GPUI through years of poor usability because they were compelled by its potential. With community maintenance, that potential might finally be realized.

If you want a version of GPUI that tracks upstream but accepts features rejected by Zed, gpui-ce is the most direct path.

---

**gpui-unofficial**

This fork solves a different problem: versioning. Upstream GPUI doesn't publish versioned releases on a predictable cadence. The crates are tied to Zed's monorepo, which makes depending on GPUI from crates.io awkward.

gpui-unofficial is the simplest solution: it publishes GPUI releases tagged to Zed's stable releases. No new features, no backend changes, no community governance. Just a stable mirror with reliable versioning.

If your only complaint about upstream GPUI is that versioning is a headache, use this fork and move on.

---

**Kael**

Kael is the most ambitious independent bet in the GPUI ecosystem. It started as adabraka-gpui, a fork for a video editor called OpenReel. At some point the maintainer realized the framework itself was more valuable than the application, so the project pivoted. The video editor is now just one consumer of Kael.

Today Kael positions itself as a general-purpose desktop UI framework. The scope is substantial: radial and conic gradients in the core styling API, webviews, form controls, rich text, Lottie animation playback, backdrop blur, gesture recognizers, system tray, global hotkeys, notifications, media playback and capture. The built-in component library, kael-ui, ships over one hundred components with eighteen themes and custom theming.

On custom shaders, the maintainer is unequivocal: committed, and at the top of the GPU roadmap. The plan is a public render target and pass API with application-registered shaders and compute pipelines across Metal, DX11, and Vulkan. Offscreen RGBA16F targets and compute on Metal are already working in development.

The maintainer's stance on the relationship with Zed is clear: the Zed team has made their priorities explicit, so Kael was deliberately not structured as a fork that tracks upstream.² Kael keeps the three native backends rather than migrating to wgpu, and builds what desktop apps need without waiting on Zed.

If you want a batteries-included framework with components, media, and custom shaders on the roadmap, Kael is your bet. But understand what you're signing up for: you're leaving upstream behind, by design.

---

**The Practical Decision Tree**

```
Is your app Zed-shaped (text, data, tooling)?
├── Yes → Upstream GPUI
│
├── Need custom shaders or 3D? → Kael
│
├── Need wgpu backend + smooth scrolling? → WGPUI
│
├── Need community governance + upstream tracking? → gpui-ce
│
└── Need stable versioned releases only? → gpui-unofficial
```

---

**The Fragmentation Question**

You might look at this landscape and see fragmentation. You're not wrong. There are now four significant forks plus upstream, each with different priorities and backends.

But there's also a case for optimism. The maintainers are aware of the fragmentation and some are actively working against it. The gpui-ce team has stated their goal is de-fragmentation and willingness to compromise for a consolidated effort. The Kael maintainer has expressed interest in coordinating with gpui-ce on shared pieces — the shape of a custom shader API in particular seems like something worth not designing four different ways.

The GitHub discussion that surfaced much of this also raised an idea worth watching: a headless primitives layer that could bridge the ecosystem without more fragmentation — a clean separation between rendering engine, headless primitives, styled widgets, and application. It's a community suggestion rather than an adopted roadmap item, but it maps directly to the architecture we'll build in Chapter 12.

The forks are a sign of life, not death. People are building things. The framework matters enough to fork.

---

**A Note on Licensing**

Some forks and components may have different licensing than upstream GPUI — see Appendix A for the full boundary.

---

**Bottom Line**

Forks exist because GPUI's focus is Zed, not you. That's not a failure — it's an opportunity. Each fork solves a different problem. The decision tree above gives you the framework; your choice depends on whether you need feature freedom, stable mirrors, or batteries-included primitives.

In the next chapter we'll look at third-party component libraries — which work across multiple forks and give you another layer of reusable UI without committing to a fork at all.

---

*¹ Futureboard is a modern DAW being built with WGPUI. See [futureboard link to be added before publication].*

*² From the Kael fork GitHub discussion thread. [Link to be verified before publication.]*
