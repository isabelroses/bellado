use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use std::{fs, path::PathBuf};

use crate::tasks::Store;

pub fn get_datastore_file() -> Result<PathBuf> {
    let docs_todo = dirs::document_dir().context("document dir could not be found")?.join("todo.json");
    let local_todo = dirs::data_local_dir().context("data dir could not be found")?.join(concat!(env!("CARGO_PKG_NAME"), "/todo.json"));

    if docs_todo.exists() {
        Ok(docs_todo)
    } else if local_todo.exists() {
        Ok(local_todo.join("todo.json"))
    } else {
        eprintln!("Error: No todo file found");
        eprintln!("Please run `bellado init` to create the required files");
        std::process::exit(1);
    }
}

pub fn create_files(path: PathBuf) -> Result<()> {
    let prefix = path
        .parent()
        .with_context(|| format!("no parent for path {}", path.display()))?;
    fs::create_dir_all(prefix)?;
    let store = Store { tasks: vec![] };
    fs::write(&path, serde_json::to_string_pretty(&store)?)?;
    Ok(())
}

pub fn get_time_string() -> Result<String> {
    let local: DateTime<Local> = Local::now();
    Ok(local.format("%H:%M %d/%m/%Y").to_string())
}
