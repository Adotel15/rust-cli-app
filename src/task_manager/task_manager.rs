
// mod => importar modulos (código de otros lados)
// use para usar con el nombre task la clase Task
use::task;

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        return TaskManager {
            tasks: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        return &self.tasks; 
    }

    pub fn get_task(&self, index: usize) -> &Task {
        return &self.tasks[index];
    }

    pub fn complete_task(&mut self, index: usize) {
        self.tasks[index].complete_task();
    }

    pub fn remove_task(&mut self, index: usize) {
        self.tasks.remove(index);
    }

    pub fn get_completed_tasks(&self) -> Vec<&Task> {
        let mut completed_tasks: Vec<&Task> = Vec::new();
        for task in &self.tasks {
            if task.is_completed() {
                completed_tasks.push(task);
            }
        }
        return completed_tasks;
    }

    pub fn get_incompleted_tasks(&self) -> Vec<&Task> {
        let mut incompleted_tasks: Vec<&Task> = Vec::new();
        for task in &self.tasks {
            if !task.is_completed() {
                incompleted_tasks.push(task);
            }
        }
        return incompleted_tasks;
    }

    pub fn get_tasks_by_title(&self) -> Vec<String> {
        let mut tasks_by_title: Vec<String> = Vec::new();
        for task in &self.tasks {
                tasks_by_title.push(task.get_title());
        }
        return tasks_by_title;
    }
}
