use candid::CandidType;
use serde::{Serialize, Deserialize};
use ic_cdk::update;
use ic_cdk::query;

// Define Task and TaskStatus structs
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

 // Define a constructor for Task struct
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

// Define TaskTracker struct
#[derive(CandidType)]
struct TaskTracker {
    tasks: Vec<Task>,
}

impl TaskTracker {
    // Define a constructor for TaskTracker struct
    fn new() -> Self {
        TaskTracker { tasks: Vec::new() }
    }

    // Add a new task to the TaskTracker
    fn add_task(&mut self, title: String, description: String, due_date: Option<u64>) -> Task {
        let id = self.tasks.len() as u64 + 1;
        let task = Task::new(id, title, description, due_date);
        self.tasks.push(task.clone());
        task
    }
    
    // Update an existing task in the TaskTracker
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

    
    // Delete a task from the TaskTracker
    fn delete_task(&mut self, id: u64) -> Option<Task> {
        if let Some(index) = self.tasks.iter().position(|t| t.id == id) {
            Some(self.tasks.remove(index))
        } else {
            None
        }
    }

    fn get_task(&self, id: u64) -> Option<Task> {    // Change return type to Option<&Task>
        self.tasks.iter().find(|t| t.id == id).cloned()
    }

    fn list_tasks(&self) -> Vec<Task> { // Change return type to &[Task]
        self.tasks.clone()
    }
}

static mut TASK_TRACKER: Option<TaskTracker> = None;

fn get_or_init_task_tracker() -> &'static mut TaskTracker {
    unsafe {
        TASK_TRACKER.get_or_insert_with(|| TaskTracker::new())
    }
}

#[update(name = "add_task")]
fn add_task(title: String, description: String, due_date: Option<u64>) -> Task {
    get_or_init_task_tracker().add_task(title, description, due_date)
}

#[update(name = "update_task")]
fn update_task(id: u64, title: String, description: String, due_date: Option<u64>) -> Option<Task> {
    get_or_init_task_tracker().update_task(id, title, description, due_date)
}

#[update(name = "delete_task")]
fn delete_task(id: u64) -> Option<Task> {
    get_or_init_task_tracker().delete_task(id)
}

#[query(name = "get_task")]
fn get_task(id: u64) -> Option<Task> {
    get_or_init_task_tracker().get_task(id)
}

#[query(name = "list_tasks")]
fn list_tasks() -> Vec<Task> {
    get_or_init_task_tracker().list_tasks()
}

// need this to generate candid
ic_cdk::export_candid!();

