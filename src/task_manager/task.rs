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