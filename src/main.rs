#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod persistence;
mod state;
mod ui;

use eframe::egui;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, hotkey::{Code, Modifiers, HotKey}};
use state::KanbanState;
use std::sync::{Arc, Mutex, OnceLock};
use tokio::runtime::Runtime;
use tray_icon::{TrayIconBuilder, menu::{Menu, MenuItem}};
use tray_icon::TrayIconEvent;

// Global egui context for waking up the UI from background threads
static EGUI_CTX: OnceLock<egui::Context> = OnceLock::new();

fn main() -> Result<(), eframe::Error> {
    // Create tokio runtime for async operations
    let rt = Runtime::new().unwrap();

    // Load initial state
    let initial_state = rt.block_on(async {
        persistence::load_state().await.unwrap_or_default()
    });

    // Setup global hotkey (Ctrl+Shift+K)
    let hotkey_manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyL);
    hotkey_manager.register(hotkey).unwrap();

    // Create app state
    let app_state = Arc::new(Mutex::new(AppState {
        kanban: initial_state,
        visible: true,
        saver: persistence::StateSaver::new(&rt),
    }));

    // Clone for hotkey listener
    let app_state_hotkey = app_state.clone();

    // Setup system tray icon
    let tray_menu = Menu::new();
    let show_item = MenuItem::new("Show/Hide", true, None);
    let quit_item = MenuItem::new("Exit", true, None);
    tray_menu.append(&show_item).unwrap();
    tray_menu.append(&quit_item).unwrap();

    // Create a simple icon (32x32 blue square with white K)
    let icon_rgba = {
        let mut rgba = vec![0u8; 32 * 32 * 4];
        for y in 0..32 {
            for x in 0..32 {
                let idx = (y * 32 + x) * 4;
                // Blue background
                rgba[idx] = 70;      // R
                rgba[idx + 1] = 130; // G
                rgba[idx + 2] = 180; // B
                rgba[idx + 3] = 255; // A
            }
        }
        rgba
    };

    let _tray_icon = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Kanban Overlay - Ctrl+Shift+L to toggle")
        .with_icon(tray_icon::Icon::from_rgba(icon_rgba, 32, 32).unwrap())
        .build()
        .unwrap();

    // Clone for tray event listener
    let app_state_tray = app_state.clone();

    // Spawn tray icon event listener thread
    std::thread::spawn(move || {
        let receiver = TrayIconEvent::receiver();
        loop {
            if let Ok(event) = receiver.recv() {
                // Match on Click event with Left button
                if let TrayIconEvent::Click { button, .. } = event {
                    if button == tray_icon::MouseButton::Left {
                        // Left click on tray icon toggles visibility
                        if let Ok(mut state) = app_state_tray.lock() {
                            state.visible = !state.visible;
                            // Wake up the UI immediately
                            if let Some(ctx) = EGUI_CTX.get() {
                                ctx.request_repaint();
                            }
                        }
                    }
                }
            }
        }
    });

    // Spawn hotkey listener thread
    std::thread::spawn(move || {
        let receiver = GlobalHotKeyEvent::receiver();

        loop {
            if let Ok(event) = receiver.recv() {
                // CRITICAL: Only toggle on key PRESS, ignore RELEASE
                // This is exactly how FlowLauncher works - prevents rapid toggling
                if event.state == global_hotkey::HotKeyState::Pressed {
                    if let Ok(mut state) = app_state_hotkey.lock() {
                        state.visible = !state.visible;
                        // Wake up the UI immediately
                        if let Some(ctx) = EGUI_CTX.get() {
                            ctx.request_repaint();
                        }
                    }
                }
                // Ignore Released events - no action needed
            }
        }
    });

    // Configure window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_resizable(true)
            .with_taskbar(false),  // Don't show in taskbar
        ..Default::default()
    };

    // Run the app
    eframe::run_native(
        "Kanban Overlay",
        options,
        Box::new(|_cc| Ok(Box::new(KanbanApp::new(app_state, rt)))),
    )
}

