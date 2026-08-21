/*
** EPITECH PROJECT, 2026
** ~/epitech/free/EpiLinter/src/epifaster
** File description:
** plugin.rs
*/

use std::{env, process::Command};

const EPICLANG_PLUGIN_PATH: &str = "/opt/epiclang/lib/plugins/epitech-plugin-banana.so";
const EPICLANG_COMPILER: &str = "clang";

pub fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();

    args.push(format!("-fplugin={}", EPICLANG_PLUGIN_PATH));

    Command::new(EPICLANG_COMPILER)
        .args(args)
        .status()
        .expect("Error: on lauching epifaster");
}
