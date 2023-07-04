use chrono::{DateTime, Local};
use clap::Parser;
use dirs;
use serde_json;
use std::{
    collections::HashSet,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

mod cli;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);

    match args.command {
        Commands::Init { } => {
            init();
        }
        Commands::List { all, complete, categories } => {
            list_tasks(all, complete, categories);
        }
        Commands::Json { pretty } => {
            display_json(pretty);
        }
        Commands::Add { task, categories } => {
            create_task(task, categories);
        }
        Commands::Complete { task_ids } => {
            toggle_completed(task_ids);
        }
        Commands::Delete { task_ids } => {
            delete_tasks(task_ids);
        }
        Commands::Edit { task, description } => {
            edit_task(task, description);
        }
        Commands::Clear {  } => {
            clear_tasks();
        }
        Commands::Get { task } => {
            get_task(task);
        }
        _ => {
            println!("Error: invalid command");
            std::process::exit(1);
        }
    }
}

// get file related functions
fn get_file() -> PathBuf {
    let docs_todo = dirs::document_dir().unwrap().join("todo.json");
    let local_todo = dirs::data_local_dir().unwrap().join("belldo/todo.json");

    if docs_todo.exists() {
        docs_todo
    } else if local_todo.exists() {
        local_todo.join("todo.json")
    } else {
        println!("Error: No todo file found");
        println!("Please run `bellado init` to create the required files");
        std::process::exit(1);
    }
}

fn load_json(path: &PathBuf) -> serde_json::Value {
    let file = File::open(&path).expect("Unable to read file");
    let reader = io::BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to parse JSON")
}

fn save_json(filename: &PathBuf, json: &serde_json::Value) {
    let file = File::create(filename).expect("Failed to create file");
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(writer, json).expect("Failed to write JSON")
}

// set the json file location
fn init() {
    if get_file().exists() {
        println!("Tasks file already exists.");
        return;
    }
    let docs_todo = dirs::document_dir().unwrap().join("todo.json");
    let local_todo = dirs::data_local_dir().unwrap().join("belldo/todo.json");

    println!("Where would you like to save the todo file");
    println!("(1) {}", docs_todo.display());
    println!("(2) {}", local_todo.display());

    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Error: Failed to read line");

    let inp: i32 = inp.trim().parse().expect("Error: Input not an integer");
    match inp {
        1 => create_files(docs_todo),
        2 => create_files(local_todo),
        _ => println!("Error: not an option"),
    }
}

fn create_files(path: PathBuf) {
    let prefix = path.parent().unwrap();
    fs::create_dir_all(prefix).unwrap();
    fs::write(&path, b"{\"tasks\": []}").expect("Error: failed to write to file");
}

fn display_json(pretty: bool) {
    let json_loc = get_file();
    let json = load_json(&json_loc);
    if pretty {
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
    } else {
        println!("{}", json);
    }
}

fn list_tasks(show_all: bool, show_complete: bool, categories: Vec<String>) {
    let json_loc = get_file();
    let json = load_json(&json_loc);
    let tasks = json["tasks"].as_array().unwrap();

    let category_set: HashSet<_> = categories.into_iter().collect();

    for task in tasks {
        let completed = task["completed"].as_bool().unwrap();
        if show_all {
            show_user(task, true, true, true, true, true, true);
            continue;
        }
        if !category_set.is_empty() {
            let task_categories = task["categories"].as_array().unwrap();
            let has_matching_category = task_categories
                .iter()
                .any(|category| category_set.contains(&category.as_str().unwrap().to_string()));

            if has_matching_category {
                if !completed && !show_complete {
                    show_user(task, true, true, true, false, false, false);
                    continue;
                }
                if show_complete {
                    show_user(task, true, true, true, false, true, true);
                }
            }
            continue;
        }
        if !completed && !show_complete {
            show_user(task, true, true, false, false, false, false);
            continue;
        }
        if show_complete {
            show_user(task, true, true, false, false, true, true);
        }
    }
}

