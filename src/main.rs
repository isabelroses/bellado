use clap::{Command, Arg, ArgAction};
use dirs;
use serde_json;
use std::path::PathBuf;
use std::fs;
use std::io::prelude::*;
use chrono::{Local, DateTime};

fn main() {
    let _file = get_file();
    let matches = Command::new("bellado")
        .version("0.1.0")
        .name("bellado")
        .arg_required_else_help(true)
        .about("A cli todo tool")
        .author("isabel roses")
        .subcommand(
            Command::new("init").about("Create the reqired files")
        )
        .subcommand(
            Command::new("new")
                .long_flag("new")
                .short_flag('n')
                .about("Create a new task")
                .arg(
                    Arg::new("task")
                        .action(ArgAction::Set)
                        .num_args(1)
                        .required(true)
                        .help("Enter the task which you wish to create"),
                )
                .arg(
                    Arg::new("categories")
                        .long("categories")
                        .short('c')
                        .action(ArgAction::Set)
                        .num_args(1..)
                        .help("Enter the task which you wish to create"),
                ),
        )
        .subcommand(
            Command::new("list")
                .long_flag("list")
                .short_flag('l')
                .about("List out tasks")
                .arg(
                    Arg::new("all")
                        .short('a')
                        .long("all")
                        .action(ArgAction::SetTrue)
                        .help("Show all tasks"),
                )
                .arg(
                    Arg::new("show_complete")
                        .short('c')
                        .long("complete")
                        .action(ArgAction::SetTrue)
                        .conflicts_with("all")
                        .help("Show completed tasks"),
                ),
        )
        .subcommand(
            Command::new("completed")
            .long_flag("completed")
            .short_flag('c')
            .about("Mark task(s) as completed")
            .arg(
                Arg::new("task_id")
                .action(ArgAction::Set)
                .num_args(1..)
                .required(true)
                .help("Task(s) to mark as compled"),
            ),
        )
        .subcommand(
            Command::new("uncomplete")
            .long_flag("uncomplete")
            .short_flag('u')
            .about("Mark task(s) as uncompleted")
            .arg(
                Arg::new("task_id")
                .action(ArgAction::Set)
                .num_args(1..)
                .required(true)
                .help("Task(s) to mark as uncompled"),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            choose_file_loc();
        }
        Some(("list", list_match)) => {
            let showall = list_match.get_flag("all");
            let show_complete = list_match.get_flag("show_complete");
            list_tasks(showall, show_complete);
        }
        Some(("new", task_match)) => {
            let task = task_match
                .get_one::<String>("task")
                .expect("Failed to read input")
                .as_str();
            let categories = task_match
                .get_many::<String>("categories")
                .expect("is present")
                .map(|s| s.as_str())
                .collect::<Vec<_>>();
            create_task(task, categories);
        }
        Some(("completed", completed_match)) => {
            let task_ids = completed_match
                .get_many::<String>("task_id")
                .expect("is present")
                .map(|s| s.as_str())
                .collect::<Vec<_>>();
            toggle_completed(task_ids, true);
        }
        Some(("uncomplete", uncomplete_match)) => {
            let task_ids = uncomplete_match
                .get_many::<String>("task_id")
                .expect("is present")
                .map(|s| s.as_str())
                .collect::<Vec<_>>();
            toggle_completed(task_ids, false);
        }

        _ => unreachable!(),
    }
}

fn get_file() -> PathBuf {
    let docs_todo = dirs::document_dir().unwrap().as_path().join("todo.json");
    let local_todo = dirs::data_local_dir().unwrap().as_path().join("belldo/todo.json");
    loop {
        if docs_todo.exists() {
            return docs_todo;
        } else if local_todo.exists() {
            return local_todo.join("todo.json");
        } else {
            choose_file_loc();
        }
    }
}

fn choose_file_loc() {
    let docs_todo = dirs::document_dir().unwrap().as_path().join("todo.json");
    let local_todo = dirs::data_local_dir().unwrap().as_path().join("belldo/todo.json");

    let mut inp = String::new();
    println!("Where would you like to save the todo file");
    println!("(1) {}", docs_todo.display());
    println!("(2) {}", local_todo.display());
    std::io::stdin().read_line(&mut inp).expect("Error: Failed to read line");
    let inp: i32 = inp.trim().parse().expect("Error: Input not an integer");
    if inp == 1 {
        create_files(docs_todo);
    } else if inp == 2 {
        create_files(local_todo);
    } else {
        println!("Error: not an option");
    }
}

