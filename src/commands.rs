use std::io::{self, Write};

use crossterm::style::Stylize;
use rand::Rng;

use crate::{State, Task, TaskError, success};

fn get_user_input<T: std::fmt::Display>(prompt: T) -> Result<String, TaskError> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_lowercase().to_string())
}

pub fn add_task(task_list: &mut Vec<Task>, desc: String) -> Result<(), TaskError> {
    let new_task = Task {
        id: rand::rng().random_range(0..1000),
        description: desc,
        state: State::Pending,
    };

    success(&[&"Task id:", &new_task.id])?;

    task_list.push(new_task);

    Ok(())
}

pub fn list_tasks(task_list: &mut [Task]) -> Result<(), TaskError> {
    for task in task_list {
        let state_style = match task.state {
            State::Completed => "Completed".green(),
            State::Pending => "Pending".yellow(),
            State::Incomplete => "Incomplete".red(),
        };

        println!(
            "{:04} {} [{}]",
            task.id.to_string().blue(),
            task.description.to_string().white(),
            state_style,
        )
    }
    Ok(())
}

pub fn edit_task(task_list: &mut [Task], id: u32) -> Result<(), TaskError> {
    if let Some(task) = task_list.iter_mut().find(|x| x.id == id) {
        loop {
            match get_user_input("Enter desired state: ".blue())?.as_str() {
                "completed" => {
                    task.mark_completed();
                    break;
                }
                "pending" => {
                    task.mark_pending();
                    break;
                }
                "incomplete" => {
                    task.mark_incomplete();
                    break;
                }
                _ => println!("Unknown state."),
            }
        }
        if get_user_input("Edit description? ".blue())?.as_str() == "y" {
            task.description = get_user_input("Enter new description: ".blue())?;
        }
        success(&[&"Edited task."])?;
    } else {
        return Err(TaskError::TaskNotFound(id));
    }
    Ok(())
}

pub fn delete_task(task_list: &mut Vec<Task>, id: u32) -> Result<Task, TaskError> {
    match task_list.iter_mut().position(|x| x.id == id) {
        Some(pos) => {
            success(&[&"Deleted task:", &task_list[pos].id])?;
            Ok(task_list.remove(pos))
        }
        None => Err(TaskError::TaskNotFound(id)),
    }
}
