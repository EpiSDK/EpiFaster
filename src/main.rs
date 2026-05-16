/*
** EPITECH PROJECT, 2026
** ~/epitech/free/EpiFaster/src
** File description:
** main.rs
*/

use std::env;

mod options;
mod arguments;
mod build;

pub fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let parameters = arguments::get_parameters(args);

    if parameters.build {
        build::compiler(parameters.files, parameters.supplement_args);
    }
}
