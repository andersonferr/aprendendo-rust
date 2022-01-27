use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Adivinha o número.");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Por favor, insira um número.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falhou a ler a string.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você inseriu {}.", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("O núnero secreto é maior."),
            Ordering::Greater => println!("O número secreto é menor."),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
