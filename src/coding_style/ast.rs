/*
** LUKAS SOIGNEUX PROJECT, 2026
** directory
** File description:
** filename
*/

use super::violation::Violation;
use super::violation::Level;

pub fn checker() {
    println!("{}", Violation::new("C-F4", Some(42), Some(2), Level::Minor));
}