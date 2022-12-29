use std::fs;
use std::env;
use std::collections::HashMap;
use std::str::FromStr;
use std::cmp;

#[derive(Debug)]
enum FileInfo {
    Dir(String),
    Size(usize, String),
}

#[derive(Debug)]
enum CommandOrInfo {
    ChangeDirectory(String),
    ListDirectory,
    Info(FileInfo),
}

fn parse_command(line: &str) -> Option<CommandOrInfo> {
    if line.len() == 0 {
        return None
    }
    let bits = line.split(" ").collect::<Vec<&str>>();
    match (bits[0], bits[1]) {
        ("$", "cd") => Some (CommandOrInfo::ChangeDirectory(bits[2].to_string())),
        ("$", "ls") => Some (CommandOrInfo::ListDirectory),
        ("dir", filename) => Some (CommandOrInfo::Info(FileInfo::Dir(bits[1].to_string()))),
        (size, filename) => Some (CommandOrInfo::Info(FileInfo::Size(usize::from_str(size).unwrap(), bits[1].to_string()))),
        _ => None
    }
}

fn parse_args() -> Vec<CommandOrInfo> {
    let args: Vec<String> = env::args().skip(1).collect();
    let data = fs::read_to_string(&args[0]).unwrap();
    let mut commands = vec![];
    for line in data.lines() {
        match parse_command(line) {
            Some(c) => commands.push(c),
            None => ()
        }
    }
    commands
}

fn change_directory(current_directory: &mut Vec<String>, to: &str) {
    println!("Started with {:?}, to {}", current_directory, to);
    if to.eq("/") {
        return;
    }
    if to.eq("..") {
        current_directory.pop();
        return;
    }
    current_directory.push(to.to_string());
}

#[derive(Debug)]
struct DirectoryInfo {
    subdirectories: Vec<String>,
    size: usize,
}

fn record_file_info(directories: &mut HashMap<String, DirectoryInfo>, current_directory: &Vec<String>, info: FileInfo) {
    let key = match current_directory.len() {
        0 => "/".to_string(),
        _ => format!("/{}", current_directory.join("/"))
    };
    if !directories.contains_key(&key) {
        directories.insert(key.clone(), DirectoryInfo { subdirectories: vec![], size: 0});
    }
    match info {
        FileInfo::Dir(subdir) => {
            let subdir_name = match key.as_str() {
                "/" => format!("/{}", subdir),
                _ => format!("{}/{}", key, subdir),
            };
            directories.get_mut(&key).unwrap().subdirectories.push(subdir_name);
        }
        FileInfo::Size(size, filename) => {
            directories.get_mut(&key).unwrap().size += size;
        }
    }
}

fn recursive_size(directories: &HashMap<String, DirectoryInfo>, dir: &str) -> usize {
    let directory = directories.get(dir).unwrap();
    let mut size = directory.size.clone();
    for subdir in &directory.subdirectories {
        size += recursive_size(&directories, &subdir);
    }
    size
}

fn minimum_fit(directories: &HashMap<String, DirectoryInfo>, dir: &str, target: usize) -> (usize, Option<usize>) {
    let directory = directories.get(dir).unwrap();
    let mut size = directory.size.clone();
    let mut current_min = None;
    for subdir in &directory.subdirectories {
        let (subdir_size, subdir_min) = minimum_fit(&directories, &subdir, target);
        current_min = match (current_min, subdir_min) {
            (None, m) | (m, None) => m,
            (Some(l), Some(r)) => Some (cmp::min(l, r)),
        };
        size += subdir_size;
    }
    if size > target {
        current_min = match current_min {
            None => Some(size),
            Some(m) => Some (cmp::min(m, size)),
        }
    }
    (size, current_min)
}


fn main() {
    let commands = parse_args();
    let mut directories = HashMap::new();
    let mut current_directory: Vec<String> = vec![];
    for command in commands {
        match command {
            CommandOrInfo::ChangeDirectory(to) => change_directory(&mut current_directory, &to),
            CommandOrInfo::ListDirectory => (),
            CommandOrInfo::Info(file_info) => record_file_info(&mut directories, &current_directory, file_info)
        }
    }
    println!("{:?}", &directories);
    let total_size = recursive_size(&directories, "/");
    println!("{} {}", total_size, total_size - 40000000);
    println!("{:?}", minimum_fit(&directories, "/", total_size - 40000000));
}
