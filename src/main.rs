use anyhow::{Context, Result};
use clap::Parser;
use dirs;
use serde_json;
use std::io::{stdin, stdout, BufWriter, Write};

mod cli;
mod io;
mod tasks;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);

    fn handle_command(args: Cli) -> Result<()> {
        match args.command {
            Commands::Init {} => {
                let datastore_file = io::get_datastore_file()?;
                if datastore_file.exists() {
                    eprint!("Data file {} already exists", datastore_file.display());
                    return Ok(());
                }

                let docs_todo = dirs::document_dir()
                    .context("document dir could not be found")?
                    .join("todo.json");

                let local_todo = dirs::data_local_dir()
                    .context("data dir could not be found")?
                    .join("belldo/todo.json");

                println!("Where would you like to save the todo file");
                println!("(1) {}", docs_todo.display());
                println!("(2) {}", local_todo.display());

                let mut inp = String::new();
                stdin()
                    .read_line(&mut inp)
                    .context("could not read line from stdin")?;

                let location_choice = inp
                    .trim()
                    .parse::<i32>()
                    .context("input was not a number")?;

                match location_choice {
                    1 => io::create_files(docs_todo)?,
                    2 => io::create_files(local_todo)?,
                    _ => println!("Error: not an option"),
                };
            }
            Commands::List {
                all,
                complete,
                categories,
            } => {
                let tasks = tasks::get_all(all, complete, categories)?;
                for task in tasks {
                    display_task(task)?;
                }
            }
            Commands::Json { pretty } => {
                let store = &tasks::load(&io::get_datastore_file()?)?;
                if pretty {
                    println!("{}", serde_json::to_string_pretty(store)?);
                } else {
                    println!("{}", serde_json::to_string(store)?);
                }
            }
            Commands::Add { task, categories } => {
                tasks::create(task, categories)?;
            }
            Commands::Complete { task_ids } => {
                for task in task_ids {
                    tasks::toggle_completion(task)?;
                }
            }
            Commands::Delete { task_ids } => {
                tasks::delete(task_ids)?;
            }
            Commands::Edit { task, description } => {
                tasks::edit(task, description)?;
            }
            Commands::Clear {} => {
                tasks::reset()?;
            }
            Commands::Get { task } => {
                let fetched_task = tasks::get(task)?;
                if let Some(task) = fetched_task {
                    display_task(task)?;
                } else {
                    println!("No task found with ID {task}");
                }
            }
        }

        Ok(())
    }

    let result = handle_command(args);

    if let Err(error) = result {
        eprint!("Error running commands: {:?}", error);
    }
}

pub fn init() -> Result<()> {
    Ok(())
}

fn display_task(task: tasks::Task) -> Result<()> {
    let stdout = stdout();
    let mut display_handle = BufWriter::new(stdout);

    if task.completed {
        write!(display_handle, "✓ ")?;
    } else {
        write!(display_handle, "✗ ")?;
    }
    write!(display_handle, "{} ", task.id)?;
    write!(display_handle, "{} ", task.text)?;
    write!(display_handle, "{} ", task.categories.join(", "))?;
    write!(display_handle, "{} ", task.created_at)?;
    if task.completed {
        write!(display_handle, "{} ", task.completed_at.context("completed_at was not set but completed was true")?)?;
    }

    display_handle.write_all(b"\n")?;
    display_handle.flush()?;

    Ok(())
}
