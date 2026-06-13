## Appendix C: The Linux Desktop Reality

In a clean cross-platform world, you write your UI code once, and the operating system provides the window frame — title bar, resize borders, close button — while your framework draws the pixels inside it. On macOS and Windows, this assumption holds. On Linux, it doesn't always.

### Server-Side and Client-Side Decoration

Linux windowing is currently mid-migration from the older X11 protocol to Wayland. Under Wayland, the compositor — the software responsible for drawing the screen — decides how windows are framed. Most modern compositors support an extension called `xdg-decoration-unstable-v1`, which lets applications request standard server-drawn decorations (Server-Side Decoration, or SSD) — the traditional model where the desktop environment draws your title bar and borders.

GNOME, the default environment for Ubuntu, Fedora, and several other major distributions, does not implement this protocol. GNOME's design philosophy favors Client-Side Decoration (CSD) — the idea that applications should draw their own title bars, integrate their toolbars into that space, and manage their own window chrome. This is a deliberate design choice with real motivations behind it — consistency of app integration, reduced duplication between toolbar and title bar — even if the practical effect, for a framework expecting SSD, is jarring.

### What This Means for Your GPUI App

GPUI is a foundational rendering layer, and by default it requests server-drawn decorations.¹ On GNOME under Wayland, a request for server decorations that the compositor doesn't honor can result in a window with no title bar, no resize borders, and no close button — just a frameless rectangle. If your users don't know a window-manager shortcut to close an unresponsive window, this is a genuinely bad first impression.

If you're targeting Linux — and Ubuntu and Fedora users are a meaningful share of any Linux desktop audience — this isn't something to discover after shipping.

### Three Ways to Handle It

**Build your own decorations.** Because GPUI gives you full control over pixels, you can draw your own title bar, implement drag-to-move by handling pointer events and passing move requests to the window manager, and draw your own resize borders. This is real work — you're building a small part of a window manager — but it gives you full control over your app's chrome on every platform, not just Linux.

**Use a component library.** As covered in Chapter 10, libraries like `gpui-component` aim to provide pre-built title bar and window-frame components designed to work correctly under CSD. If you've already adopted one of these libraries for other reasons, check whether it solves this problem for you as well.

**Don't support GNOME Wayland natively.** Some applications simply document that GNOME users should run the app under XWayland — a compatibility layer that makes GNOME behave like X11 and draw a standard frame, sometimes with minor visual quirks. This is the lowest-effort option, but it pushes a workaround onto exactly the users least likely to know about it.

None of these is universally right. The choice depends on how much of your audience runs GNOME, how much control you want over your app's chrome anyway, and how much time you have before shipping. The point of this appendix isn't to tell you which path to take — it's to make sure you're choosing deliberately, rather than discovering the frameless-rectangle problem from a confused bug report after launch.

---

*¹ Verify `WindowDecorations::Server` as GPUI's actual default and exact behavior against current source before publication.*

