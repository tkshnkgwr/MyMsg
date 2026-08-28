# MyMsg Architecture Design (ARCHITECTURE)

This document details the internal architecture, data flow, module breakdown, state management, rendering lifecycle, and error handling model of `MyMsg`.

---

## 1. Architectural Overview

`MyMsg` is built upon Rust's strict safety/ownership model and `eframe` (Immediate Mode GUI), delivering zero-overhead desktop popup alerts.

```mermaid
graph TD
    CLI[CLI Execution] --> ArgParser[clap::Parser / CliArgs]
    ArgParser --> DelayCheck{delay > 0 ?}
    DelayCheck -- Yes --> Sleep[std::thread::sleep Wait]
    Sleep --> MonDetect[get_active_monitor_center_position: Active Monitor Centering]
    DelayCheck -- No --> MonDetect
    MonDetect --> WinInit[eframe::run_native: with_position & centered: false]
    WinInit --> FontSetup[setup_japanese_fonts: CJK Font Setup]
    FontSetup --> AppState[MyMsgApp::new Initialization]
    AppState --> EventLoop[eframe::App::update Event Loop]
    EventLoop --> KeyCheck{Esc / Enter Pressed?}
    KeyCheck -- Yes --> CloseCmd[ViewportCommand::Close]
    KeyCheck -- No --> Render[egui::CentralPanel & ScrollArea Render]
```

---

## 2. Source Code Module Breakdown

The codebase is refactored into 6 modular files organized by responsibility:

```
src/
├── main.rs       # Application entry point (main, delay sleep, viewport setup)
├── cli.rs        # CLI argument models (CliArgs, IconType, ThemeMode, dimension resolution)
├── color.rs      # Color & theme parser (parse_color, ThemePalette, palette resolver)
├── font.rs       # System font auto-detection & registration (setup_japanese_fonts)
├── monitor.rs    # Multi-monitor active cursor display positioning (Win32 API)
└── app.rs        # GUI state model (MyMsgApp) & rendering loop (eframe::App)
```

---

## 3. Core Structs & Data Types

### 3.1 `CliArgs` (`src/cli.rs`)
Derived via `clap::Parser` to map command-line flags safely into typed fields:


```rust
pub struct CliArgs {
    pub message_arg: Option<String>,
    pub message_opt: Option<String>,
    pub size: String,
    pub font_size: Option<f32>,
    pub color: Option<String>,
    pub bg_color: Option<String>,
    pub blink: bool,
    pub font: String,
    pub delay: u64,
    pub icon: Option<String>,
    pub theme: Option<String>,
}
```

### 2.2 `MyMsgApp` (Application State)
Holds execution state during GUI runtime:

```rust
pub struct MyMsgApp {
    pub message: String,          // Display string
    pub text_color: Color32,      // Base text color
    pub bg_color: Color32,        // Background color
    pub font_id: FontId,          // Font size & family
    pub blink: bool,              // Blink animation flag
    pub start_time: Instant,      // Start time for phase calculation
    pub icon: Option<IconType>,   // Notification icon enum
    pub theme: ThemeMode,         // Theme mode enum
}
```

---

## 3. Rendering Lifecycle & Event Loop

### 3.1 Immediate Mode GUI Model
`egui` re-evaluates UI definitions on every frame triggered by events:

1. **Input Handling Phase**:
   Uses `ctx.input(|i| ...)` to intercept `Escape` and `Enter` key presses, instantly issuing `ViewportCommand::Close`.
2. **Blink Animation Phase**:
   If `blink == true`, computes opacity phase from `start_time.elapsed()` and requests low-power repaints via `ctx.request_repaint_after(Duration::from_millis(250))`.
3. **Panel Layout Phase**:
   Applies slim framing margins (`Margin::symmetric(16.0, 10.0)`) in `egui::CentralPanel` to render centered text and the bottom action bar.

---

## 4. Font Pipeline (`setup_japanese_fonts`)

When the GUI context initializes (`cc.egui_ctx`), system CJK fonts are loaded dynamically from OS paths:

```mermaid
sequenceDiagram
    participant Main as main()
    participant Runner as eframe::run_native
    participant Ctx as egui::Context
    participant FS as Local FileSystem

    Main->>Runner: Init (native_options, closure)
    Runner->>Ctx: Create CreationContext
    Runner->>Main: Box::new(|cc| ...)
    Main->>FS: Load C:\Windows\Fonts\meiryo.ttc (etc.)
    FS-->>Main: Font Binary Data (Vec<u8>)
    Main->>Ctx: set_fonts(FontDefinitions)
    Main->>Runner: Ok(Box::new(MyMsgApp))
```

---

## 5. Error Handling Model

- **CLI Syntax Errors**: `clap` outputs clear errors and usage instructions to stderr, exiting with non-zero code.
- **Color Parsing Failures**: Unrecognized colors fall back safely to defaults (text: `#F0F0F0`, background: `#1A1B26`).
- **Font Missing**: If system fonts are missing, falls back cleanly to default built-in fonts without panicking.
- **Out-of-Range Delays**: Clamped safely to `0`–`3600` seconds via `clamp_delay_seconds`.
