use crate::utils::file_to_lines;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Filetype {
    FILE,
    DIRECTORY,
}

#[derive(Debug, PartialEq)]
pub struct FileOrDirValue {
    ty: Filetype,
    name: String,
    size: Option<u64>,
}

impl FileOrDirValue {
    pub fn new_dir(name: String) -> FileOrDirValue {
        FileOrDirValue {
            ty: Filetype::DIRECTORY,
            name: name,
            size: None,
        }
    }

    pub fn new_file(name: String, size: u64) -> FileOrDirValue {
        FileOrDirValue {
            ty: Filetype::FILE,
            name: name,
            size: Some(size),
        }
    }
}

pub fn get_directory_size(
    directory_info: &HashMap<String, Vec<FileOrDirValue>>,
    name: &str,
) -> u64 {
    let children = directory_info.get(name).unwrap();
    children
        .iter()
        .map(|child| {
            if child.ty == Filetype::FILE {
                child.size.unwrap()
            } else {
                get_directory_size(directory_info, &child.name)
            }
        })
        .sum()
}

pub fn general_parser(filepath: &str, part: usize) -> Result<(), std::io::Error> {
    let mut absolute_dir = "/".to_string();

    let mut directory_info: HashMap<String, Vec<FileOrDirValue>> =
        HashMap::from([("/".to_owned(), vec![])]);

    for line in file_to_lines(filepath)? {
        let txt = line.unwrap();
        let first_char = txt.chars().next().unwrap();
        let mut cmd_or_info = txt.split(' ');
        if first_char == '$' {
            let (_, cmd, arg) = (
                cmd_or_info.next(),
                cmd_or_info.next().unwrap().to_string(),
                cmd_or_info.next(),
            );
            if cmd == "ls" {
            } else if cmd == "cd" {
                if arg.unwrap() == ".." {
                    let mut splits = absolute_dir
                        .split("/")
                        .filter(|x| *x != "")
                        .collect::<Vec<&str>>();
                    splits.pop();
                    absolute_dir = format!("/{}/", splits.join("/"));
                    if absolute_dir == "//" {
                        absolute_dir = "/".to_string();
                    }
                } else if arg.unwrap() == "/" {
                    absolute_dir = "/".to_string();
                } else {
                    absolute_dir = format!("{absolute_dir}{}/", arg.unwrap().to_string());
                }
            } else {
                panic!("Unexpected command: {cmd}");
            }
        } else {
            let (file_or_dir_info, name) =
                (cmd_or_info.next().unwrap(), cmd_or_info.next().unwrap());
            let size = file_or_dir_info.parse::<u64>();
            let name = name.to_owned();
            let child = if file_or_dir_info == "dir" {
                let dir_name = format!("{absolute_dir}{name}/");
                FileOrDirValue::new_dir(dir_name.clone())
            } else {
                let size = size.unwrap();
                FileOrDirValue::new_file(name.clone(), size)
            };
            if directory_info.contains_key(&absolute_dir) {
                directory_info.get_mut(&absolute_dir).unwrap().push(child);
            } else {
                directory_info.insert(absolute_dir.clone(), vec![child]);
            };
        }
    }

    let directory_keys: Vec<String> = directory_info.keys().map(|key| key.to_owned()).collect();

    if part == 1 {
        let directory_size: u64 = directory_keys
            .iter()
            .map(|name| get_directory_size(&directory_info, name))
            .filter(|total_size| *total_size < 100000)
            .sum();

        println!("Total: {directory_size}");
    } else if part == 2 {
        let needed_space: u64 = 70000000 - 30000000;

        let directory_sizes = directory_keys
            .iter()
            .map(|name| (name, get_directory_size(&directory_info, name)))
            .collect::<Vec<(&String, u64)>>();
        let total_used_space = directory_sizes
            .iter()
            .filter(|(name, _)| *name == "/")
            .map(|(_, size)| *size)
            .next()
            .unwrap();
        let directory_size = directory_sizes
            .iter()
            .filter(|(_, size)| *size >= (total_used_space - needed_space))
            .map(|(_, size)| *size)
            .min()
            .unwrap();

        println!("Total: {directory_size}");
    }

    Ok(())
}

pub fn part_1(filepath: &str) -> Result<(), std::io::Error> {
    general_parser(filepath, 1)
}

pub fn part_2(filepath: &str) -> Result<(), std::io::Error> {
    general_parser(filepath, 2)
}
