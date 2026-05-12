/*
** EPITECH PROJECT, 2026
** ~/epitech/free/EpiLinter/src/epifaster
** File description:
** plugin.rs
*/

use std::{env, process::Command};

const EPICLANG_PLUGIN_PATH: &str = ".local/lib/epiclang/plugins/epitech-plugin-banana.so";
const EPICLANG_COMPILER: &str = "clang";

pub fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let home_dir = env::home_dir().expect("Error: home directory not found");

    args.push(format!("-fplugin={}/{}", home_dir.display(), EPICLANG_PLUGIN_PATH));

    Command::new(EPICLANG_COMPILER)
        .args(args)
        .status()
        .expect("Error: on lauching epifaster");
}
