use candid::CandidType;
use serde::{Serialize, Deserialize};
use ic_cdk::query;
use ic_cdk::update;

#[derive(CandidType, Clone, Serialize, Deserialize)]
struct Task {
    id: u64,
    title: String,
    description: String,
    status: TaskStatus,
    due_date: Option<u64>,
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
enum TaskStatus {
    Pending,
    Completed,
}

impl Task {
    fn new(id: u64, title: String, description: String, due_date: Option<u64>) -> Self {
        Task {
            id,
            title,
            description,
            status: TaskStatus::Pending,
            due_date,
        }
    }
}

#[derive(CandidType)]
struct TaskTracker {
    tasks: Vec<Task>,
}

impl TaskTracker {
    fn new() -> Self {
        TaskTracker { tasks: Vec::new() }
    }

    fn add_task(&mut self, title: String, description: String, due_date: Option<u64>) -> Task {
        let id = self.tasks.len() as u64 + 1;
        let task = Task::new(id, title, description, due_date);
        self.tasks.push(task.clone());
        task
    }

    fn update_task(&mut self, id: u64, title: String, description: String, due_date: Option<u64>) -> Option<Task> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.title = title;
            task.description = description;
            task.due_date = due_date;
            Some(task.clone())
        } else {
            None
        }
    }

    fn delete_task(&mut self, id: u64) -> Option<Task> {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }

    fn get_task(&self, id: u64) -> Option<Task> {
        self.tasks.iter().find(|t| t.id == id).cloned()
    }

    fn list_tasks(&self) -> Vec<Task> {
        self.tasks.clone()
    }
}

#[update]
fn add_task(title: String, description: String, due_date: Option<u64>) -> Task {
    TaskTracker::new().add_task(title, description, due_date)
}

#[update]
fn update_task(id: u64, title: String, description: String, due_date: Option<u64>) -> Option<Task> {
    TaskTracker::new().update_task(id, title, description, due_date)
}

#[update]
fn delete_task(id: u64) -> Option<Task> {
    TaskTracker::new().delete_task(id)
}

#[query]
fn get_task(id: u64) -> Option<Task> {
    TaskTracker::new().get_task(id)
}

#[query]
fn list_tasks() -> Vec<Task> {
    TaskTracker::new().list_tasks()
}

// need this to generate candid
ic_cdk::export_candid!();
