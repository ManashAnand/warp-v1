use dialoguer::{theme::ColorfulTheme, Input, Select};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub title: String,
    pub completed: bool,
}

pub fn load_tasks() -> Vec<Task> {
    let content = fs::read_to_string("tasks.json").unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&content).unwrap_or_else(|_| vec![])
}

pub fn save_tasks(tasks: &[Task]) -> Result<(), String> {
    let json = serde_json::to_string_pretty(tasks)
        .map_err(|e| format!("Failed to serialize tasks: {}", e))?;
    fs::write("tasks.json", json).map_err(|e| format!("Failed to write file: {}", e))?;
    Ok(())
}

pub fn add_task(tasks: &mut Vec<Task>) {
    let title = Input::new()
        .with_prompt("Enter task title")
        .interact_text()
        .unwrap();
    tasks.push(Task {
        title,
        completed: false,
    });

    if let Err(e) = save_tasks(tasks) {
        eprintln!("Error saving task: {}", e);
    }
    println!("");
}

pub fn toggle_complition(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks to toggle.");
        return;
    }

    let items: Vec<String> = tasks
        .iter()
        .enumerate()
        .map(|(i, task)| {
            format!(
                "{}. {} {}",
                i + 1,
                if task.completed { "✅" } else { "⬜" },
                task.title
            )
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select task to toogle completion")
        .items(&items)
        .interact()
        .unwrap();

    tasks[selection].completed = !tasks[selection].completed;

    if let Err(e) = save_tasks(tasks) {
        eprintln!("Failed to save after toggle: {}", e);
    } else {
        println!("✅ Task updated!");
    }
    println!("");
}

pub fn delete_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks to toggle.");
        return;
    }

    let items: Vec<String> = tasks
        .iter()
        .enumerate()
        .map(|(i, item)| {
            format!(
                "
    {} {} {}
        ",
                i + 1,
                item.completed,
                item.title
            )
        })
        .collect();

    let selections = Select::new()
        .items(&items)
        .default(0)
        .with_prompt("Choose the task to delete")
        .interact()
        .unwrap();

    tasks.remove(selections);
    if let Err(e) = save_tasks(tasks) {
        eprintln!("Failed to save after toggle: {}", e);
    } else {
        println!("✅ Task updated!");
    }
    println!("");
}

pub fn show_tasks() {
    println!("\n==== TODO LIST ====");

    let prev_task = load_tasks();

    if prev_task.is_empty() {
        println!("(No tasks yet)");
        return;
    }

    for (i, task) in prev_task.iter().enumerate() {
        println!(
            "{}. [{}] {}",
            i + 1,
            if task.completed { "✅" } else { "⬜" },
            task.title
        );
    }
    println!("");
}