struct AppState {
    kanban: KanbanState,
    visible: bool,
    saver: persistence::StateSaver,
}

struct KanbanApp {
    state: Arc<Mutex<AppState>>,
    runtime: Runtime,
    normal_size: [f32; 2],
    normal_pos: Option<egui::Pos2>,
    last_visible_state: bool,  // Track previous visibility to detect transitions
}

impl KanbanApp {
    fn new(state: Arc<Mutex<AppState>>, runtime: Runtime) -> Self {
        Self {
            state,
            runtime,
            normal_size: [1000.0, 700.0],
            normal_pos: None,
            last_visible_state: true,  // Start visible
        }
    }
}

impl eframe::App for KanbanApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // DEBUG: Log what events trigger updates
        ctx.input(|i| {
            if !i.events.is_empty() {
                eprintln!("[DEBUG] update() called with events: {:#?}", i.events);
            }
        });

        // Store context globally on first frame so background threads can wake up the UI
        let _ = EGUI_CTX.set(ctx.clone());

        // Check if we should be visible
        let should_be_visible = {
            let state = self.state.lock().unwrap();
            state.visible
        };

        // WORKAROUND for egui bug #5229: ViewportCommand::Visible(true) doesn't work after Visible(false)
        // Instead of hiding, we use a "ghost window" approach like FlowLauncher:
        // - When "hidden": shrink to 1x1 pixel and move off-screen
        // - When "shown": restore size/position and focus

        // Detect visibility transition to avoid sending commands every frame
        let visibility_changed = should_be_visible != self.last_visible_state;
        self.last_visible_state = should_be_visible;

        if should_be_visible {
            // SHOW: Restore window to normal size and position

            // Only send viewport commands when transitioning from hidden to visible
            // Sending them every frame causes continuous repaints
            if visibility_changed {
                // Enable always-on-top when showing
                ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::AlwaysOnTop));

                // Restore normal size
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(self.normal_size.into()));

                // Restore position if we saved one
                if let Some(pos) = self.normal_pos {
                    ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(pos));
                }

                // Focus the window
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            }

            // Save current position if we have it (for next show)
            if let Some(pos) = ctx.input(|i| i.viewport().outer_rect.map(|r| r.min)) {
                if pos.x > -10000.0 {  // Only save if not off-screen
                    self.normal_pos = Some(pos);
                }
            }

            // Render UI
            {
                let mut app_state = self.state.lock().unwrap();
                ui::render_ui(ctx, &mut app_state.kanban);

                // Only save state when there's actual interaction (ctx has events)
                // Don't save every frame - wasteful in reactive mode
                if ctx.input(|i| i.events.len() > 0) {
                    app_state.saver.save(app_state.kanban.clone());
                }
            }

            // Check for Escape to hide
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                let mut state = self.state.lock().unwrap();
                state.visible = false;
            }

            // Use reactive mode: only repaint when there are input events
            // This allows CPU to drop to 0-1% when idle, just like FlowLauncher
            // egui automatically repaints on mouse/keyboard input
        } else {
            // HIDE: Shrink to 1x1 pixel and move off-screen (ghost window approach)

            // Only send viewport commands when transitioning from visible to hidden
            if visibility_changed {
                // CRITICAL: Disable always-on-top when hidden to avoid conflicts with other overlays (NVIDIA, Discord, etc.)
                // This prevents z-order battles that cause continuous redraws
                ctx.send_viewport_cmd(egui::ViewportCommand::WindowLevel(egui::WindowLevel::Normal));

                // Move window far off-screen
                ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(egui::Pos2::new(-10000.0, -10000.0)));

                // Shrink to minimal size
                ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::Vec2::new(1.0, 1.0)));
            }

            // When hidden: DO NOT request periodic repaints
            // Background threads will call ctx.request_repaint() when needed
            // This achieves TRUE 0% CPU usage like FlowLauncher!
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Save state one final time
        let state = self.state.lock().unwrap();
        self.runtime.block_on(async {
            let _ = persistence::save_state(&state.kanban).await;
        });
    }
}
