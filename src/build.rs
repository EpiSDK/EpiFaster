/*
** EPITECH PROJECT, 2026
** ~/epitech/free/EpiFaster/src
** File description:
** build.rs
*/

use std::process::Command;

const EPICLANG_COMPILER: &str = "clang";

pub fn compiler(files: Vec<String>, supplement_args: Vec<String>) {
    Command::new(EPICLANG_COMPILER)
        .args(files)
        .args(supplement_args)
        .status()
        .expect("Epiclang fail to build");
}
