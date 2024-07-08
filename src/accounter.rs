use super::task::{SaveData, Task};
use serde_json;

pub struct Coordinator {
    tasks: Vec<Task>,
}

impl Coordinator {
    pub fn new() -> Coordinator {
        Coordinator {
            tasks: Vec::<Task>::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn del_task(&mut self, position: usize) {
        self.tasks.remove(position);
        // self.tasks.retain(|item| item != task);
    }

    pub fn get_position(&self, task: &Task) -> Option<usize> {
        self.tasks.iter().position(|x| x == task)
    }

    pub fn get_tasks(&self) -> Vec<&Task> {
        let mut tasks: Vec<&Task> = self.tasks.iter().collect();
        tasks.sort_by(|a, b| b.due.cmp(&a.due));
        return tasks;
    }

    pub fn serialize(&self) -> String {
        let save_data: Vec<SaveData> = self.tasks.iter().map(|t| t.to_savedata()).collect();
        serde_json::to_string(&save_data).unwrap()
    }

    pub fn deserialzie(&mut self, json: &str) {
        let save_data: Vec<SaveData> = serde_json::from_str(json).unwrap();
        self.tasks.clear();
        self.tasks.extend(save_data.iter().map(|s| s.to_task()));
    }
}
