/*
** LUKAS SOIGNEUX PROJECT, 2026
** directory
** File description:
** filename
*/

use crate::coding_style::{abstract_syntax_tree::abstract_syntax_tree, violation::Violation};

pub fn checker(file: String) {
    let err = Violation::new("C-A1", Some(0), Some(5), Some(0), Some(file.clone()), None);

    println!("{err}");
    println!("{}", Violation::get_context(err));
    abstract_syntax_tree(file);
}