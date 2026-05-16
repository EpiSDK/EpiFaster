/*
** LUKAS SOIGNEUX PROJECT, 2026
** directory
** File description:
** filename
*/

use std::env;

mod arguments;
mod build;
mod options;
mod coding_style;

pub fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let parameters = arguments::get_parameters(args);

    coding_style::ast::checker();
    if parameters.build {
        build::compiler(parameters.files, parameters.supplement_args);
    }
}
