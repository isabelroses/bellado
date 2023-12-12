use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Debug, Parser)]
#[command(name = "bellado")]
#[command(about, long_about = None, version, author, arg_required_else_help = true)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create the reqired files
    #[command(arg_required_else_help = false)]
    Init {
        /// Initialize a git repo
        #[arg(short, long)]
        git: bool,
    },
    #[command(arg_required_else_help = true)]
    Import {
        /// Bring your own todo git repo
        #[arg(required = true)]
        users_repo: String,
    },
    #[command(arg_required_else_help = true, short_flag = 'g')]
    Git {
        /// Initialize a git repo
        #[arg(short, long, conflicts_with = "push")]
        init: bool,
        /// Push the changes
        #[arg(short, long, conflicts_with = "pull")]
        push: bool,
        ///Pull the changes
        #[arg(short = 'P', long, conflicts_with = "push")]
        pull: bool,
    },
    /// Create a new task
    #[command(arg_required_else_help = true, short_flag = 'a')]
    Add {
        /// The task you wish to create
        #[arg(required = true)]
        task: String,

        /// The category for the task you wish to create
        #[arg(required = false, short, num_args = 1..)]
        categories: Vec<String>,
    },
    /// List out tasks
    #[command(arg_required_else_help = false, short_flag = 'l')]
    List {
        /// Show all tasks
        #[arg(short, long)]
        all: bool,

        /// Show completed tasks
        #[arg(short, conflicts_with = "all")]
        complete: bool,

        /// Show the table header
        #[arg(long)]
        header: bool,

        /// Format the output as a table
        #[arg(long = "table", short = 't')]
        as_table: bool,

        /// Show tasks that match the given categories
        #[arg(short = 's', conflicts_with = "all", num_args = 1..)]
        categories: Vec<String>,
    },
    /// Export the JSON file, in diffrent formats
    #[command(arg_required_else_help = false)]
    Export {
        /// Display the JSON in a pretty format
        #[arg(short, long, group = "json_group", conflicts_with = "markdown")]
        json: bool,

        /// Display the JSON in a pretty format
        #[arg(short, group = "json_group", conflicts_with = "markdown")]
        pretty: bool,

        /// Display the JSON in a pretty format
        #[arg(
            short,
            long,
            conflicts_with = "json",
            conflicts_with = "pretty",
            group = "markdown_group"
        )]
        markdown: bool,
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
    #[command(arg_required_else_help = true, short_flag = 'G')]
    Get {
        /// The ID of the task you wish to edit
        #[arg(required = true)]
        task: u64,

        /// Format the output as a table
        #[arg(short = 't', long = "table")]
        as_table: bool,

        /// Show the table header
        #[arg(long)]
        header: bool,
    },
    /// Create completion files for bellado
    #[command(arg_required_else_help = true)]
    Completions {
        /// The shell that you wish to make the commands for
        shell: Shell,
    },
}
