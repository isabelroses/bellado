use anyhow::{Context, Ok, Result};
use chrono::{DateTime, Local};
use dirs;
use std::{fs, path::PathBuf};

pub fn get_datastore_file() -> Result<PathBuf> {
    let todo = dirs::data_local_dir()
        .context("data dir could not be found")?
        .join(concat!(env!("CARGO_PKG_NAME"), "/todo.json"));

    if !check_file().is_err() {
        Ok(todo)
    } else {
        eprintln!("Error: No todo file found");
        eprintln!("Please run `bellado init` to create the required files");
        std::process::exit(1);
    }
}

pub fn check_file() -> Result<()> {
    let todo = dirs::data_local_dir()
        .context("data dir could not be found")?
        .join(concat!(env!("CARGO_PKG_NAME"), "/todo.json"));

    if todo.exists() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("No todo file found"))
    }
}

pub fn get_datastore_dir() -> Result<PathBuf> {
    let local_todo = dirs::data_local_dir()
        .context("data dir could not be found")?
        .join(env!("CARGO_PKG_NAME"));

    if local_todo.exists() {
        Ok(local_todo)
    } else {
        eprintln!("Error: No todo file found");
        eprintln!("Please run `bellado init` to create the required files");
        std::process::exit(1);
    }
}

pub fn create_files() -> Result<()> {
    let path = dirs::data_local_dir()
        .context("data dir could not be found")?
        .join(concat!(env!("CARGO_PKG_NAME"), "/todo.json"));

    let prefix = path
        .parent()
        .with_context(|| format!("no parent for path {}", path.display()))?;

    fs::create_dir_all(prefix)?;
    fs::write(&path, "[]")?;
    Ok(())
}

pub fn get_time_string() -> Result<String> {
    let local: DateTime<Local> = Local::now();
    Ok(local.format("%H:%M %d/%m/%Y").to_string())
}
