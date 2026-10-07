use std::cmp::Ordering;
use std::io::{self, Write};

use rand::prelude::*;

fn main() {
    println!("Давай сыграем в игру! Угадай число!");
    println!("Число которое я загадал находится в диапазоне от 1 до 100 включительно.\n");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        print!("Пожалуйста, введите ваше число: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Запись строки не удалась\n");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Не удалось распознать число. Пожалуйста, введите целое число от 1 до 100.\n");
                continue;
            },
        };

        if guess > 100 || guess < 1 {
            println!("Напоминаю, я загадал число в диапозоне от 1 до 100.\n");
            continue;
        }

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Мало, нужно БОЛЬШЕ!\n"),
            Ordering::Greater => println!("Многовато, давай поменьше.\n"),
            Ordering::Equal => {
                println!("Ты угадал! Это ПОБЕДА!!!");
                println!("На, держи конфетку 🍬");
                break;
            }
        }
    }
}
