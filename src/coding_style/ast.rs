/*
** LUKAS SOIGNEUX PROJECT, 2026
** directory
** File description:
** filename
*/

use super::violation::Violation;

pub fn checker() {
    println!("{}", Violation::new("C-O3", Some(42), Some(43), Some(2), "src/file.c", None));
}