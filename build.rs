use clap::CommandFactory;
use clap_complete::generate_to;
use std::{env, fs, process};

include!("src/cli.rs");

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        Some(outdir) => outdir,
        None => {
            eprintln!(
                "OUT_DIR environment variable not defined. \
                Please file a bug: \
                https://github.com/isabelroses/bellado/issues/new"
            );
            process::exit(1);
        }
    };
    fs::create_dir_all(&outdir).unwrap();

    let mut cmd = Cli::command();
    let name = cmd.get_name().to_string();
    let _ = generate_to(Shell::Bash, &mut cmd, name.clone(), &outdir);
    let _ = generate_to(Shell::Zsh, &mut cmd, name.clone(), &outdir);
    let _ = generate_to(Shell::Fish, &mut cmd, name.clone(), &outdir);
    let _ = generate_to(Shell::PowerShell, &mut cmd, name.clone(), &outdir);
}
