/*
** LUKAS SOIGNEUX PROJECT, 2026
** directory
** File description:
** filename
*/

use std::fmt;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Level {
    Info,
    Minor,
    Major
}

pub struct Violation {
    pub reference: &'static str,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub level: Level,
}

impl Violation {
    pub fn new(reference: &'static str, line: Option<u32>, column: Option<u32>, level: Level) -> Self {
        Self { reference, line, column, level }
    }
    
    fn get_level(level: Level) -> &'static str {
        match level {
            Level::Info => "Info",
            Level::Minor => "Minor",
            Level::Major => "Major",
        }
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} (line: {}, col: {})",
            Violation::get_level(self.level),
            self.reference,
            self.line.map_or("?".to_string(), |l| l.to_string()),
            self.column.map_or("?".to_string(), |c| c.to_string()),
        )
    }
}
