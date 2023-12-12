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
        #[arg(short, long, conflicts_with = "push", conflicts_with = "pull")]
        init: bool,
        /// Push the changes
        #[arg(short, long, conflicts_with = "pull", conflicts_with = "init")]
        push: bool,
        ///Pull the changes
        #[arg(short = 'P', long, conflicts_with = "push", conflicts_with = "init")]
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
    #[command(arg_required_else_help = false, long_flag = "export", short_flag = 'x')]
    Export {
        /// Export as JSON
        #[arg(
            short,
            long,
            conflicts_with = "markdown",
            conflicts_with = "with_categories"
        )]
        json: bool,

        /// Export as JSON in a pretty format
        #[arg(short, conflicts_with = "markdown", conflicts_with = "with_categories")]
        pretty: bool,

        /// Export as Markdown
        #[arg(short, long, conflicts_with = "json", conflicts_with = "pretty")]
        markdown: bool,

        /// Export as Markdown with categories
        #[arg(
            short = 'c',
            long = "categories",
            conflicts_with = "json",
            conflicts_with = "pretty"
        )]
        with_categories: bool,
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
