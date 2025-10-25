use crate::commands::execute_command;
use crate::state::{KanbanState, Task, TaskId};
use egui::{Color32, Frame, Margin, Rounding, Stroke, Vec2};
use std::sync::atomic::{AtomicBool, Ordering};

// Track if style has been initialized to avoid setting it every frame
static STYLE_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn render_ui(ctx: &egui::Context, state: &mut KanbanState) {
    // CRITICAL: Only set style ONCE, not every frame!
    // Setting style every frame causes continuous repaints = high CPU
    if !STYLE_INITIALIZED.load(Ordering::Relaxed) {
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = Color32::from_rgba_premultiplied(25, 25, 35, 240);
        style.visuals.window_rounding = Rounding::same(10.0);
        style.visuals.window_stroke = Stroke::new(1.0, Color32::from_rgb(60, 60, 80));
        ctx.set_style(style);
        STYLE_INITIALIZED.store(true, Ordering::Relaxed);
    }

    egui::CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(8.0, 8.0);

            // Command bar at top
            render_command_bar(ui, state);

            ui.add_space(10.0);

            // Status message
            if let Some(msg) = &state.status_message {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(msg).color(Color32::from_rgb(100, 200, 100)));
                });
                ui.add_space(5.0);
            }

            // Kanban columns
            egui::ScrollArea::horizontal()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    render_columns(ui, state);
                });
        });
}

fn render_command_bar(ui: &mut egui::Ui, state: &mut KanbanState) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(">").color(Color32::from_rgb(100, 150, 255)).size(18.0));

        let response = ui.add(
            egui::TextEdit::singleline(&mut state.command_input)
                .desired_width(ui.available_width() - 80.0)
                .hint_text("Type a command... (try 'help')")
                .frame(true),
        );

        // Only request focus if the input doesn't already have it
        // This prevents calling request_focus() every frame which causes continuous repaints
        if !response.has_focus() && state.command_input.is_empty() {
            response.request_focus();
        }

        // Execute on Enter - check if Enter was pressed while focused
        if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            let command = state.command_input.clone();
            if !command.trim().is_empty() {
                execute_command(&command, state);
                state.command_input.clear();
            }
        }

        // Clear button
        if ui.button("✕").clicked() {
            state.command_input.clear();
        }
    });
}

fn render_columns(ui: &mut egui::Ui, state: &mut KanbanState) {
    ui.horizontal_top(|ui| {
        let column_width = 300.0;
        let num_columns = state.columns.len();

        for col_idx in 0..num_columns {
            render_column(ui, state, col_idx, column_width);
            if col_idx < num_columns - 1 {
                ui.add_space(15.0);
            }
        }
    });
}

fn render_column(ui: &mut egui::Ui, state: &mut KanbanState, col_idx: usize, width: f32) {
    ui.vertical(|ui| {
        ui.set_width(width);

        // Column header
        let column_name = state.columns[col_idx].name.clone();
        let task_count = state.columns[col_idx].tasks.len();

        ui.horizontal(|ui| {
            ui.heading(egui::RichText::new(&column_name).size(20.0));
            ui.label(egui::RichText::new(format!("({})", task_count)).color(Color32::GRAY));
        });

        ui.separator();

        // Tasks
        egui::ScrollArea::vertical()
            .max_height(600.0)
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let tasks = state.columns[col_idx].tasks.clone();
                let mut task_to_delete: Option<TaskId> = None;

                for task in tasks.iter() {
                    let response = render_task_card(ui, task, &column_name);

                    // Handle drag start
                    if response.drag_started() {
                        state.dragging = Some(task.id);
                    }

                    // Handle drop
                    if let Some(dragging_id) = state.dragging {
                        if response.hovered() && ui.input(|i| i.pointer.any_released()) {
                            // Move task to this column
                            let _ = state.move_task(dragging_id, &column_name);
                            state.dragging = None;
                        }
                    }

                    // Context menu (right-click)
                    response.context_menu(|ui| {
                        if ui.button("Delete").clicked() {
                            task_to_delete = Some(task.id);
                            ui.close_menu();
                        }
                        if ui.button("Copy ID").clicked() {
                            ui.output_mut(|o| o.copied_text = task.short_id());
                            ui.close_menu();
                        }
                    });
                }

                // Delete task if requested
                if let Some(id) = task_to_delete {
                    let _ = state.delete_task(id);
                }

                // Drop zone for empty columns
                if tasks.is_empty() {
                    if let Some(dragging_id) = state.dragging {
                        let drop_zone = ui.allocate_response(
                            Vec2::new(width - 20.0, 50.0),
                            egui::Sense::hover(),
                        );

                        Frame::none()
                            .fill(Color32::from_rgba_premultiplied(50, 50, 60, 100))
                            .rounding(Rounding::same(5.0))
                            .stroke(Stroke::new(2.0, Color32::from_rgb(80, 80, 100)))
                            .show(ui, |ui| {
                                ui.centered_and_justified(|ui| {
                                    ui.label("Drop here");
                                });
                            });

                        if drop_zone.hovered() && ui.input(|i| i.pointer.any_released()) {
                            let _ = state.move_task(dragging_id, &column_name);
                            state.dragging = None;
                        }
                    }
                }
            });
    });
}

fn render_task_card(ui: &mut egui::Ui, task: &Task, _column_name: &str) -> egui::Response {
    let frame = Frame::none()
        .fill(Color32::from_rgb(40, 42, 54))
        .rounding(Rounding::same(8.0))
        .stroke(Stroke::new(1.0, Color32::from_rgb(60, 62, 74)))
        .inner_margin(Margin::same(10.0));

    frame
        .show(ui, |ui| {
            ui.set_min_width(280.0);

            // Task ID (short)
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(&task.short_id())
                        .size(10.0)
                        .color(Color32::GRAY),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Tags
                    for tag in &task.tags {
                        ui.label(
                            egui::RichText::new(format!("#{}", tag))
                                .size(10.0)
                                .color(Color32::from_rgb(100, 150, 255)),
                        );
                    }
                });
            });

            // Task title
            ui.label(egui::RichText::new(&task.title).size(14.0));

            // Description if exists
            if let Some(desc) = &task.description {
                ui.add_space(5.0);
                ui.label(
                    egui::RichText::new(desc)
                        .size(11.0)
                        .color(Color32::LIGHT_GRAY),
                );
            }
        })
        .response
        .interact(egui::Sense::click_and_drag())
        .on_hover_cursor(egui::CursorIcon::Grab)
}
