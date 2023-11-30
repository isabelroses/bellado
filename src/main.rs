use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use prettytable::{format, row, Cell, Row, Table};
use serde_json;

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
                header,
                categories,
            } => {
                let tasks = tasks::get_all(all, complete, categories)?;
                display_tasks(tasks, header)?;
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
                    let mut table = Table::new();
                    table.set_format(format::FormatBuilder::new().padding(1, 1).build());
                    add_task(task, &mut table)?;
                    table.printstd();
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

fn display_tasks(tasks: Vec<tasks::Task>, header: bool) -> Result<()> {
    let mut table = Table::new();

    table.set_format(format::FormatBuilder::new().padding(1, 1).build());

    if header {
        table.set_titles(row![
            "ID",
            "Task",
            "Categories",
            "Created At",
            "Completed At",
            "Completed"
        ]);
    }

    for task in tasks {
        add_task(task, &mut table)?;
    }

    // Print the table to stdout
    table.printstd();

    Ok(())
}

fn add_task(task: tasks::Task, table: &mut Table) -> Result<()> {
    let categories = task.categories.join(", ");
    table.add_row(Row::new(vec![
        Cell::new(&task.id.to_string()),
        Cell::new(&task.text),
        Cell::new(&categories),
        Cell::new(&task.created_at),
        Cell::new(&task.completed_at.unwrap_or_else(|| "".to_string())),
        Cell::new(if task.completed { "✓" } else { "✗" }),
    ]));

    Ok(())
}
