#[derive(Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Default)]
pub struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, title: &str) -> usize {
        let id = self.tasks.len() + 1;
        self.tasks.push(Task {
            id,
            title: title.to_string(),
            completed: false,
        });
        id
    }

    pub fn list_tasks(&self) -> &[Task] {
        &self.tasks
    }

    pub fn complete_task(&mut self, id: usize) -> Result<(), String> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.completed = true;
            Ok(())
        } else {
            Err(format!("Task with id {} not found.", id))
        }
    }

    pub fn delete_task(&mut self, id: usize) -> Result<(), String> {
        if let Some(index) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(index);
            Ok(())
        } else {
            Err(format!("Task with id {} not found.", id))
        }
    }

    // ✅ Fixed: Add missing `clear_all()` method
    pub fn clear_all(&mut self) {
        self.tasks.clear();
    }
}