use std::io::Write;
use crate::dice::parser::parse_dice_roll;
use crate::dice::rolls::{roll, RollResult};

mod dice;

fn main() {

    let mut input = String::new();
    println!("Welcome to the Dice Roller!");
    loop {
        print!(">: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line"); // CAN PANIC HERE!
        input = input.trim().to_lowercase();

        if input.is_empty() {
            continue;
        }
        if input.eq(&"quit".to_string()) || input.eq(&"q".to_string()) || input.eq(&"exit".to_string()) {
            break;
        }

        roll_dice_app(
            input.split_whitespace().map(|s| s.to_string()).collect()
        );
        input.clear();
    }
}

fn roll_dice_app(roll_expressions: Vec<String>) {
    let mut results = Vec::<RollResult>::with_capacity(roll_expressions.len());
    for i in 0..roll_expressions.len() {
        let expression = &roll_expressions[i];
        let rolls = parse_dice_roll(expression).unwrap();
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
