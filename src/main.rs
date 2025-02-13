mod todo;

use eframe::egui;
use todo::TodoList;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "To-Do List",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    todo_list: TodoList,
    new_task: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            todo_list: TodoList::new(),
            new_task: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("To-Do List");

            // Input new task
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_task);
                if ui.button("Add Task").clicked() {
                    if !self.new_task.trim().is_empty() {
                        self.todo_list.add_task(self.new_task.as_str());
                        self.new_task.clear();
                    }
                }
            });

            ui.separator();

            // Collect tasks to avoid immutable & mutable borrow conflict
            let tasks: Vec<_> = self.todo_list.list_tasks().to_vec();

            // Display tasks
            for task in tasks {
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{} [{}]",
                        task.title,
                        if task.completed {
                            "Completed"
                        } else {
                            "Pending"
                        }
                    ));
                    if ui.button("Complete").clicked() {
                        if let Err(err) = self.todo_list.complete_task(task.id) {
                            eprintln!("{}", err);
                        }
                    }
                    if ui.button("Delete").clicked() {
                        if let Err(err) = self.todo_list.delete_task(task.id) {
                            eprintln!("{}", err);
                        }
                    }
                });
            }

            ui.separator();

            // Clear all tasks
            if ui.button("Clear All").clicked() {
                self.todo_list.clear_all();
            }
        });
    }
}