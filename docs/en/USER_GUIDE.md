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

### 10-Minute Timed Reminder (600s)
```powershell
MyMsg "10 minutes elapsed. Time to stretch!" -d 600 -c gold -i info
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
