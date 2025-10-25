use crate::state::{KanbanState, TaskId};

/// Parse and execute a command
pub fn execute_command(input: &str, state: &mut KanbanState) {
    let input = input.trim();
    if input.is_empty() {
        return;
    }

    let parts: Vec<&str> = input.split_whitespace().collect();
    let command = parts[0].to_lowercase();

    let result = match command.as_str() {
        "add" | "a" => cmd_add(&parts[1..], state),
        "move" | "mv" | "m" => cmd_move(&parts[1..], state),
        "delete" | "del" | "d" => cmd_delete(&parts[1..], state),
        "edit" | "e" => cmd_edit(&parts[1..], state),
        "list" | "ls" | "l" => cmd_list(&parts[1..], state),
        "clear" | "c" => {
            // Clear all tasks (with confirmation in future)
            for column in &mut state.columns {
                column.tasks.clear();
            }
            Ok("Cleared all tasks".to_string())
        }
        "help" | "h" | "?" => cmd_help(),
        _ => Err(format!("Unknown command: {}. Type 'help' for commands.", command)),
    };

    match result {
        Ok(msg) => state.set_status(msg),
        Err(err) => state.set_status(format!("Error: {}", err)),
    }
}

/// Add a new task
/// Usage: add "task title" [to column] [#tag1 #tag2]
fn cmd_add(args: &[&str], state: &mut KanbanState) -> Result<String, String> {
    if args.is_empty() {
        return Err("Usage: add \"task title\" [to column] [#tags]".to_string());
    }

    // Parse task title (look for quoted string or take first arg)
    let (title, remaining) = parse_quoted_or_first(args)?;

    // Parse optional "to column"
    let mut column_name = "todo";
    let mut tag_start = 0;

    for (i, arg) in remaining.iter().enumerate() {
        if arg.eq_ignore_ascii_case("to") && i + 1 < remaining.len() {
            column_name = remaining[i + 1];
            tag_start = i + 2;
            break;
        }
        if arg.starts_with('#') {
            tag_start = i;
            break;
        }
    }

    // Parse tags
    let tags: Vec<String> = remaining[tag_start..]
        .iter()
        .filter(|s| s.starts_with('#'))
        .map(|s| s.trim_start_matches('#').to_string())
        .collect();

    let task_id = state.add_task(title.clone(), column_name, tags)?;
    Ok(format!("Added task '{}' to {} [{}]", title, column_name, task_id))
}

/// Move a task to another column
/// Usage: move <task-id> to <column>
fn cmd_move(args: &[&str], state: &mut KanbanState) -> Result<String, String> {
    if args.len() < 3 {
        return Err("Usage: move <task-id> to <column>".to_string());
    }

    let task_id = parse_task_id(args[0])?;

    if !args[1].eq_ignore_ascii_case("to") {
        return Err("Expected 'to' after task ID".to_string());
    }

    let column_name = args[2];
    state.move_task(task_id, column_name)?;
    Ok(format!("Moved task to {}", column_name))
}

/// Delete a task
/// Usage: delete <task-id>
fn cmd_delete(args: &[&str], state: &mut KanbanState) -> Result<String, String> {
    if args.is_empty() {
        return Err("Usage: delete <task-id>".to_string());
    }

    let task_id = parse_task_id(args[0])?;
    state.delete_task(task_id)?;
    Ok(format!("Deleted task {}", args[0]))
}

/// Edit a task's title
/// Usage: edit <task-id> "new title"
fn cmd_edit(args: &[&str], state: &mut KanbanState) -> Result<String, String> {
    if args.len() < 2 {
        return Err("Usage: edit <task-id> \"new title\"".to_string());
    }

    let task_id = parse_task_id(args[0])?;
    let (new_title, _) = parse_quoted_or_first(&args[1..])?;

    state.edit_task_title(task_id, new_title)?;
    Ok(format!("Updated task {}", args[0]))
}

/// List tasks in a column or all columns
/// Usage: list [column]
fn cmd_list(args: &[&str], state: &mut KanbanState) -> Result<String, String> {
    if args.is_empty() {
        // List all
        let total: usize = state.columns.iter().map(|c| c.tasks.len()).sum();
        Ok(format!("Total tasks: {}", total))
    } else {
        // List specific column
        let column_name = args[0];
        if let Some(column) = state.find_column_by_name(column_name) {
            Ok(format!("{}: {} tasks", column.name, column.tasks.len()))
        } else {
            Err(format!("Column '{}' not found", column_name))
        }
    }
}

/// Show help message
fn cmd_help() -> Result<String, String> {
    Ok(r#"Commands:
  add "title" [to column] [#tags]  - Add a task
  move <id> to <column>            - Move a task
  delete <id>                      - Delete a task
  edit <id> "new title"            - Edit a task
  list [column]                    - List tasks
  clear                            - Clear all tasks
  help                             - Show this help
  
Task IDs are the first 8 characters shown on each card.
Press Ctrl+Shift+K to toggle overlay."#.to_string())
}

/// Parse a quoted string or take the first argument
fn parse_quoted_or_first<'a>(args: &'a [&'a str]) -> Result<(String, Vec<&'a str>), String> {
    if args.is_empty() {
        return Err("Expected argument".to_string());
    }

    // Check if first arg starts with quote
    if args[0].starts_with('"') {
        // Find closing quote
        let mut title_parts = Vec::new();
        let mut found_closing = false;
        let mut remaining_start = 0;

        for (i, arg) in args.iter().enumerate() {
            if i == 0 {
                title_parts.push(arg.trim_start_matches('"'));
            } else {
                title_parts.push(*arg);
            }

            if arg.ends_with('"') {
                *title_parts.last_mut().unwrap() = title_parts.last().unwrap().trim_end_matches('"');
                found_closing = true;
                remaining_start = i + 1;
                break;
            }
        }

        if !found_closing {
            return Err("Unclosed quote".to_string());
        }

        let title = title_parts.join(" ").trim_matches('"').to_string();
        let remaining = args[remaining_start..].to_vec();
        Ok((title, remaining))
    } else {
        // No quotes, just take first arg
        Ok((args[0].to_string(), args[1..].to_vec()))
    }
}

/// Parse a task ID (simple number)
fn parse_task_id(id_str: &str) -> Result<TaskId, String> {
    id_str.parse::<u32>()
        .map_err(|_| format!("Invalid task ID: {}", id_str))
}

/// Find task ID by short ID prefix
#[allow(dead_code)]
pub fn find_task_by_short_id(state: &KanbanState, short_id: &str) -> Option<TaskId> {
    for column in &state.columns {
        for task in &column.tasks {
            if task.short_id().starts_with(short_id) {
                return Some(task.id);
            }
        }
    }
    None
}

/// Enhanced parse_task_id that supports short IDs
#[allow(dead_code)]
pub fn parse_task_id_with_state(id_str: &str, state: &KanbanState) -> Result<TaskId, String> {
    // Try parsing as number
    if let Ok(id) = id_str.parse::<u32>() {
        return Ok(id);
    }

    // Try short ID
    find_task_by_short_id(state, id_str)
        .ok_or_else(|| format!("Task not found: {}", id_str))
}
