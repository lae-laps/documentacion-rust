use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivina el número");

    let random_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Introduce tu número : ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Tu número es : {}", guess);

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Muy pequeño!"),
            Ordering::Greater => println!("Muy grande"),
            Ordering::Equal => {
                println!("Ganaste!");
                break;
            }
        }
    }
}
