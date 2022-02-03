use std::{io, str, process};

fn read<T: str::FromStr>(msg: &str) -> T {
    loop {
        let mut line = String::new();

        println!("{}:", msg);

        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");

        if line.len() == 0 {
            println!("EOF");
            process::exit(1);
        }

        if let Ok(n) = line.trim().parse() {
            return n;
        }

        println!("Entrada Inválida!");
    }
}

fn main() {
    let n: i32 = read("Digite a quantidade de números a serem inseridos");

    let mut numbers: Vec<f64> = Vec::new();
    for i in 1..=n {
        let k: f64 = read(&format!("Digite a {}ª entrada", i));
        if k.is_nan() {
            panic!("Valor inválido: NaN");
        }

        numbers.push(k);
    }

    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = numbers.len();
    if len == 0 {
        return;
    }

    let median;

    let mid = len / 2;
    if len % 2 == 0 {
        median = (numbers[mid - 1] + numbers[mid]) / 2.0;
    } else {
        median = numbers[mid];
    }

    println!("A mediana é {}", median);
}
