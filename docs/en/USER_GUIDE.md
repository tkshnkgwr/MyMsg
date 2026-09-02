# MyMsg User Manual (USER_GUIDE)

This document provides a practical user guide, cookbook recipes, and shell integration examples for `MyMsg`.

---

## 1. Basic Usage

### 1.1 Simplest Invocation
```powershell
MyMsg "Build completed successfully!"
```
A native popup window appears centered on the active display where your mouse cursor resides. In multi-monitor setups, it reliably targets your current working monitor.

### 1.2 Dismissing the Popup
Dismiss the window instantly through any of the following actions:
- **Press `Esc`** (fastest keyboard shortcut)
- **Press `Enter`**
- Click the bottom **`[✕ 閉じる (Esc / Enter)]`** button
- Click the standard window title bar **`✕`** button

---

## 2. Cookbook & Quick Recipes

### Large Urgent Alert
```powershell
MyMsg "CRITICAL: Database connection lost!" -s large -c red
```

### Compact Subtle Notification
```powershell
MyMsg "Saved successfully" -s small -c green
```

### Blinking Warning Alert
```powershell
MyMsg "Warning: Low disk space remaining!" -c yellow -b
```

### Custom Hex Theme Colors
```powershell
MyMsg "Batch job finished" -c "#00FFCC" --bg-color "#111827"
```

### Monospaced Code / File Path Notification
```powershell
MyMsg "Artifact path: /var/log/build.log" -f mono
```

### Status Icons for Warnings and Errors
```powershell
# Warning icon
MyMsg "Disk usage exceeded 90% threshold!" -i warn

# Error icon with blink
MyMsg "Database migration failed!" -i error -b

# Success / OK icon
MyMsg "All deployment stages completed!" -i ok
```

### Multi-Line Text with Escape Characters
```powershell
MyMsg "Summary:\n- Success: 120\n- Skipped: 3\n- Failed: 0" -i ok
```

### Light Mode Theme
```powershell
MyMsg "Team sync meeting starting in 5 minutes" -t light -i info
```

### Timed Reminders (Seconds, Units, or Exact Time)
```powershell
# 10 minutes from now (supports 10m, 1h, 30s, or seconds)
MyMsg "10 minutes elapsed. Time to stretch!" -d 10m -c gold -i info

# Exact time of day (12:00 lunch reminder)
MyMsg "Lunch break time!" -d 12:00 -i ok

# End of workday desktop banner toast at 18:00
MyMsg "End of day reminder" -d 18:00 --toast -i info
```

### Explicit Monitor Placement
```powershell
MyMsg "Main Screen Alert" --monitor primary -i ok
```

### Auto-Dismissing Popup (5-Second Toast-like Popup)
```powershell
MyMsg "Quick Status Check" --timeout 5 -i ok
```

### OS Native Toast Notification Mode (No GUI Window)
```powershell
MyMsg "Background job completed" --toast -i ok
```

---

## 3. Automation & Shell Integration

### 3.1 PowerShell Recipes

#### Notify on Long-Running Build Completion
```powershell
cargo build --release; MyMsg "Cargo build finished!" -c cyan
```

#### Conditional Branching on Exit Code
```powershell
npm test
if ($LASTEXITCODE -eq 0) {
    MyMsg "All unit tests passed!" -c green
} else {
    MyMsg "Unit tests failed! Check logs." -c red -b
}
```

### 3.2 Windows Batch (.bat) Script
```bat
@echo off
echo Starting backup process...
robocopy C:\Data D:\Backup /MIR
if %ERRORLEVEL% LEQ 1 (
    MyMsg.exe "Backup completed successfully" -c green
) else (
    MyMsg.exe "Backup encountered errors!" -c red -b
)
```

### 3.3 Linux / macOS (Bash / Zsh) Script
```bash
./deploy.sh && \
./MyMsg "Deployment succeeded!" -c cyan || \
./MyMsg "Deployment failed!" -c red -b
```

### 3.4 Windows Task Scheduler & Scheduled Jobs Integration

When triggering `MyMsg` periodically via **Windows Task Scheduler**, `cron`, or background daemons, using **`--timeout` or `--toast` (`-T`) is strongly recommended**.

#### Key Windows Task Scheduler Configuration Guidelines
1. **Security Options**:
   - Ensure **"Run only when user is logged on"** is selected.
   - *Note*: If configured with "Run whether user is logged on or not" or under the `SYSTEM` account, Windows Session 0 Isolation will execute the process in a non-interactive background session, preventing the popup window from being visible on your desktop.
2. **Always Specify `--timeout <seconds>` (Strongly Recommended)**:
   - If an alert pops up while you are away from the computer and no timeout is set, the process will remain open indefinitely.
   - This can cause Task Scheduler's "Do not start a new instance" rule to **block or skip subsequent scheduled runs**.
   - Using `--timeout 15` or `--timeout 30` ensures the window automatically closes and the process exits cleanly.

#### Task Scheduler Action Setup Example
- **Program/script**: `C:\Tools\MyMsg\MyMsg.exe` (full absolute path)
- **Add arguments (optional)**: `"Scheduled backup completed" -i ok -c green --timeout 20`
- **Start in (optional)**: `C:\Tools\MyMsg`

#### OS Toast Mode for Scheduled Notifications
For subtle notifications that appear in the bottom-right action center without interrupting your workflow with a modal popup:
```powershell
MyMsg "Stand up and stretch!" -i info --toast
```

