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
    let files = parameters.files;
    // let mut handles = Vec::new();
        
    for file in files.clone() {
    //     handles.push(thread::spawn(move || {
            coding_style::banana::checker(file);
        // }));
    }
    // for handle in handles {
    //     handle.join().expect("Error on thread");
    // }
    
    if parameters.build {
        build::compiler(files, parameters.supplement_args);
    }
}
