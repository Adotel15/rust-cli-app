
use std::io;
use std::process::Command;

pub struct Task {
    id: u32,
    title: String,
    description: String,
    completed: bool,
    date: String,
}

// pub => public struct


impl Task {

    pub fn new(title: String, description: String, date: String, id: u32) -> Task {
        return Task {
            id,
            title,
            description,
            completed: false,
            date,
        }
    }

    pub fn get_task(&self) -> String {
        return format!("Title: {}\nDescription: {}\nCompleted: {}\nDate: {}", self.title, self.description, self.completed, self.date)
    }

    pub fn get_title(&self) -> &String {
        return &self.title
    }

    pub fn complete_task(&mut self) {
        self.completed = true;
    }

    pub fn is_completed(&self) -> bool {
        return self.completed
    }
}


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
                tasks_by_title.push(task.get_title().to_string());
        }
        return tasks_by_title;
    }
}


fn print_menu() {
    println!("Task Manager");
    println!("1. Add task");
    println!("2. Complete task");
    println!("3. Remove task");
    println!("4. Get completed tasks");
    println!("5. Get incompleted tasks");
    println!("6. Get tasks by title");
    println!("7. Exit");
    println!("Enter your choice: ");
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .unwrap();
    } else {
        Command::new("clear")
            .status()
            .unwrap();
    }
}


fn main() {
    let mut task_manager = TaskManager::new();
    loop {
        print_menu();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice = choice.trim();
        clear_screen();

        match choice {
            "1" => {
                println!("Add task");
                println!("Enter title: ");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                title = title.trim().to_string();

                println!("Enter description: ");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line");
                description = description.trim().to_string();

                println!("Enter date: ");
                let mut date = String::new();
                io::stdin().read_line(&mut date).expect("Failed to read line");
                date = date.trim().to_string();

                let id = task_manager.get_tasks().len() as u32;
                let task = Task::new(title, description, date, id);
                task_manager.add_task(task);
            },
            "2" => {
                println!("Enter task index: ");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read line");
                index = index.trim().to_string();
                let index: usize = index.parse().expect("Please type a number!");

                task_manager.complete_task(index);
            },
            "3" => {
                println!("Enter task index: ");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read line");
                index = index.trim().to_string();
                let index: usize = index.parse().expect("Please type a number!");

                task_manager.remove_task(index);
            },
            "4" => {
                let completed_tasks = task_manager.get_completed_tasks();
                for task in completed_tasks {
                    println!("{}", task.get_task());
                }
            },
            "5" => {
                let incompleted_tasks = task_manager.get_incompleted_tasks();
                for task in incompleted_tasks {
                    println!("{}", task.get_task());
                }
            },
            "6" => {
                let tasks_by_title = task_manager.get_tasks_by_title();
                for task in tasks_by_title {
                    println!("{}", task);
                }
            },
            "7" => {
                break;
            },
            _ => {
                println!("Invalid choice");
            }

        }
        println!("Press enter to continue...");
        let mut enter = String::new();
        io::stdin().read_line(&mut enter).expect("Failed to read line");    
        clear_screen();
    }

}