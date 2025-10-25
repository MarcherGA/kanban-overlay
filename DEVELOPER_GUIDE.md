# Developer Quick Start Guide

## Project Structure Explained

### Core Files

1. **`src/state.rs`** - Your Data Model
   - `KanbanState`: The entire board state
   - `Column`: A column (Todo, Doing, Done)
   - `Task`: Individual task cards
   
   Key methods:
   - `add_task()` - Add new task
   - `move_task()` - Move between columns
   - `delete_task()` - Remove task
   - `find_task()` - Search by ID

2. **`src/commands.rs`** - Command Parser
   - `execute_command()` - Main entry point
   - Individual `cmd_*` functions for each command
   - `parse_quoted_or_first()` - Handle quoted strings
   - `find_task_by_short_id()` - Resolve short IDs

3. **`src/ui.rs`** - Visual Rendering
   - `render_ui()` - Main render function
   - `render_command_bar()` - Top input bar
   - `render_columns()` - Kanban columns
   - `render_task_card()` - Individual task cards

4. **`src/persistence.rs`** - Save/Load
   - `load_state()` - Read from disk
   - `save_state()` - Write to disk
   - `StateSaver` - Background auto-save

5. **`src/main.rs`** - Application Entry
   - `main()` - Setup and run
   - `KanbanApp` - Main app struct
   - Hotkey listener thread
   - Window management

## How It Works

### Startup Sequence

```
1. main() is called
2. Create Tokio runtime for async
3. Load state from ~/.kanban/state.json
4. Setup global hotkey (Ctrl+Shift+L)
5. Create shared app state (Arc<Mutex>)
6. Spawn hotkey listener thread
7. Create eframe window with egui
8. Enter event loop
```

### Event Loop

```
Every frame (~60 FPS):
1. Check if window should be visible
2. If hidden, skip rendering
3. If visible:
   - Lock app state
   - Call render_ui()
   - Save state (queued)
   - Unlock state
4. Check for Escape key
5. Request next frame
```

### Command Execution Flow

```
User types: add "Fix bug" to doing #urgent

1. User presses Enter
2. render_command_bar() detects Enter key
3. Calls execute_command() with input
4. execute_command() parses:
   - Command: "add"
   - Calls cmd_add()
5. cmd_add() parses:
   - Title: "Fix bug"
   - Column: "doing"
   - Tags: ["urgent"]
6. Calls state.add_task()
7. State is mutated
8. Success message set
9. Next frame renders updated state
10. Background save is queued
```

### Drag & Drop Flow

```
1. User clicks on task card
2. egui detects drag_started()
3. state.dragging = Some(task_id)
4. User drags over different column
5. egui detects hovered() + pointer_released()
6. Call state.move_task()
7. state.dragging = None
8. Next frame renders updated positions
```

## Key Rust Concepts Used

### 1. Arc<Mutex<T>> - Shared State

```rust
let app_state = Arc::new(Mutex::new(AppState { ... }));
```

- `Arc` = Atomic Reference Counted (shared ownership)
- `Mutex` = Mutual exclusion (thread-safe access)
- Needed because hotkey thread and UI thread both access state

### 2. Serde - Serialization

```rust
#[derive(Serialize, Deserialize)]
struct Task { ... }
```

- Automatically converts structs to/from JSON
- `#[serde(skip)]` excludes fields from serialization

### 3. Tokio - Async Runtime

```rust
let rt = Runtime::new().unwrap();
rt.block_on(async { ... });
```

- Allows async file I/O without blocking UI
- `StateSaver` uses async channel for batching

### 4. Pattern Matching

```rust
match command.as_str() {
    "add" => cmd_add(...),
    "move" => cmd_move(...),
    _ => Err("Unknown command"),
}
```

- Rust's powerful switch-case on steroids
- Compiler ensures all cases are handled

## Common Modifications

### Adding a New Command

1. Add to `commands.rs`:

```rust
fn cmd_yourcommand(args: &[&str], state: &mut KanbanState) -> Result<String, String> {
    // Your logic here
    Ok("Success message".to_string())
}
```

2. Register in `execute_command()`:

