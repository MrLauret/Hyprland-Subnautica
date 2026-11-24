use std::io::stdout;
use crossterm::{ExecutableCommand, terminal, cursor};

pub fn terminal_setup() {
    stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout().execute(cursor::MoveToRow(0)).unwrap();
}

pub fn enter_to_continue() {
    
}