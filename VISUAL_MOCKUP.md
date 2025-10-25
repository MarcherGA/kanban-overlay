# Kanban Overlay - Visual Preview

## Main Window

```
┌────────────────────────────────────────────────────────────────────────────┐
│  >  Type a command... (try 'help')                                    ✕   │
├────────────────────────────────────────────────────────────────────────────┤
│  ✓ Added task 'Fix login bug' to doing [a1b2c3d4]                         │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│  ┌──────────────────────┐  ┌──────────────────────┐  ┌──────────────────┐│
│  │  Todo (3)            │  │  Doing (2)           │  │  Done (1)        ││
│  ├──────────────────────┤  ├──────────────────────┤  ├──────────────────┤│
│  │                      │  │                      │  │                  ││
│  │ ┌──────────────────┐ │  │ ┌──────────────────┐ │  │ ┌──────────────┐││
│  │ │ 12345678  #urgent│ │  │ │ a1b2c3d4    #bug │ │  │ │ 9876fedc     │││
│  │ │                  │ │  │ │                  │ │  │ │              │││
│  │ │ Fix login bug    │ │  │ │ Review PR #123   │ │  │ │ Update docs  │││
│  │ └──────────────────┘ │  │ └──────────────────┘ │  │ └──────────────┘││
│  │                      │  │                      │  │                  ││
│  │ ┌──────────────────┐ │  │ ┌──────────────────┐ │  │                  ││
│  │ │ abcd1234   #feat │ │  │ │ def45678  #test  │ │  │                  ││
│  │ │                  │ │  │ │                  │ │  │                  ││
│  │ │ Add dark mode    │ │  │ │ Test payments    │ │  │                  ││
│  │ └──────────────────┘ │  │ └──────────────────┘ │  │                  ││
│  │                      │  │                      │  │                  ││
│  │ ┌──────────────────┐ │  │                      │  │                  ││
│  │ │ 11112222   #docs │ │  │                      │  │                  ││
│  │ │                  │ │  │                      │  │                  ││
│  │ │ Write API guide  │ │  │                      │  │                  ││
│  │ └──────────────────┘ │  │                      │  │                  ││
│  │                      │  │                      │  │                  ││
│  └──────────────────────┘  └──────────────────────┘  └──────────────────┘│
│                                                                            │
└────────────────────────────────────────────────────────────────────────────┘
      ↑                              ↑                          ↑
  Command Bar                   Task Cards              Three Columns
```

## Color Scheme

```
Background:     Dark blue-gray   RGB(25, 25, 35)   α=240
Border:         Light gray       RGB(60, 60, 80)
Task Card:      Darker gray      RGB(40, 42, 54)
Task Border:    Medium gray      RGB(60, 62, 74)
Text:           White            RGB(255, 255, 255)
Task ID:        Light gray       RGB(128, 128, 128)
Tags:           Blue             RGB(100, 150, 255)
Status:         Green            RGB(100, 200, 100)
Command Prompt: Blue             RGB(100, 150, 255)
```

## Interactions

### Command Bar
```
┌────────────────────────────────────────────────┐
│  >  add "Fix bug" to doing #urgent        ✕  │  ← Type here
└────────────────────────────────────────────────┘
     ↑                                        ↑
  Prompt                                  Clear
  (always visible)                        button
```

### Task Card - Normal State
```
┌──────────────────────────┐
│ a1b2c3d4      #urgent    │  ← ID + Tags
│                          │
│ Fix the login bug        │  ← Title
└──────────────────────────┘
```

### Task Card - Hover State
```
┌══════════════════════════┐  ← Highlight
║ a1b2c3d4      #urgent    ║
║                          ║
║ Fix the login bug        ║  ← Cursor: Grab hand
╚══════════════════════════╝
```

### Task Card - Dragging
```
    ┌──────────────────────┐
    │ a1b2c3d4      #urgent│  ← Following mouse
    │                      │     Semi-transparent
    │ Fix the login bug    │
    └──────────────────────┘
          ↓ ↓ ↓
    Moving to "Done" column
```

### Task Card - Right-Click Menu
```
┌──────────────────────────┐
│ a1b2c3d4      #urgent    │
│                          │
│ Fix the login bug        │
└──────────────────────────┘
  │
  └─→ ┌──────────────┐
      │ Delete       │  ← Context menu
      ├──────────────┤
      │ Copy ID      │
      └──────────────┘
```