fn get_task(id: u64) {
    let json_loc = get_file();
    let json = load_json(&json_loc);
    let tasks = json["tasks"].as_array().unwrap();

    for task in tasks {
        if let Some(task_id) = task.get("id").and_then(serde_json::Value::as_u64) {
            if task_id == id {
                show_user(task, true, true, true, true, true, true);
            }
        }
    }
}

fn show_user(
    inp: &serde_json::Value,
    id: bool,
    text: bool,
    category: bool,
    created_at: bool,
    completed_at: bool,
    completed: bool,
) {
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout);
    if completed {
        if inp["completed"].as_bool() == Some(true) {
            write!(handle, "✓ ").expect("Failed to write to stdout");
        } else {
            write!(handle, "✗ ").expect("Failed to write to stdout");
        }
    }
    if id {
        write!(handle, "{} ", inp["id"]).expect("Failed to write to stdout");
    }
    if text {
        write!(handle, "{} ", inp["text"]).expect("Failed to write to stdout");
    }
    if category {
        let categories = inp["categories"]
            .as_array()
            .map(|categories| {
                categories
                    .iter()
                    .map(|category| category.as_str().unwrap())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_else(|| String::new());
        if !categories.is_empty() {
            write!(handle, "[{}] ", categories).expect("Failed to write to stdout");
        }
    }
    if created_at {
        write!(handle, "{} ", inp["created_at"]).expect("Failed to write to stdout");
    }
    if completed_at {
        write!(handle, "{} ", inp["completed_at"]).expect("Failed to write to stdout");
    }
    handle.write_all(b"\n").unwrap();
    handle.flush().unwrap();
}

fn create_task(inp: String, categories: Vec<String>) {
    let mut json = load_json(&get_file());
    let tasks = json
        .get_mut("tasks")
        .expect("Failed to parse tasks")
        .as_array_mut()
        .expect("Failed to parse tasks");
    let next_id = tasks.last().map_or(0, |task| task["id"].as_u64().unwrap()) + 1;

    let new_task = serde_json::json!({
        "id": next_id,
        "text": inp.to_string(),
        "categories": categories,
        "created_at": get_time(),
        "completed_at": "N/A",
        "completed": false
    });

    tasks.push(new_task);
    save_json(&get_file(), &json);
}

fn toggle_completed(task_ids: Vec<u64>) {
    let mut json = load_json(&get_file());
    let tasks = json
        .get_mut("tasks")
        .expect("Failed to parse tasks")
        .as_array_mut()
        .expect("Failed to parse tasks");

    for task in tasks.iter_mut() {
        if let Some(id) = task.get("id").and_then(serde_json::Value::as_u64) {
            if task_ids.contains(&id) {
                let complete = task["completed"].as_bool().unwrap();
                task["completed"] = serde_json::json!(!complete);
                if !complete {
                    task["completed_at"] = serde_json::json!(get_time());
                } else {
                    task["completed_at"] = serde_json::json!("N/A");
                }
            }
        }
    }

    save_json(&get_file(), &json);
}

fn delete_tasks(task_ids: Vec<u64>) {
    let mut json = load_json(&get_file());
    let tasks = json
        .get_mut("tasks")
        .expect("Failed to parse tasks")
        .as_array_mut()
        .expect("Failed to parse tasks");

    tasks.retain(|task| {
        if let Some(id) = task.get("id").and_then(serde_json::Value::as_u64) {
            !task_ids.contains(&id)
        } else {
            true
        }
    });

    save_json(&get_file(), &json);
}

fn edit_task(task_id: u64, inp: String) {
    let mut json = load_json(&get_file());
    let tasks = json
        .get_mut("tasks")
        .expect("Failed to parse tasks")
        .as_array_mut()
        .expect("Failed to parse tasks");

    for task in tasks.iter_mut() {
        if let Some(id) = task.get("id").and_then(serde_json::Value::as_u64) {
            if id == task_id {
                task["text"] = serde_json::json!(inp);
            }
        }
    }

    save_json(&get_file(), &json);
}

fn clear_tasks() {
    let json = serde_json::json!({"tasks": []});
    save_json(&get_file(), &json);
}

// get time in a provided format
fn get_time() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%H:%M %d/%m/%Y").to_string()
}
