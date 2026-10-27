/*
** EPITECH PROJECT, 2026
** ~/epitech/free/EpiLinter/src/epifaster
** File description:
** plugin.rs
*/

use std::{env, process::Command};

const EPICLANG_PLUGIN_PATH: &str = "/opt/epiclang/lib/plugins/epitech-plugin-banana.so";
const DEFAULT_COMPILER: &str = "clang-21";

pub fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let compiler = env::var("CC").unwrap_or_else(|_| DEFAULT_COMPILER.to_string());

    if args.iter().any(|v| v == "--version-epiclang") {
        println!("epiclang v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    args.push(format!("-fplugin={}", EPICLANG_PLUGIN_PATH));

    Command::new(compiler)
        .args(args)
        .status()
        .expect("Error: on lauching epifaster");
}
