## Chapter 14: Distribution

You have built your application. The text input works, the platform features are wired, and the pure core is thoroughly tested. Now comes the final step: getting your binary into users' hands.

Distribution is where GPUI's native foundation pays its largest dividend. No runtime, no embedded browser, no second executable to manage. Just a compiled binary and the system libraries it depends on.

### Binary Size: The Actual Numbers

A minimal Electron app — `node_modules`, Chromium, and a few hundred lines of JavaScript — typically runs 80 to 150 megabytes compressed, often over 200 megabytes unpacked. Every Electron app ships the entire Chromium browser engine, regardless of whether the app needs tabs, devtools, or a PDF viewer.

A minimal GPUI application, compiled in release mode with LTO enabled and stripped of debug symbols, typically lands in the single-digit to low double-digit megabytes — roughly a factor of six to ten times smaller, depending on platform and what your app includes.

Where does the difference come from? Electron ships a full web browser alongside your code. GPUI ships only what you use: the windowing glue, the GPU renderer, your application logic, and exactly the assets you include. No dead code, no hidden browser features.

To put a number on it: a markdown previewer with the functionality we sketched in Chapter 12, built with GPUI and optimized for size, comes in around 8 megabytes. A comparable Electron application — same features, plus Chromium — exceeds 90 megabytes. The exact figures will vary by platform and what your app includes, but the order of magnitude is the point: GPUI ships your app, not a browser.



### Build Pipeline

The simplest distribution pipeline for a GPUI application is `cargo build --release`. This produces a binary in `target/release/` that you can rename, sign, and bundle.

```bash
cargo build --release --target x86_64-pc-windows-msvc   # Windows
cargo build --release --target x86_64-apple-darwin      # macOS Intel
cargo build --release --target aarch64-apple-darwin     # macOS Apple Silicon
cargo build --release --target x86_64-unknown-linux-gnu # Linux
```

For cross-compilation from a single machine, use `cross`, which manages toolchain and linker configuration via Docker containers.

```bash
cross build --release --target aarch64-apple-darwin
```

### Code Signing and Notarization

Code signing is non-negotiable for distribution on macOS and Windows. Users expect applications that open without security warnings.

**On macOS:** You need an Apple Developer ID certificate. Sign the binary with `codesign`, then submit it to Apple for notarization using `xcrun notarytool`. GPUI's binary is signed like any other executable — no framework-specific steps required.

```bash
codesign --force --options runtime --sign "Developer ID Application: Your Name" myapp
ditto -c -k --keepParent myapp.app myapp.zip
xcrun notarytool submit myapp.zip --apple-id your@email.com --team-id TEAMID --wait
```

**On Windows:** You need a code signing certificate from a trusted authority. Sign the executable with `signtool`:

```bash
signtool sign /fd SHA256 /a /f mycert.pfx /p password myapp.exe
```

**On Linux:** Code signing is less common, but you can distribute via package repositories that verify GPG signatures.

### Bundling for Distribution

Each platform expects applications in a specific format.

**On macOS:** A `.app` bundle is a directory structure with a fixed layout. GPUI does not require any special entries in `Info.plist` beyond the standard executable name and bundle identifier. Use `cargo-bundle` or write a simple script to create the bundle:

```
MyApp.app/
└── Contents/
    ├── Info.plist
    ├── MacOS/
    │   └── myapp
    └── Resources/
```

**On Windows:** A standalone `.exe` works, but users expect installers. Tools like `Inno Setup` or `WiX Toolset` can wrap your executable into a traditional installer. MSIX packaging is also an option for distribution through the Microsoft Store.

**On Linux:** Package formats vary by distribution. `.deb` for Debian/Ubuntu, `.rpm` for Fedora/RHEL, and AppImage for distribution-agnostic portable binaries. AppImage is the simplest cross-distribution option: it bundles the executable and its dependencies into a single file that runs on any Linux system.

### Fork-Specific Distribution Considerations

If you built your application on a fork, distribution may change.

**WGPUI** uses the same binary pipeline as upstream GPUI. No additional steps required.

**Kael** includes additional dependencies for media playback, networking, and system tray integration. These may pull in platform-specific dynamic libraries. If you distribute as a single static binary, ensure those dependencies are linked statically or bundle them alongside your executable. Kael's documentation provides the exact library list for each platform.

**gpui-ce** tracks upstream closely, so distribution is identical to upstream GPUI.

**gpui-unofficial** is a versioned mirror of upstream. No changes to distribution.

### The Update Problem

Electron has a built-in auto-updater. GPUI does not.

If your application needs automatic updates, you must build that logic yourself. The pattern is straightforward: check a remote manifest on startup, download a new binary if available, replace the current executable, and relaunch. For signed binaries, verify the signature before replacement.

Several Rust crates provide update infrastructure. `self_update` and `update-informer` are popular choices that integrate with GitHub Releases, Amazon S3, or custom servers. GPUI's async system can drive the download and installation without blocking the UI.

The trade-off is clear: you control the update experience, but you also own the implementation.

### The Electron Comparison, Finalized

| Metric | GPUI | Electron |
|--------|------|----------|
| Minimal binary size | 8–15 MB | 80–150 MB |
| Memory overhead | Application only | Chromium + Node.js |
| Startup time | Instant (0.1–0.3s) | Delayed (0.5–1.5s) |
| Native feel | Full | Emulated |
| Auto-updater | You build | Built-in |
| Cross-platform UI consistency | Your responsibility | High (web standards) |

GPUI wins on size, speed, and native integration. Electron wins on out-of-the-box updates and web rendering consistency. Neither is universally better. The right choice depends on your application's priorities.

### Final Checklist

Before shipping, verify:

- [ ] Release build compiles with `--release` and LTO enabled
- [ ] Binary is stripped of debug symbols (`strip` or `split-debuginfo`)
- [ ] Code signing is applied for macOS and Windows
- [ ] Bundling matches platform expectations (`.app`, `.exe`, `.deb`, or AppImage)
- [ ] Update mechanism is implemented or explicitly skipped
- [ ] Fork-specific dependencies are accounted for (Kael's media libraries, etc.)

### Next Steps

Your application is now ready for users.

But before you close this book, read the Epilogue. It answers the question you may still be asking: "Was this the right investment, given the fragmentation?" The answer is more hopeful than you might think.

---

