use std::io;

fn main() {
    
    let mut year = String::new();

    io::stdin()
        .read_line(&mut year)
        .expect("Erro ao ler o ano.");

    let year: u32 = year
        .trim()
        .parse()
        .expect("Ano tem que ser um numero inteiro positivo");

    if year % 400 == 0 {
        println!("Ano é bissexto.");
    } else if year % 100 == 0 {
        println!("Ano é não bissexto.");
    } else if year % 4 == 0 {
        println!("Ano é bissexto.");
    } else {
        println!("Ano é não bissexto.");
    }
}
