use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Debug, Parser)]
#[command(name = "bellado")]
#[command(about, long_about = None, version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create the reqired files
    #[command(arg_required_else_help = false)]
    Init {},
    /// Create a new task
    #[command(arg_required_else_help = true, short_flag = 'a')]
    Add {
        /// The task you wish to create
        task: String,
        #[arg(short = 'c', num_args = 1..)]

        /// The category for the task you wish to create
        #[arg(required = false)]
        categories: Vec<String>,
    },
    /// List out tasks
    #[command(arg_required_else_help = false, short_flag = 'l')]
    List {
        /// Show all tasks
        #[arg(short = 'a')]
        all: bool,

        /// Show completed tasks
        #[arg(short = 'c', conflicts_with = "all")]
        complete: bool,

        /// Show tasks that match the given categories
        #[arg(short = 's', conflicts_with = "all", num_args = 1..)]
        categories: Vec<String>,
    },
    /// Output the JSON file
    #[command(arg_required_else_help = false, short_flag = 'j')]
    Json {
        /// Display the JSON in a pretty format
        #[arg(short = 'p')]
        pretty: bool,
    },
    /// Mark task(s) as completed
    #[command(arg_required_else_help = true, short_flag = 'c')]
    Complete {
        /// Task(s) to mark as completed
        #[arg(required = true)]
        task_ids: Vec<u64>,
    },
    /// Delete task(s)
    #[command(arg_required_else_help = true, short_flag = 'd')]
    Delete {
        /// Task(s) to delete
        #[arg(required = true)]
        task_ids: Vec<u64>,
    },
    /// Edit the description of task
    #[command(arg_required_else_help = true, short_flag = 'e')]
    Edit {
        /// The ID of the task you wish to get
        task: u64,
        /// The new description
        description: String,
    },
    /// Delete all tasks
    #[command(arg_required_else_help = false, short_flag = 'C')]
    Clear {},
    /// Get a task by ID
    #[command(arg_required_else_help = true, short_flag = 'g')]
    Get {
        /// The ID of the task you wish to edit
        task: u64,
    },
    /// Create completion files for bellado
    #[command(arg_required_else_help = true)]
    Completions {
        /// The shell that you wish to make the commands for
        shell: Shell
    },
}