```rust
match command.as_str() {
    "yourcommand" | "yc" => cmd_yourcommand(&parts[1..], state),
    // ... existing commands
}
```

### Adding a New Field to Task

1. Add to `Task` struct in `state.rs`:

```rust
pub struct Task {
    // ... existing fields
    pub priority: u8,  // Your new field
}
```

2. Update `Task::new()`:

```rust
pub fn new(title: String, tags: Vec<String>) -> Self {
    Self {
        // ... existing fields
        priority: 0,  // Default value
    }
}
```

3. Add `#[serde(default)]` if optional:

```rust
#[serde(default)]
pub priority: u8,
```

### Changing Window Size

In `main.rs`, modify `NativeOptions`:

```rust
viewport: egui::ViewportBuilder::default()
    .with_inner_size([1200.0, 800.0])  // Change these
    // ...
```

### Adding Keyboard Shortcuts

In `ui.rs`, add to your render function:

```rust
if ctx.input(|i| i.key_pressed(egui::Key::N) && i.modifiers.ctrl) {
    // Create new task
}
```

## Debugging Tips

### Print State

```rust
println!("{:#?}", state.kanban);
```

### Enable Logging

Add to `Cargo.toml`:
```toml
env_logger = "0.11"
```

In `main.rs`:
```rust
env_logger::init();
log::info!("Application started");
```

### Check File Location

```rust
println!("State file: {:?}", persistence::get_state_file());
```

### Inspect egui State

```rust
ui.label(format!("Mouse pos: {:?}", ctx.pointer_latest_pos()));
ui.label(format!("Dragging: {:?}", state.dragging));
```

## Performance Optimization

### 1. Reduce Repaints

Current code requests repaint every frame. For better battery:

```rust
// Only repaint when needed
if state_changed {
    ctx.request_repaint();
} else {
    ctx.request_repaint_after(Duration::from_millis(100));
}
```

### 2. Virtual Scrolling

For >100 tasks per column, implement:

```rust
use egui_extras::{TableBuilder, Column};

TableBuilder::new(ui)
    .column(Column::auto())
    .body(|mut body| {
        for task in visible_tasks {
            body.row(40.0, |mut row| {
                // Render task
            });
        }
    });
```

### 3. Lazy Window Creation

Currently window is always created. For minimum memory:

```rust
// Only create window when visible
if should_be_visible && window.is_none() {
    window = Some(create_window());
}
```

## Testing

### Manual Testing Checklist

- [ ] Add task to each column
- [ ] Move task between columns
- [ ] Delete task
- [ ] Edit task title
- [ ] Drag and drop task
- [ ] Right-click context menu
- [ ] Toggle with hotkey
- [ ] Escape to hide
- [ ] Restart app (verify persistence)
- [ ] Long task titles
- [ ] Many tags
- [ ] Empty columns

### Adding Unit Tests

In `commands.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_add_command() {
        let mut state = KanbanState::default();
        execute_command("add \"Test task\" to todo", &mut state);
        assert_eq!(state.columns[0].tasks.len(), 1);
    }
}
```

Run tests:
```bash
cargo test
```

## Next Steps

1. **Learn Rust basics**: [The Rust Book](https://doc.rust-lang.org/book/)
2. **Explore egui**: [egui documentation](https://docs.rs/egui/)
3. **Add features**: Start with simple commands
4. **Customize UI**: Experiment with colors and layout

## Common Errors & Solutions

### Error: "cannot borrow as mutable"

```rust
// Problem:
for column in &state.columns {
    state.add_task(...);  // Can't mutate while iterating
}

// Solution:
let column_name = state.columns[0].name.clone();
state.add_task(...);
```

### Error: "moved value"

```rust
// Problem:
let task = state.tasks[0];  // Move
println!("{:?}", task);
println!("{:?}", task);  // Error: already moved

// Solution:
let task = &state.tasks[0];  // Borrow instead
// or
let task = state.tasks[0].clone();  // Clone
```

### Error: "cannot return reference"

```rust
// Problem:
fn get_task(&self) -> &Task {
    let task = Task::new(...);
    &task  // Error: returns reference to local
}

// Solution:
fn get_task(&self) -> Task {
    Task::new(...)  // Return owned value
}
```

Happy coding! 🦀
