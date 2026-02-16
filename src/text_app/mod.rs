use std::cmp::{max, min};
use std::io::Write;
use crossterm::{cursor, event, terminal, ExecutableCommand};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::terminal::ClearType;
use crate::dice::error::DiceError;

pub fn text_app_loop(prompt: &str, app_logic: &dyn Fn(Vec<String>) -> Result<(), DiceError>) -> Result<(), DiceError> {
    let mut stdout = std::io::stdout();
    let mut input = String::new();
    let mut history = Vec::<String>::new();
    let mut history_index = 0;

    loop {
        print!("{prompt}");

        terminal::enable_raw_mode().expect("Failed to enable raw mode");
        // Read keys until Enter builds a command line.
        loop {
            stdout.flush().expect("Failed to flush stdout");
            // Blocks until an event is available
            if let Event::Key(key) = event::read().expect("Failed to read key") {
                match key.code {
                    KeyCode::Enter => {
                        println!();
                        break;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        println!();
                        stdout.execute(cursor::MoveToColumn(0)).expect("Failed to reset cursor.");
                        println!("Exiting...");
                        terminal::disable_raw_mode().expect("Failed to disable raw mode");

                        return Ok(());
                    }
                    KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                        input.push(c);
                        history_index = 0;
                        print!("{c}")
                    }
                    KeyCode::Esc => {
                        input.clear();
                        history_index = 0;
                        continue;
                    }
                    KeyCode::Up => {
                        if history.is_empty() {
                            continue;
                        }
                        history_index += 1;
                        history_index = min(history_index, history.len());
                        input = history[history.len() - history_index].clone();

                        // Redraw prompt + current input.
                        stdout.execute(cursor::MoveToColumn(0)).expect("Failed to reset cursor.");
                        stdout.execute(terminal::Clear(ClearType::CurrentLine)).expect("Failed to clear line.");
                        print!("{prompt}{input}");
                    }
                    KeyCode::Down => {
                        if history.is_empty() {
                            continue;
                        }
                        history_index -= 1;
                        history_index = max(1, history_index);
                        input = history[history.len() - history_index].clone();

                        // Redraw prompt + current input.
                        stdout.execute(cursor::MoveToColumn(0)).expect("Failed to reset cursor.");
                        stdout.execute(terminal::Clear(ClearType::CurrentLine)).expect("Failed to clear line.");
                        print!("{prompt}{input}");
                    }
                    KeyCode::Backspace => {
                        input.pop();
                        history_index = 0;

                        // Redraw prompt + current input (so removed char is reflected).
                        stdout.execute(cursor::MoveToColumn(0)).expect("Failed to reset cursor.");
                        stdout.execute(terminal::Clear(ClearType::CurrentLine)).expect("Failed to clear line.");
                        print!("{prompt}{input}");
                    }
                    _ => {}
                }
            }
        }
        stdout.execute(cursor::MoveToColumn(0)).expect("Failed to reset cursor.");
        terminal::disable_raw_mode().expect("Failed to disable raw mode");

        input = input.trim().to_lowercase();
        if input.is_empty() {
            continue;
        }
        if input.eq(&"quit".to_string()) || input.eq(&"q".to_string()) || input.eq(&"exit".to_string()) {
            break;
        }

        history.push(input.clone());
        app_logic(
            input.clone().split_whitespace().map(|s| s.to_string()).collect()
        )?;
        input.clear();
    }
    Ok(())
}