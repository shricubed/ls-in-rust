use chrono::{DateTime, Local};
use std::error::Error;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(default_value = ".", parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    if let Err(ref e) = run(&opt.path) {
        println!("{}", e);
        process::exit(1);
    }
}

fn permission_parse(mode: u16) -> String {
    let user = triplet(mode, 256, 128, 64);
    let group = triplet(mode, 32, 16, 8);
    let other = triplet(mode, 4, 2, 1);
    [user, group, other].join("")
}

fn triplet(mode: u16, read: u16, write: u16, execute: u16) -> String {
    match (mode & read, mode & write, mode & execute) {
        (0, 0, 0) => "---",
        (_, 0, 0) => "r--",
        (0, _, 0) => "-w-",
        (0, 0, _) => "--x",
        (_, 0, _) => "r-x",
        (_, _, 0) => "rw-",
        (0, _, _) => "-wx",
        (_, _, _) => "rwx",
    }
    .to_string()
}

fn run(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_name = entry
                .file_name()
                .into_string()
                .or_else(|f| Err(format!("Invalid entry: {:?}", f)))?;
            let metadata = entry.metadata()?;
            let size = metadata.len();
            let mode = metadata.permissions().mode();
            let modified: DateTime<Local> = DateTime::from(metadata.modified()?);

            println!(
                "{} {:>5} {} {}",
                permission_parse(mode as u16),
                size,
                modified.format("%_d %b %H:%M").to_string(),
                file_name
            );
        }
    }
    Ok(())
}
