use aswwisftd::{
    Cli, Command, TaskError, add_task, delete_task, edit_task, list_tasks, load_tasks, print_error,
    save_tasks,
};
use clap::Parser;
use std::path::Path;

fn main() {
    if let Err(e) = run() {
        print_error(&e);
    }
}

fn run() -> Result<(), TaskError> {
    let cli = Cli::parse();
    let path = "tasks.json";
    let mut task_list = if Path::new(path).exists() {
        load_tasks(path)?
    } else {
        Vec::new()
    };

    match cli.command {
        Command::Add { description } => {
            add_task(&mut task_list, description)?;
            save_tasks(&mut task_list, path)?
        }
        Command::List => list_tasks(&mut task_list)?,
        Command::Edit { id } => {
            edit_task(&mut task_list, id)?;
            save_tasks(&mut task_list, path)?
        }
        Command::Delete { id } => {
            delete_task(&mut task_list, id)?;
            save_tasks(&mut task_list, path)?
        }
    }
    Ok(())
}
