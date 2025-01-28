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

    pub fn add_task(&mut self, title: String) -> usize {
        let id = self.tasks.len() + 1;
        let task = Task {
            id,
            title,
            completed: false,
        };
        self.tasks.push(task);
        id
    }

    pub fn list_tasks(&self) -> Vec<Task> {
        self.tasks.clone()
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

    pub fn clear_all(&mut self) {
        self.tasks.clear();
    }
}