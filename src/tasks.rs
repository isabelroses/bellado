use serde_json;
use std::{collections::HashSet, fs::File, io::BufReader, path::Path};

use crate::io;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u64,
    pub text: String,
    pub categories: Vec<String>,
    pub created_at: String,
    pub completed_at: Option<String>,
    pub completed: bool,
}

pub fn load(path: &Path) -> Result<Vec<Task>> {
    let file = File::open(&path).context("fs failed to read the file")?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).context("serde_json failed to read from the file")
}

pub fn save(filename: &Path, store: Vec<Task>) -> Result<()> {
    let file = File::create(filename)
        .with_context(|| format!("fs failed to create file {}", filename.display()))?;
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &store).context("serde_json failed to write to the file")
}

pub fn get(id: u64) -> Result<Option<Task>> {
    let store = load(&io::get_datastore_file()?)?;

    let task = store
        .iter()
        .filter(|t| t.id == id)
        .collect::<Vec<&Task>>()
        .first()
        .cloned()
        .cloned();

    Ok(task)
}

pub fn get_all(show_all: bool, show_complete: bool, categories: Vec<String>) -> Result<Vec<Task>> {
    let store = load(&io::get_datastore_file()?)?;

    if show_all {
        return Ok(store.clone());
    }

    if show_complete {
        return Ok(store
            .iter()
            .filter(|t| t.completed)
            .map(|t| t.clone())
            .collect());
    }

    if categories.is_empty() {
        return Ok(store
            .iter()
            .filter(|t| !t.completed)
            .map(|t| t.clone())
            .collect());
    }

    let category_set: HashSet<String> = categories.into_iter().collect();

    let filtered_tasks = store
        .iter()
        .filter(|t| t.categories.iter().any(|c| category_set.contains(c)))
        .map(|t| t.clone())
        .collect();

    Ok(filtered_tasks)
}

pub fn create(text: String, categories: Vec<String>) -> Result<()> {
    let mut store = load(&io::get_datastore_file()?)?;
    store.sort_by(|a, b| a.id.cmp(&b.id));
    let next_id = store.last().map_or(0, |task| task.id) + 1;

    let task: Task = Task {
        id: next_id,
        text,
        categories,
        created_at: io::get_time_string()?,
        completed_at: None,
        completed: false,
    };

    store.push(task);

    save(&io::get_datastore_file()?, store)
}

pub fn edit(task_id: u64, inp: String) -> Result<Option<Task>> {
    let mut store = load(&io::get_datastore_file()?)?;
    let task = store.iter_mut().find(|t| t.id == task_id);

    if let Some(task) = task {
        task.text = inp;

        let task = task.clone();
        save(&io::get_datastore_file()?, store)?;

        Ok(Some(task))
    } else {
        Ok(None)
    }
}

pub fn reset() -> Result<()> {
    save(&io::get_datastore_file()?, vec![])
}

pub fn delete(task_ids: Vec<u64>) -> Result<()> {
    let mut store = load(&io::get_datastore_file()?)?;

    store.retain(|task: &Task| !task_ids.contains(&task.id));

    save(&io::get_datastore_file()?, store)
}

pub fn toggle_completion(id: u64) -> Result<()> {
    let mut store = load(&io::get_datastore_file()?)?;
    let task = store.iter_mut().find(|t| t.id == id);

    if let Some(task) = task {
        if task.completed {
            task.completed_at = None;
        } else {
            task.completed_at = Some(io::get_time_string()?);
        }
        task.completed = !task.completed;
        save(&io::get_datastore_file()?, store)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_show_all() {
        let tasks = vec![
            Task {
                id: 1,
                text: "Task 1".to_string(),
                categories: vec!["Category 1".to_string()],
                created_at: "2022-01-01".to_string(),
                completed_at: None,
                completed: false,
            },
            Task {
                id: 2,
                text: "Task 2".to_string(),
                categories: vec!["Category 2".to_string()],
                created_at: "2022-01-02".to_string(),
                completed_at: None,
                completed: false,
            },
        ];

        assert_eq!(
            get_all(false, false, vec![]).unwrap(),
            tasks.clone(),
            "get_all should return incomplete tasks when no categories are specified"
        );
    }
}
