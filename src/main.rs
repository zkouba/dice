use std::cmp::{max, min};
use std::io::{Write};
use crossterm::{cursor, event, terminal, ExecutableCommand};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::terminal::ClearType;
use crate::dice::parser::parse_dice_roll;
use crate::dice::rolls::{roll, RollResult};

mod dice;

fn main() {
    let prompt = ">: ";
    let mut stdout = std::io::stdout();
    let mut input = String::new();
    let mut history = Vec::<String>::new();

    println!("\n\nWelcome to the Dice Roller!");
    stdout.flush().expect("Failed to flush stdout");
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
                        return;
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
                        stdout.execute(cursor::MoveToColumn(0)).unwrap();
                        stdout.execute(terminal::Clear(ClearType::CurrentLine)).unwrap();
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
                        stdout.execute(cursor::MoveToColumn(0)).unwrap();
                        stdout.execute(terminal::Clear(ClearType::CurrentLine)).unwrap();
                        print!("{prompt}{input}");
                    }
                    KeyCode::Backspace => {
                        input.pop();
                        history_index = 0;

                        // Redraw prompt + current input (so removed char is reflected).
                        stdout.execute(cursor::MoveToColumn(0)).unwrap();
                        stdout.execute(terminal::Clear(ClearType::CurrentLine)).unwrap();
                        print!("{prompt}{input}");
                    }
                    KeyCode::Char(c) => {
                        if !key.modifiers.contains(KeyModifiers::CONTROL) {
                            input.push(c);
                            history_index = 0;
                            print!("{c}")
                        }
                    }
                    _ => {}
                }
            }
        }
        stdout.execute(cursor::MoveToColumn(0)).unwrap();
        terminal::disable_raw_mode().expect("Failed to disable raw mode");

        input = input.trim().to_lowercase();
        if input.is_empty() {
            continue;
        }
        if input.eq(&"quit".to_string()) || input.eq(&"q".to_string()) || input.eq(&"exit".to_string()) {
            break;
        }

        history.push(input.clone());
        roll_dice_app(
            input.clone().split_whitespace().map(|s| s.to_string()).collect()
        );
        input.clear();
    }
}

fn roll_dice_app(roll_expressions: Vec<String>) {
    let mut results = Vec::<RollResult>::with_capacity(roll_expressions.len());
    for i in 0..roll_expressions.len() {
        let expression = &roll_expressions[i];
        let rolls_res = parse_dice_roll(expression);
        if rolls_res.is_err() {
            println!("Error parsing dice roll: {}", roll_expressions[i]);
            return;
        }
        let rolls = rolls_res.unwrap();
        for roll_cmd in rolls {
            results.push(roll(roll_cmd));
        }
    }

    let mut sum: u8 = 0;
    for result in results {
        println!("{:?}", result);
        sum += result.value;
    }
    println!("Total: {}", sum);
}
