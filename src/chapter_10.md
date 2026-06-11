# Chapter 10: Third-Party Components

You've made it through the fork landscape. You understand the upstream constraints and have chosen a path. Now you need to actually build your UI — buttons, inputs, dialogs, tables. Do you build everything from scratch using GPUI's primitives, or do you stand on the shoulders of those who've gone before?

This chapter maps the third-party component landscape. Unlike the forks in Chapter 9, which change the framework itself, these libraries work on top of GPUI. You can use them with upstream or with most forks, and they range from focused component collections to full-blown UI toolkits.

To add any of these libraries to your project, the pattern is the same: `cargo add gpui-component` (or `gpui-ui-kit`, etc.) and follow the library's specific setup instructions. Now let's look at what each one offers.

---

## gpui-component

The most complete component library in the ecosystem is gpui-component, built by the team at Longbridge for their trading platform. It's not an academic exercise — it's production code running a real financial application, which means it has been battle-tested against serious performance requirements.

The library includes over sixty cross-platform desktop UI components, drawing inspiration from native macOS and Windows controls while blending in modern design language from shadcn/ui. The feature set is extensive: virtualized tables and lists for smooth rendering of large datasets, built-in charting capabilities, a high-performance code editor supporting up to two hundred thousand lines with Language Server Protocol integration, dock layouts for panel arrangements, and native Markdown with basic HTML rendering.

The component list covers most of what a typical desktop app needs. For core UI, you get buttons with multiple variants, cards, dialogs, menus, menu bars, tabs, and toasts. For forms, there's input with rich editing capabilities including mouse drag selection, clipboard operations, and Emacs keybindings, plus number inputs with spin buttons, checkboxes, toggles, selects, color pickers, sliders, and wizards. For data display, badges, progress indicators, spinners, avatars, and typography components. For feedback, alerts, inline alerts, and tooltips. For layout, stacks, spacers, dividers, pane dividers, accordions, and breadcrumbs.

The library has one non-negotiable requirement: your entire application must be wrapped in a `Root` component that provides theming and global state management. This is not optional. If you're integrating gpui-component into an existing app, you'll need to refactor to put `Root` at the top of your view tree. The licensing is Apache 2.0, matching GPUI itself, so you can use it in commercial applications without concern.

---

## gpui-ui-kit

If gpui-component is the production workhorse, gpui-ui-kit is the more experimental sibling. It offers a similar range of components with some notable additions, particularly controls that are well-suited for audio applications — potentiometer knobs, vertical sliders, and volume controls.

The component set overlaps heavily with gpui-component: buttons, icon buttons, cards, dialogs, menus, tabs, toasts, inputs, checkboxes, toggles, selects, color pickers, sliders, badges, progress indicators, spinners, avatars, typography, alerts, tooltips, stacks, spacers, dividers, and breadcrumbs.

Where gpui-ui-kit distinguishes itself is in its audio-focused controls and its documentation presentation. The library is well-documented on docs.rs, and the API feels slightly more polished in places. That said, it has fewer components overall than gpui-component and lacks some of the advanced features like virtualized tables and built-in charting.

Use gpui-ui-kit when you need audio-style controls or prefer its API design. Use gpui-component when you need maximum component coverage and production pedigree.

---

## gpui-kit (Vitesse)

This one is worth watching but not yet ready for prime time. gpui-kit, also called Vitesse, is an upcoming UI toolkit with an ambitious three-layer architecture: headless primitives in the vein of Radix or Headless UI, a color library with optional themes, and styled components built on top. The goal is to provide accessible, unstyled building blocks that you can customize completely, separate from opinionated design decisions.

As of this writing, the project is still in planning. The repository exists, the vision is documented, but there's not yet a stable release you can depend on. Keep an eye on it. If the team delivers on the headless primitives layer, it could become the foundation that the ecosystem desperately needs.

---

## kael-ui

Remember Kael from Chapter 9? The batteries-included fork ships with its own component library, kael-ui, living inside the same monorepo. This is not a separate library you can pull in independently — it's part of the Kael framework. The component set is substantial: over one hundred components with eighteen built-in themes and full custom theming support.

Because kael-ui is tightly coupled to the Kael fork, you can't easily use it with upstream GPUI or other forks. The components assume Kael's extended styling API, its custom primitives, and its rendering backend. If you've chosen Kael as your framework, you get kael-ui for free. If you're on upstream, this is not an option.

---

## The Missing Headless Primitives Layer

Here's the problem that none of these libraries fully solves.

Every component library currently available is opinionated. gpui-component has a specific look. gpui-ui-kit has another. kael-ui has a third. If you don't like their default styling, or if your application's design language doesn't match, you're left fighting the library's theming system — or rebuilding components from scratch.

What's missing is a headless primitives layer: unstyled, accessible components that handle behavior, focus management, keyboard interactions, and accessibility attributes, but leave styling completely to the developer. This is what Radix provides for React, what Headless UI provides for Vue, and what gpui-kit (Vitesse) is promising but hasn't delivered.

The absence of this layer matters most for text input. As we discussed in Appendix A, GPUI itself doesn't ship an input component. The third-party libraries do, but each comes with its own styling and behavior assumptions. If you need a text input that matches your custom design, you're back to building it yourself from GPUI's primitives, as we'll cover in Chapter 11.

---

## When to Use a Library vs. Build Your Own

Here's the decision rule.

Use gpui-component if you want the most complete, production-tested component set and you're willing to accept its design language and the `Root` wrapper requirement. This is the fastest path from zero to functional app.

Use gpui-ui-kit if you prefer its API or need controls suited for audio applications.

Use kael-ui if you've already committed to the Kael framework. You get components as part of the deal.

Build your own components if your application has a custom design language that doesn't match these libraries, or if you need fine-grained control over behavior that the libraries don't expose. The effort is higher, but the result is uniquely yours.

Wait for gpui-kit if you want headless primitives and are willing to be patient. The project hasn't shipped yet, but its vision is the right one for the ecosystem.

---

## The Root Problem

One more thing about gpui-component's `Root` requirement. It's not arbitrary — the `Root` component manages theme context, global shortcuts, dialog stacks, and notification toasts. This is a reasonable architectural choice. But it means you cannot incrementally adopt gpui-component. Your entire application must be wrapped from the start, or you refactor later.

If you're starting a new project, this is fine. Add `Root` at the top of your view hierarchy and forget about it. If you're adding gpui-component to an existing project, budget time for the refactor.

---

## Bottom Line

The third-party component landscape is healthier than the fork landscape suggests. gpui-component gives you over sixty production-ready components. gpui-ui-kit offers a solid alternative with audio-focused extras. kael-ui is there if you're on Kael. And gpui-kit points toward a headless future that doesn't yet exist.

Each library makes design decisions you may want to override. That's not a flaw — it's the nature of opinionated code. The question is whether those opinions align with your needs.

In the next chapter, we'll drop down to the foundations: text and data. Because whether you use a component library or build your own, you need to understand how GPUI handles text storage, selection, and input. And that's where things get interesting.

