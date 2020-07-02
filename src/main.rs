extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Угадай число!\n");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //For tests only
    //println!("Загаданное число: {}", secret_number);

    loop {
        println!("\nВведи предпологаемое число:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Не удалось прочитать строку :(");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Ваша попытка: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less      => print!("Слишком маленькое!"),
            Ordering::Greater   => print!("Слишком большое!"),
            Ordering::Equal     => {
                print!("Вы выиграли!");
                break;
            },
        }
    }
}
