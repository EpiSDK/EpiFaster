/*
** EPITECH PROJECT, 2026
** ~/epitech/free/EpiFaster/src
** File description:
** arguments.rs
*/

use std::fs;
use crate::options;

pub struct Parameter {
    build: bool,
    directories: Vec<String>,
    files: Vec<String>,
    supplement_args: Vec<String>
}

fn assign_parameter(build: bool, directories: Vec<String>, files: Vec<String>, supplement_args: Vec<String>) -> Parameter {
    Parameter {
        build: build,
        directories: directories,
        files: files,
        supplement_args: supplement_args,
    }
}

fn handle_path_argument(arg: &str, files: &mut Vec<String>, directories: &mut Vec<String>) -> bool {
    if let Ok(metadata) = fs::metadata(&arg) {
        if metadata.is_file() {
            files.push(arg.to_string());
        }
        if metadata.is_dir() {
            directories.push(arg.to_string());
        }
        return true;
    }
    false
}

pub fn get_parameters(args: Vec<String>) -> Parameter {
    let mut build = false;
    let mut files: Vec<String> = vec![];
    let mut directories: Vec<String> = vec![];
    let mut supplement_arg: Vec<String> = vec![];

    for arg in args {
        match arg.as_str() {
            "-h" | "--help" => options::help(),
            "-v" | "--version" => options::version(),
            "-b" | "--build" => build = true,
            _ => {
                if !handle_path_argument(&arg, &mut files, &mut directories) && build {
                    supplement_arg.push(arg);
                }
            }
        }
    }
    return assign_parameter(build, directories, files, supplement_arg);
}
