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
            id: id,
            title: title,
            description: description,
            completed: completed: false,
            date: date,
        }
    }

    pub fn get_task(self: &Task) -> String {
        return format!("Title: {}\nDescription: {}\nCompleted: {}\nDate: {}", self.title, self.description, self.completed, self.date)
    }

    pub fn get_title(&self: &Task) -> &String {
        return &self.title
    }

    pub fn complete_task(&mut self: &Task) {
        self.completed = true;
    }

    pub fn is_completed(&self: &Task) -> bool {
        return self.completed
    }
}