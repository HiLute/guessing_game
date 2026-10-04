use std::cmp::Ordering;
use std::io::{self, Write};

use rand::prelude::*;

fn main() {
    println!("Угадай число!");

    let secret_number = rand::rng().random_range(1..=100);

    // println!("Секретное число: {secret_number}");

    loop {
        print!("Пожалуйста, введите ваше число: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Запись строки не удалась");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Мало, нужно БОЛЬШЕ!\n"),
            Ordering::Greater => println!("Многовато, давай поменьше.\n"),
            Ordering::Equal => {
                println!("Ты угадал! Это ПОБЕДА!!!");
                break;
            }
        }
    }
}
