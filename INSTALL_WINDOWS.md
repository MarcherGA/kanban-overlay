# Getting Started on Windows

## Prerequisites

### Install Rust
1. Go to https://rustup.rs/
2. Download and run `rustup-init.exe`
3. Follow the installation wizard
4. Restart your terminal/command prompt

### Verify Installation
Open Command Prompt or PowerShell:
```bash
cargo --version
```
Should show: `cargo 1.x.x`

## Building the Project

### Option 1: Using the Build Script (Easiest)
1. Extract the zip file
2. Open folder in Command Prompt
3. Double-click `build.bat`
   - Or run: `.\build.bat`

### Option 2: Manual Build
```bash
# Navigate to project
cd kanban-overlay

# Build release version
T
```

Build takes 2-5 minutes on first run (downloads dependencies).

## Running the Application

### First Run
```bash
cargo run --release
```

Or directly run the executable:
```bash
.\target\release\kanban-overlay.exe
```

### What to Expect
1. A transparent overlay window appears
2. Command bar at the top shows: `> Type a command...`
3. Three columns: Todo, Doing, Done

## Quick Test

Type these commands to test:
```bash
> add "My first task" to todo
> add "Testing" to doing #test
> list
```

You should see tasks appear in the columns!

## Making it Run on Startup (Optional)

### Method 1: Startup Folder
1. Press `Win + R`
2. Type: `shell:startup`
3. Create shortcut to `kanban-overlay.exe`

### Method 2: Task Scheduler
1. Open Task Scheduler
2. Create Basic Task
3. Trigger: At logon
4. Action: Start program
5. Program: `path\to\kanban-overlay.exe`

## Troubleshooting

### Build Errors

**Error: "linking with `link.exe` failed"**
- Install Visual Studio Build Tools: https://visualstudio.microsoft.com/downloads/
- Select "Desktop development with C++"

**Error: "cannot find -luser32"**
- Install Windows SDK
- Or install full Visual Studio Community

### Runtime Issues

**Hotkey not working:**
- Try running as Administrator
- Check if other apps use Ctrl+Shift+L
- Edit `main.rs` to change hotkey

**Window not transparent:**
- Should work by default on Windows 10/11
- Check Windows visual effects are enabled

**Application crashes:**
- Check console for error messages
- Delete `~\.kanban\state.json` to reset
- Report issue with error message

## Next Steps

1. **Read the Quick Reference:** `QUICKREF.md`
2. **Learn commands:** Type `help` in the app
3. **Customize:** Check `DEVELOPER_GUIDE.md`

## File Locations

**Executable:**
```
kanban-overlay\target\release\kanban-overlay.exe
```

**Data file:**
```
C:\Users\YourName\.kanban\state.json
```

**Config location (for future):**
```
C:\Users\YourName\.kanban\config.toml
```

## Performance Tips

1. **Always build with `--release`** flag for speed
2. Close the app with data saved (don't force kill)
3. Keep task count under 500 for best performance
4. Use SSD for faster load times

## Updating

To update the code:
```bash
# Pull changes (if using git)
git pull

# Rebuild
cargo build --release
```

## Uninstalling

1. Close the application
2. Delete the `kanban-overlay` folder
3. Delete `C:\Users\YourName\.kanban` (optional, removes data)
4. Remove startup shortcut (if created)

## Getting Help

- Check `README.md` for full documentation
- See `DEVELOPER_GUIDE.md` for customization
- Read `ARCHITECTURE.md` to understand how it works

---

**You're all set! Press `Ctrl+Shift+L` and start managing tasks! 🚀**