### Empty Column Drop Zone
```
┌──────────────────────┐
│  Done (0)            │
├──────────────────────┤
│                      │
│ ╔══════════════════╗ │
│ ║                  ║ │  ← Drop zone appears
│ ║   Drop here      ║ │     when dragging
│ ║                  ║ │
│ ╚══════════════════╝ │
│                      │
└──────────────────────┘
```

## Window States

### Hidden (Ctrl+Shift+L pressed)
```
[Window invisible]
[Process still running in background]
[Ready to show instantly]
```

### Visible (Ctrl+Shift+L pressed again)
```
[Window appears with fade-in]
[Command bar auto-focused]
[Ready for input]
```

## Status Messages

### Success Message
```
┌────────────────────────────────────────────────┐
│  >  add "Test task" to todo                   │
├────────────────────────────────────────────────┤
│  ✓ Added task 'Test task' to todo [12345678]  │  ← Green text
└────────────────────────────────────────────────┘
```

### Error Message
```
┌────────────────────────────────────────────────┐
│  >  move invalidid to done                    │
├────────────────────────────────────────────────┤
│  Error: Task not found: invalidid             │  ← Red/orange text
└────────────────────────────────────────────────┘
```

## Keyboard Focus

### Command Bar Focused (Default)
```
┌────────────────────────────────────────────────┐
│  >  █                                          │  ← Blinking cursor
└────────────────────────────────────────────────┘
```

## Size & Position

```
Default Window Size:  1000x700 pixels
Position:             Center of screen (first launch)
                      Remembers position (future feature)
Transparency:         240/255 (94% opaque)
Always on Top:        Yes
Decorations:          None (no title bar, no window buttons)
Resizable:            Yes (drag edges)
```

## Animations (Optional)

### Show Animation
```
Frame 1:  [Opacity: 0%]
Frame 2:  [Opacity: 30%]
Frame 3:  [Opacity: 60%]
Frame 4:  [Opacity: 100%]  ← Takes ~100ms
```

### Hide Animation
```
Frame 1:  [Opacity: 100%]
Frame 2:  [Opacity: 0%]    ← Instant hide
```

### Drag Animation
```
Task follows mouse pointer smoothly
60 FPS for fluid motion
Slight transparency while dragging
Drop with subtle bounce effect (optional)
```

## Multi-Monitor Support

```
[Monitor 1]           [Monitor 2]
┌─────────────┐      ┌─────────────┐
│             │      │             │
│   Desktop   │      │  ┌────────┐ │  ← Window can be
│             │      │  │ Kanban │ │     moved between
│             │      │  │ Overlay│ │     monitors
│             │      │  └────────┘ │
└─────────────┘      └─────────────┘
```

## Example Workflow Animation

```
1. Press Ctrl+Shift+L
   [Window appears - 150ms]

2. Type command:
   > add "New feature" to doing #priority
   [Text appears as you type]

3. Press Enter
   [Command executes - 5ms]
   [Status message appears: "✓ Added task..."]
   [Task card appears in "Doing" column]
   [Background save queued]

4. Grab task with mouse
   [Hover effect - cursor changes to hand]
   [Click and drag]
   [Task becomes semi-transparent]
   [Follows mouse]

5. Drop in "Done" column
   [Task animates to position]
   [Column counts update]
   [Background save queued]

6. Press Escape or Ctrl+Shift+L
   [Window hides - 25ms]
   [Background save completes]
```

## Dark Theme (Default)

```
╔════════════════════════════════════════════╗
║  Dark blue-gray background                 ║
║  White text for high contrast              ║
║  Subtle borders                            ║
║  Blue accents for interactive elements     ║
║  Translucent effect                        ║
╚════════════════════════════════════════════╝
```

## Light Theme (Future Enhancement)

```
┌────────────────────────────────────────────┐
│  Light gray background                     │
│  Dark text for readability                 │
│  Subtle shadows                            │
│  Blue accents                              │
│  Semi-transparent                          │
└────────────────────────────────────────────┘
```

---

**This is what you're building!** 🎨

The actual application will look and feel like this, with smooth animations, transparent background, and responsive interactions.
