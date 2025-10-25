use crate::state::KanbanState;
use std::path::PathBuf;
use tokio::fs;
use tokio::sync::mpsc;

/// Get the config directory path
pub fn get_config_dir() -> PathBuf {
    let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(".kanban");
    path
}

/// Get the state file path
pub fn get_state_file() -> PathBuf {
    let mut path = get_config_dir();
    path.push("state.json");
    path
}

/// Load state from disk
pub async fn load_state() -> Result<KanbanState, String> {
    let path = get_state_file();

    if !path.exists() {
        return Ok(KanbanState::default());
    }

    let contents = fs::read_to_string(&path)
        .await
        .map_err(|e| format!("Failed to read state file: {}", e))?;

    serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse state file: {}", e))
}

/// Save state to disk (async)
pub async fn save_state(state: &KanbanState) -> Result<(), String> {
    let dir = get_config_dir();

    // Create directory if it doesn't exist
    fs::create_dir_all(&dir)
        .await
        .map_err(|e| format!("Failed to create config directory: {}", e))?;

    let path = get_state_file();
    let contents = serde_json::to_string_pretty(state)
        .map_err(|e| format!("Failed to serialize state: {}", e))?;

    fs::write(&path, contents)
        .await
        .map_err(|e| format!("Failed to write state file: {}", e))?;

    Ok(())
}

/// Background saver that batches writes
pub struct StateSaver {
    tx: mpsc::Sender<KanbanState>,
}

impl StateSaver {
    pub fn new(runtime: &tokio::runtime::Runtime) -> Self {
        let (tx, mut rx) = mpsc::channel::<KanbanState>(10);

        // Spawn background task with lower priority to avoid CPU spikes
        runtime.spawn(async move {
            let mut pending_state: Option<KanbanState> = None;

            loop {
                tokio::select! {
                    // Receive new state
                    Some(state) = rx.recv() => {
                        pending_state = Some(state);
                    }
                    // When we have pending state, wait 5 seconds then save
                    // Longer delay reduces frequency of saves and CPU spikes
                    _ = tokio::time::sleep(std::time::Duration::from_secs(5)), if pending_state.is_some() => {
                        if let Some(state) = pending_state.take() {
                            // Save happens on tokio runtime, won't block main thread
                            if let Err(e) = save_state(&state).await {
                                eprintln!("Failed to save state: {}", e);
                            }
                        }
                    }
                }
            }
        });

        Self { tx }
    }

    /// Queue a state save (non-blocking)
    pub fn save(&self, state: KanbanState) {
        let _ = self.tx.try_send(state);
    }
}

// We need dirs crate for home_dir
// Add this to Cargo.toml dependencies
