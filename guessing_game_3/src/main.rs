use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个数字！");
                continue;
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("你猜小啦！"),
            Ordering::Greater => println!("你猜大啦！"),
            Ordering::Equal => {
                println!("你猜对了！");
                break;
            },
        }
    }
}
