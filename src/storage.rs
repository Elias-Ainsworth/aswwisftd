use std::{fs::File, io::BufWriter};

use crate::{TaskError, tasks::Task};

pub fn save_tasks(task_list: &mut Vec<Task>, path: &str) -> Result<(), TaskError> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, task_list)?;
    Ok(())
}

pub fn load_tasks(path: &str) -> Result<Vec<Task>, TaskError> {
    let file = File::open(path)?;
    Ok(serde_json::from_reader(file)?)
}
