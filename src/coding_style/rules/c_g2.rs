/*
** LUKAS SOIGNEUX PROJECT, 2026
** directory
** File description:
** filename
*/

use std::str::FromStr;
use crate::Violation;
use tree_sitter::Node;
use crate::coding_style::abstract_syntax_tree::Rules;

pub fn check_separation_function(node: Node<'_>, rules: &mut Rules, infraction: &mut Violation) {
    let mut cursor = node.walk();
    cursor.goto_previous_sibling();
    
    let prev_sibling = node.prev_sibling();
    
    if let Some(prev) = prev_sibling {
        let current_start = node.start_position().row;
        let prev_end = prev.end_position().row + 1;
        let gap = current_start.saturating_sub(prev_end);
        
        if gap < 1 && prev.kind() == "function_definition" {
            Violation::push(
                infraction,
                "C-G2",
                Some(prev_end + 1),
                Some(current_start),
                Some(0),
                Some(rules.file.clone()),
                Some(String::from_str("no blank line between function declarations").expect("Error")),
            );
        } else if gap > 1 && prev.kind() == "function_definition" {
            Violation::push(
                infraction,
                "C-G2",
                Some(prev_end + 1),
                Some(current_start),
                Some(0),
                Some(rules.file.clone()),
                Some(String::from_str("too many blank lines between function declarations").expect("Error")),
            );
        }
    }
}