/*
** LUKAS SOIGNEUX PROJECT, 2026
** directory
** File description:
** filename
*/

use std::fmt;
use colored::Colorize;

#[derive(Clone, Copy)]
pub enum Level {
    Info,
    Minor,
    Major,
    Fatal
}

pub struct Violation {
    pub reference: &'static str,
    pub line_start: Option<u32>,
    pub line_end: Option<u32>,
    pub column: Option<u32>,
    pub file: &'static str,
    pub error: Option<String>,
}

impl Violation {
    pub fn new(reference: &'static str, line_start: Option<u32>, line_end: Option<u32>, column: Option<u32>, file: &'static str, error: Option<String>) -> Self {
        Self { reference, line_start, line_end, column, file, error }
    }
    
    fn get_level(level: Level) -> &'static str {
        match level {
            Level::Info => "Info",
            Level::Minor => "Minor",
            Level::Major => "Major",
            Level::Fatal => "Fatal"
        }
    }

    fn get_error(reference: &'static str, error: Option<String>) -> (String, Level) {
        match reference {
            "C-A1" => ("pointer must be a constant when not modified in the function".to_string(), Level::Info),
            "C-A3" => ("file not ending with a newline".to_string(), Level::Info),
            "C-C1" => (error.unwrap_or_default(), Level::Major),
            "C-C2" => (error.unwrap_or_default(), Level::Major),
            "C-C3" => (error.unwrap_or_default(), Level::Major),
            "C-F2" => (error.unwrap_or_default(), Level::Minor),
            "C-F3" => (error.unwrap_or_default(), Level::Major),
            "C-F4" => (error.unwrap_or_default(), Level::Major),
            "C-F5" => (error.unwrap_or_default(), Level::Major),
            "C-F6" => ("function pointer without parameter".to_string(), Level::Major),
            "C-F7" => ("structure received by copy".to_string(), Level::Major),
            "C-F8" => ("comment inside function".to_string(), Level::Minor),
            "C-G1" => ("file not starting with standard Epitech header".to_string(), Level::Minor),
            "C-G2" => (error.unwrap_or_default(), Level::Minor),
            "C-G3" => (error.unwrap_or_default(), Level::Minor),
            "C-G4" => ("global variable".to_string(), Level::Major),
            "G-G5" => ("non-header file include".to_string(), Level::Major),
            "C-G6" => (error.unwrap_or_default(), Level::Minor),
            "C-G7" => ("trailing space".to_string(), Level::Minor),
            "C-G8" => ("trailing empty line".to_string(), Level::Minor),
            "C-G10" => ("inline assembler is forbidden".to_string(), Level::Fatal),
            "C-H1" => (error.unwrap_or_default(), Level::Major),
            "C-H2" => (error.unwrap_or_default(), Level::Major),
            "C-H3" => (error.unwrap_or_default(), Level::Major),
            "C-L1" => (error.unwrap_or_default(), Level::Major),
            "C-L2" => (error.unwrap_or_default(), Level::Minor),
            "C-L3" => (error.unwrap_or_default(), Level::Minor),
            "C-L4" => (error.unwrap_or_default(), Level::Minor),
            "C-L5" => (error.unwrap_or_default(), Level::Major),
            "C-L6" => (error.unwrap_or_default(), Level::Minor),
            "C-O1" => ("forbidden temp file".to_string(), Level::Major),
            "C-O2" => ("source file must be .c or .h".to_string(), Level::Major),
            "C-O3" => (error.unwrap_or_default(), Level::Major),
            "C-V1" => (error.unwrap_or_default(), Level::Minor),
            "C-V3" => (error.unwrap_or_default(), Level::Minor),
            _ => {
                panic!();
            }
        }
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let error = Violation::get_error(self.reference, self.error.clone());
        
        write!(
            f,
            "{}:{}:{}: {}: {} [{}] {} ({})",
            self.file.to_string().bold(),
            self.line_start.map_or("?".to_string(), |l| l.to_string()).bold(),
            self.column.map_or("?".to_string(), |c| c.to_string()).bold(),
            "warning".magenta().bold(),
            "[Banana]".bold(),
            Violation::get_level(error.1).to_string().bold(),
            error.0.to_string().bold(),
            self.reference.to_string().bold(),
        )
    }
}
