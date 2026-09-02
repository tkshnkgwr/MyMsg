# MyMsg Resource Footprints (FOOTPRINTS)

This document establishes performance targets and reports real-world benchmark metrics (RAM, CPU, startup latency, and binary size) for `MyMsg`.

---

## 1. Target Metrics

| Metric | Target | Rationale |
| :--- | :---: | :--- |
| **Idle CPU Utilization** | **0.0%** | Zero system drag during background automation |
| **Cold Start Latency** | **< 150 ms** | Immediate visual feedback upon trigger |
| **Working Set Memory (RAM)** | **< 35 MB** | Safe execution on memory-constrained VMs |
| **Release Binary Size** | **< 10 MB (uncompressed)** | Ultra-lean single-binary portability |
| **Delay Mode CPU/RAM** | **0.0% / < 5 MB** | Zero GUI allocation while sleeping |
| **Toast Mode CPU/RAM** | **0.0% / < 6 MB** | Instant dispatch and exit (< 20 ms) |

---

## 2. Benchmark Results (Windows 11 x86_64)

| Metric | Measured | Status | Notes |
| :--- | :---: | :---: | :--- |
| **Idle CPU Usage** | **0.0%** | ✅ PASS | Event-driven loop sleeps between events |
| **Blinking CPU Usage** | **0.1%–0.3%** | ✅ PASS | 250ms repaint cadence |
| **Startup Latency (GUI Mode)** | **~80 ms** | ✅ PASS | Instant window display |
| **Execution Time (Toast Mode)** | **~15 ms** | ✅ PASS | Instant toast notification without GUI initialization |
| **RAM Usage (Active GUI)** | **~24 MB** | ✅ PASS | Includes loaded font atlas |
| **RAM Usage (Delay Sleep)** | **~3.2 MB** | ✅ PASS | Minimal runtime footprint before GUI init |
| **Release Binary Size** | **~5.2 MB** | ✅ PASS | Optimized Rust 1.80+ release build (`MyMsg.exe`) |

---

## 3. Architectural Optimizations

1. **Deferred GUI Window Creation**:
   During `--delay` sleep mode, no DirectX / OpenGL handles or GUI structures are allocated, keeping resource overhead negligible.
2. **GUI Bypass in OS Toast Mode**:
   When `--toast` is specified, `eframe` window creation and graphics context setup are completely bypassed, dispatching the notification and terminating in ~15 ms.
3. **Event-Driven Rendering**:
   `egui` operates in pure reactive mode, repainting only on keyboard/mouse events or blink timers.
4. **Clean Exit via `--timeout`**:
   Dispatches `ViewportCommand::Close` upon timeout, preventing memory leaks and orphaned background processes.

