use crate::io;
use std::process::Command;

pub fn init() {
    let _ = Command::new("git")
        .arg("init")
        .arg(&io::get_datastore_dir().unwrap())
        .output();
}

pub fn clone_repo(users_repo: String) {
    let _ = Command::new("git")
        .arg("clone")
        .arg(users_repo)
        .arg(&io::get_datastore_dir().unwrap())
        .output();

    // But if the users repo is empty, we need to create the todo.json file
    if io::check_file().is_err() {
        let _ = io::create_files();
    }
}

fn commit() {
    let _ = Command::new("git")
        .arg("-C")
        .arg(&io::get_datastore_dir().unwrap())
        .arg("add")
        .arg(".")
        .output();

    let commit_message = format!("{}: Update todo list", &io::get_time_string().unwrap());

    let _ = Command::new("git")
        .arg("-C")
        .arg(&io::get_datastore_dir().unwrap())
        .arg("commit")
        .arg("-m")
        .arg(commit_message)
        .output();
}

pub fn push() {
    commit();
    pull();

    let _ = Command::new("git")
        .arg("-C")
        .arg(&io::get_datastore_dir().unwrap())
        .arg("push")
        .output();
}

pub fn pull() {
    let _ = Command::new("git")
        .arg("-C")
        .arg(&io::get_datastore_dir().unwrap())
        .arg("pull")
        .output();
}
