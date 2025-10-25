# Kanban Overlay - Quick Reference Card

## 🚀 Launch
```bash
cargo run --release
```

## ⌨️ Global Shortcuts
| Key | Action |
|-----|--------|
| `Ctrl+Shift+K` | Toggle overlay (show/hide) |
| `Escape` | Hide overlay |
| `Enter` | Execute command |

## 📝 Commands

### Task Management
```bash
# Add task to default column (todo)
add "Task title"

# Add to specific column
add "Task title" to doing
add "Task title" to done

# Add with tags
add "Task title" #urgent #bug
add "Task title" to doing #feature

# Complete example
add "Fix login bug" to doing #urgent #backend
```

### Moving Tasks
```bash
# Move task between columns (use 8-char ID from card)
move a1b2c3d4 to doing
move a1b2c3d4 to done
```

### Editing Tasks
```bash
# Edit task title
edit a1b2c3d4 "New task title"
```

### Deleting Tasks
```bash
# Delete a task
delete a1b2c3d4

# Clear all tasks (careful!)
clear
```

### Viewing
```bash
# List total tasks
list

# List tasks in specific column
list todo
list doing
list done
```

### Help
```bash
# Show command help
help
```

## 🖱️ Mouse Actions

| Action | Result |
|--------|--------|
| **Click & Drag** task | Move between columns |
| **Right-click** task | Open context menu |
| **Context > Delete** | Delete task |
| **Context > Copy ID** | Copy task ID to clipboard |
| **Scroll** in column | Scroll through tasks |

## 🎯 Task IDs

Each task shows an 8-character ID at the top:
```
┌─────────────┐
│ a1b2c3d4    │ ← This is the ID
│ Task Title  │
└─────────────┘
```

Use this ID in commands:
```bash
move a1b2c3d4 to done
edit a1b2c3d4 "Updated title"
delete a1b2c3d4
```

## 🏷️ Tags

Add tags with `#` prefix:
```bash
add "Task" #tag1 #tag2 #tag3
```

Tags appear colored on task cards:
```
┌─────────────────────────┐
│ a1b2c3d4    #urgent #bug│ ← Tags here
│ Fix critical issue      │
└─────────────────────────┘
```

## 💾 Data Storage

**Location:**
- Windows: `C:\Users\YourName\.kanban\state.json`
- Linux/Mac: `~/.kanban/state.json`

**Auto-save:** Every 2 seconds
**Manual save:** Automatic on exit

## 🎨 Default Columns

1. **Todo** - Backlog of tasks
2. **Doing** - Work in progress
3. **Done** - Completed tasks

## 📊 Status Messages

Commands show feedback at the top:
```
> add "Test task" to todo
✓ Added task 'Test task' to todo [a1b2c3d4]
```

## 🔥 Quick Workflow

**Add and start working on a task:**
```bash
add "Implement feature X" to doing #feature
```

**Move task through workflow:**
```bash
move a1b2c3d4 to done
```

**Review what's in progress:**
```bash
list doing
```

## ⚡ Pro Tips

1. **Use short commands:** `a` for add, `m` for move, `d` for delete, `l` for list
2. **Quote titles:** Use quotes for multi-word titles: `add "My task"`
3. **Drag for speed:** Drag & drop is faster than typing move commands
4. **Right-click:** Quickly delete tasks via context menu
5. **Copy IDs:** Right-click > Copy ID when you need task IDs
6. **Keep it open:** Use Ctrl+Shift+K to quickly show/hide, don't close the app

## 🐛 Troubleshooting

**Overlay not showing?**
- Press `Ctrl+Shift+K` again
- Check if minimized to tray
- Restart the application

**Hotkey not working?**
- Another app might be using it
- Try running as administrator (Windows)

**Tasks not saving?**
- Check `~/.kanban/` directory permissions
- Look for errors in console

**Slow performance?**
- Make sure you ran `cargo run --release`
- Try reducing number of tasks
- Close other GPU-intensive apps

## 📚 More Info

- Full documentation: `README.md`
- Developer guide: `DEVELOPER_GUIDE.md`
- Architecture: `ARCHITECTURE.md`

## 🎯 Example Session

```bash
# Start your day
> list
Total tasks: 0

# Add some tasks
> add "Review PR #123" to todo #code
✓ Added task 'Review PR #123' to todo [12345678]

> add "Fix bug in login" to todo #urgent #bug
✓ Added task 'Fix bug in login' to todo [abcd1234]

> add "Update docs" to todo #docs
✓ Added task 'Update docs' to todo [9876fedc]

# Start working
> move 12345678 to doing
✓ Moved task to doing

# Complete work
> move 12345678 to done
✓ Moved task to done

# Check progress
> list
Total tasks: 3

> list done
Done: 1 tasks
```

---

**Remember:** Press `Ctrl+Shift+K` anytime to toggle!

**Happy task managing! 🚀**
