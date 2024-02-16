use candid::CandidType;
use serde::{Serialize, Deserialize};
use ic_cdk::update;
use ic_cdk::query;
use std::collections::HashMap;

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
    tasks: HashMap<u64, Task>,
    next_id: u64,
}

impl TaskTracker {
    fn new() -> Self {
        TaskTracker {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_task(&mut self, title: String, description: String, due_date: Option<u64>) -> Task {
        if title.is_empty() {
            return Err(Error::ValidationError("Title cannot be empty".to_string()));
        }
        if description.is_empty() {
            return Err(Error::ValidationError("Description cannot be empty".to_string()));
        }
        if due_date.is_empty() {
            return Err(Error::ValidationError("Due date cannot be empty".to_string()));
        }
        
        let id = self.tasks.len() as u64 + 1;
        let task = Task::new(id, title, description, due_date);
        self.tasks.insert(task.id, task.clone());
        self.next_id += 1;
        task
    }

    fn update_task(&mut self, id: u64, title: String, description: String, due_date: Option<u64>) -> Option<Task> {
        if title.is_empty() {
            return Err(Error::ValidationError("Title cannot be empty".to_string()));
        }
        if description.is_empty() {
            return Err(Error::ValidationError("Description cannot be empty".to_string()));
        }
        if due_date.is_empty() {
            return Err(Error::ValidationError("Due date cannot be empty".to_string()));
        }
        
        if let Some(task) = self.tasks.get_mut(&id) {
            task.title = title;
            task.description = description;
            task.due_date = due_date;
            Some(self.tasks.insert(id, task.clone()))
        } else {
            None
        }
    }

    fn delete_task(&mut self, id: u64) -> Option<Task> {
        if let Some(index) = self.tasks.get_mut(&id) {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }

    fn get_task(&self, id: u64) -> Option<Task> {
        self.tasks.get_mut(&id).cloned()
    }

    fn list_tasks(&self) -> Vec<Task> {
        self.tasks.clone()
    }

    // Example method for updating task status
    fn update_task_status(&mut self, id: u64, status: TaskStatus) -> Option<&Task> {
        self.tasks.get_mut(&id).map(|task| {
            task.status = status;
            task
        })
    }
}

static TASK_TRACKER: once_cell::sync::Lazy<std::sync::Mutex<TaskTracker>> =
    once_cell::sync::Lazy::new(|| std::sync::Mutex::new(TaskTracker::new()));

#[update(name = "add_task")]
fn add_task(title: String, description: String, due_date: Option<u64>) -> Task {
    TASK_TRACKER.lock().unwrap().add_task(title, description, due_date)
}

#[update(name = "update_task")]
fn update_task(id: u64, title: String, description: String, due_date: Option<u64>) -> Option<Task> {
    TASK_TRACKER.lock().unwrap().update_task(id, title, description, due_date)
}

#[update(name = "delete_task")]
fn delete_task(id: u64) -> Option<Task> {
    TASK_TRACKER.lock().unwrap().delete_task(id)
}

#[query(name = "get_task")]
fn get_task(id: u64) -> Option<Task> {
    TASK_TRACKER.lock().unwrap().get_task(id)
}

#[query(name = "list_tasks")]
fn list_tasks() -> Vec<Task> {
    TASK_TRACKER.lock().unwrap().list_tasks()
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    ValidationError(String),
}

// need this to generate candid
ic_cdk::export_candid!();

