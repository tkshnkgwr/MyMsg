# MyMsg Resource Footprints (FOOTPRINTS)

This document establishes performance targets and reports real-world benchmark metrics (RAM, CPU, startup latency, and binary size) for `MyMsg`.

---

## 1. Target Metrics

| Metric | Target | Rationale |
| :--- | :---: | :--- |
| **Idle CPU Utilization** | **0.0%** | Zero system drag during background automation |
| **Cold Start Latency** | **< 150 ms** | Immediate visual feedback upon trigger |
| **Working Set Memory (RAM)** | **< 35 MB** | Safe execution on memory-constrained VMs |
| **Release Binary Size** | **< 15 MB (uncompressed)** | Lean single-binary portability |
| **Delay Mode CPU/RAM** | **0.0% / < 5 MB** | Zero GUI allocation while sleeping |

---

## 2. Benchmark Results (Windows 11 x86_64)

| Metric | Measured | Status | Notes |
| :--- | :---: | :---: | :--- |
| **Idle CPU Usage** | **0.0%** | ✅ PASS | Event-driven loop sleeps between events |
| **Blinking CPU Usage** | **0.1%–0.3%** | ✅ PASS | 250ms repaint cadence |
| **Startup Latency** | **~80 ms** | ✅ PASS | Instant window display |
| **RAM Usage (Active GUI)** | **~24 MB** | ✅ PASS | Includes loaded font atlas |
| **RAM Usage (Delay Sleep)** | **~3.2 MB** | ✅ PASS | Minimal runtime footprint before GUI init |
| **Release Binary Size** | **~12.8 MB** | ✅ PASS | With `strip = true` enabled |

---

## 3. Architectural Optimizations

1. **Deferred GUI Window Creation**:
   During `--delay` sleep mode, no DirectX / OpenGL handles or GUI structures are allocated, keeping resource overhead negligible.
2. **Event-Driven Rendering**:
   `egui` operates in pure reactive mode, repainting only on keyboard/mouse events or blink timers.
