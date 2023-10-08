use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use serde_json;
use std::io::{stdout, BufWriter, Write};

mod cli;
mod git;
mod io;
mod tasks;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();
    //println!("{:?}", args);

    fn handle_command(args: Cli) -> Result<()> {
        match args.command {
            Commands::Init { git_bk } => {
                io::create_files()?;

                if git_bk {
                    git::init();
                }
            }
            Commands::Import { users_repo } => {
                git::clone_repo(users_repo);
            }
            Commands::Git {
                git_init,
                git_push,
                git_pull,
            } => {
                if git_init {
                    git::init();
                }
                if git_push {
                    git::push();
                }
                if git_pull {
                    git::pull();
                }
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
            Commands::Completions { shell } => {
                // Generate the completions and exit immediately
                let mut cmd = Cli::command();
                let name = cmd.get_name().to_string();
                eprintln!("Generating completions for {shell}");
                generate(shell, &mut cmd, name, &mut std::io::stdout());
                std::process::exit(0);
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
        write!(
            display_handle,
            "{} ",
            task.completed_at
                .context("completed_at was not set but completed was true")?
        )?;
    }

    display_handle.write_all(b"\n")?;
    display_handle.flush()?;

    Ok(())
}
