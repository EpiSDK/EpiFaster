/*
** EPITECH PROJECT, 2026
** ~/epitech/free/EpiFaster/src
** File description:
** main.rs
*/

use std::env;
mod options;
mod arguments;

// const EPICLANG_COMPILER: &str = "clang";

pub fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut param: arguments::Parameter;

    param = arguments::get_parameters(args);
}
