use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};

/// A unique identifier for tasks (simple 4-digit number)
pub type TaskId = u32;

/// A unique identifier for columns
pub type ColumnId = u32;

/// Global counter for task IDs
static NEXT_TASK_ID: AtomicU32 = AtomicU32::new(1000);

fn next_task_id() -> TaskId {
    let id = NEXT_TASK_ID.fetch_add(1, Ordering::SeqCst);
    // Keep it within 4 digits (1000-9999)
    if id > 9999 {
        NEXT_TASK_ID.store(1000, Ordering::SeqCst);
        1000
    } else {
        id
    }
}

/// The main kanban board state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanState {
    pub columns: Vec<Column>,
    #[serde(skip)]
    pub command_input: String,
    #[serde(skip)]
    pub dragging: Option<TaskId>,
    #[serde(skip)]
    pub status_message: Option<String>,
}

impl Default for KanbanState {
    fn default() -> Self {
        Self {
            columns: vec![
                Column::new("Todo"),
                Column::new("Doing"),
                Column::new("Done"),
            ],
            command_input: String::new(),
            dragging: None,
            status_message: None,
        }
    }
}

impl KanbanState {
    /// Find a column by name (case-insensitive)
    pub fn find_column_by_name(&mut self, name: &str) -> Option<&mut Column> {
        self.columns
            .iter_mut()
            .find(|c| c.name.eq_ignore_ascii_case(name))
    }

    /// Find a task by ID across all columns
    #[allow(dead_code)]
    pub fn find_task(&self, task_id: TaskId) -> Option<(ColumnId, &Task)> {
        for column in &self.columns {
            if let Some(task) = column.tasks.iter().find(|t| t.id == task_id) {
                return Some((column.id, task));
            }
        }
        None
    }

    /// Move a task from one column to another
    pub fn move_task(&mut self, task_id: TaskId, target_column_name: &str) -> Result<(), String> {
        // Find source column and task
        let mut task_to_move = None;

        for column in &mut self.columns {
            if let Some(pos) = column.tasks.iter().position(|t| t.id == task_id) {
                task_to_move = Some(column.tasks.remove(pos));
                break;
            }
        }

        let task = task_to_move.ok_or("Task not found")?;

        // Find target column and add task
        let target_column = self
            .find_column_by_name(target_column_name)
            .ok_or_else(|| format!("Column '{}' not found", target_column_name))?;

        target_column.tasks.push(task);
        Ok(())
    }

    /// Add a new task to a column
    pub fn add_task(&mut self, title: String, column_name: &str, tags: Vec<String>) -> Result<TaskId, String> {
        let column = self
            .find_column_by_name(column_name)
            .ok_or_else(|| format!("Column '{}' not found", column_name))?;

        let task = Task::new(title, tags);
        let task_id = task.id;
        column.tasks.push(task);
        Ok(task_id)
    }

    /// Delete a task by ID
    pub fn delete_task(&mut self, task_id: TaskId) -> Result<(), String> {
        for column in &mut self.columns {
            if let Some(pos) = column.tasks.iter().position(|t| t.id == task_id) {
                column.tasks.remove(pos);
                return Ok(());
            }
        }
        Err("Task not found".to_string())
    }

    /// Edit a task's title
    pub fn edit_task_title(&mut self, task_id: TaskId, new_title: String) -> Result<(), String> {
        for column in &mut self.columns {
            if let Some(task) = column.tasks.iter_mut().find(|t| t.id == task_id) {
                task.title = new_title;
                return Ok(());
            }
        }
        Err("Task not found".to_string())
    }

    /// Set status message
    pub fn set_status(&mut self, message: String) {
        self.status_message = Some(message);
    }
}

/// A column in the kanban board
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Column {
    pub id: ColumnId,
    pub name: String,
    pub tasks: Vec<Task>,
    pub position: usize,
}

static NEXT_COLUMN_ID: AtomicU32 = AtomicU32::new(1);

impl Column {
    pub fn new(name: &str) -> Self {
        Self {
            id: NEXT_COLUMN_ID.fetch_add(1, Ordering::SeqCst),
            name: name.to_string(),
            tasks: Vec::new(),
            position: 0,
        }
    }
}

/// A task card
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub created: DateTime<Utc>,
}

impl Task {
    pub fn new(title: String, tags: Vec<String>) -> Self {
        Self {
            id: next_task_id(),
            title,
            description: None,
            tags,
            created: Utc::now(),
        }
    }

    /// Get task ID as string
    pub fn short_id(&self) -> String {
        self.id.to_string()
    }
}
