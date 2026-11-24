use std::io::{stdout, stdin};
use crossterm::{ExecutableCommand, terminal, cursor};

pub fn terminal_setup() {
    stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout().execute(cursor::MoveToRow(0)).unwrap();
}

pub fn enter_to_continue() {
    let mut raw_string = String::new();
    println!("Press enter to continue");
    stdin().read_line(&mut raw_string).unwrap();
}