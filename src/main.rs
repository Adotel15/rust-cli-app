
mod task;
mod task_manager;

use task::Task;
use task_manager::TaskManager;
use std::io;
use std::process::Command;

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
        choice = choice.trim();

        match(choice) {
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