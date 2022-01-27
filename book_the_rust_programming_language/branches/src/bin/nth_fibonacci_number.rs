use std::io;

fn main() {
    loop {
        let mut line = String::new();

        println!("Insira a posição ao qual deseja saber o elemento da sequencia de Fibonacci (0 para sair):");

        io::stdin()
            .read_line(&mut line)
            .expect("falha ao ler a linha");

        let nth = match line.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Entrada inválida!");
                continue;
            }
        };

        if nth <= 0 {
            break;
        }

        let (mut a, mut b) = (1, 0);

        for _ in 0..nth {
            let soma = a + b;
            a = b;
            b = soma;
        }

        let suffix = if nth % 10 == 1 {
            "st"
        } else if nth % 10 == 2 {
            "nd"
        } else if nth % 10 == 3 {
            "rd"
        } else {
            "th"
        };

        println!("O {}{} elemento é {}", nth, suffix, b);
    }
}