fn create_files(path: PathBuf) {
    let prefix = path.parent().unwrap();
    fs::create_dir_all(prefix).unwrap();
    let mut file = fs::File::create(path).expect("Error: failed to create file");
    file.write_all(b"{\"tasks\": []}").expect("Error: failed to write to file");
}

fn list_tasks(showall: bool, show_complete: bool) {
    let json_loc = get_file();
    let json = loadjson(json_loc);
    let tasks = json.get("tasks").unwrap().as_array().unwrap();
    for task in tasks {
        if showall {
            println!("{}", show_user(task, true, true, true, true, true, true));
        } else if !task["completed"].as_bool().unwrap() && !show_complete {
            println!("{}", show_user(task, true, true, false, false, false, false));
        } else if task["completed"].as_bool().unwrap() && show_complete {
            println!("{}", show_user(task, true, true, false, false, true, true));
        }
    }
}

fn show_user(inp: &serde_json::Value, id: bool, text: bool, catagory: bool, created_at: bool, completed_at: bool, completed: bool) -> String {
    let mut out: String = "".to_string();
    if id {
        out = [out.clone(), inp["id"].to_string()].join(" ");
    }
    if text {
        out = [out.clone(), inp["text"].to_string()].join(" ");
    }
    if catagory {
        out = [out.clone(), inp["categories"].to_string()].join(" ");
    }
    if created_at {
        out = [out.clone(), inp["created_at"].to_string()].join(" ");
    }
    if completed_at {
        out = [out.clone(), inp["completed_at"].to_string()].join(" ");
    }
    if completed {
        out = [out.clone(), inp["completed"].to_string()].join(" ");
    }
    return out.to_string();
}

fn loadjson(x: PathBuf) -> serde_json::Value {
    let path = x.into_os_string().into_string().unwrap();
    let file = std::fs::read_to_string(path).expect("Unable to read file");
    let json: serde_json::Value = serde_json::from_str(&file).expect("Eror: JSON was not well-formatted");
    return json;
}

fn create_task(inp: &str, categories: Vec<&str> ) {
    let mut json = loadjson(get_file());
    let tasks = json.get_mut("tasks").expect("Failed to parse tasks").as_array_mut().expect("Failed to parse tasks");
    let last_id = match tasks.last() {
        Some(task) => task["id"].as_u64().unwrap() + 1,
        None => 1,
    };
    let new_task = serde_json::json!({
        "id": last_id,
        "text": inp.to_string(),
        "categories": categories,
        "created_at": get_time(),
        "completed_at": "N/A",
        "completed": false
    });
    tasks.push(new_task);

    save_json(get_file(), &json);
}

fn get_time() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted_date = local.format("%H:%M %d %B %Y").to_string();
    return formatted_date;
}

fn save_json(filename: PathBuf, json: &serde_json::Value) {
    let serialized = serde_json::to_string_pretty(json).expect("Failed to serialize JSON");
    let mut file = fs::File::create(&filename).expect("Failed to open file for writing");
    file.write_all(serialized.as_bytes()).expect("Failed to write JSON to file");
    file.flush().expect("Failed to flush file");
    file.sync_all().expect("Failed to sync file");
}

fn toggle_completed(task_ids: Vec<&str>, complete: bool) {
    let mut json = loadjson(get_file());
    let tasks = json
        .get_mut("tasks")
        .expect("Failed to parse tasks")
        .as_array_mut()
        .expect("Failed to parse tasks");

    for task in tasks.iter_mut() {
        if let Some(id) = task.get("id").and_then(serde_json::Value::as_u64) {
            if task_ids.contains(&id.to_string().as_str()) && complete {
                task["completed"] = serde_json::json!(true);
                task["completed_at"] = serde_json::json!(get_time());
            } else if task_ids.contains(&id.to_string().as_str()) {
                task["completed"] = serde_json::json!(false);
                task["completed_at"] = serde_json::json!("N/A");
            }
        }
    }

    save_json(get_file(), &json);
}
