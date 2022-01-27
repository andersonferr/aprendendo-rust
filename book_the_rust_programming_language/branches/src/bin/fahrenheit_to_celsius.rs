use std::io;

fn read_float64(msg: &str) -> f64 {
    loop {
        let mut line = String::new();

        println!("{}:", msg);

        io::stdin()
            .read_line(&mut line)
            .expect("Falha ao ler da entrada padrão");

        let n: f64 = match line.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Entrade inválida.");
                continue;
            }
        };

        break n;
    }
}

fn main() {
    let f = read_float64("Insira a temperatura em °F");

    let c = (f - 32.0) * (5.0 / 9.0);

    println!("O resultado da conversão é {}°C", c);
}
