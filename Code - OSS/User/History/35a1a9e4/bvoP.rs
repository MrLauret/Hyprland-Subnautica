use crossterm::{ExecutableCommand, terminal, cursor};

fn terminal_setup() {
    stdout().execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    stdout().execute(cursor::MoveToRow(0)).unwrap();
}