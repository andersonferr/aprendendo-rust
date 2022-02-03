use std::{collections::HashMap, io, process, str};

fn read(msg: &str) -> i32 {
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
    let n = read("Digite a quantidade de números a serem lidos");

    if n <= 0 {
        println!("Nenhuma entrada a ser lida!");
        return;
    }

    let mut numbers: Vec<i32> = Vec::new();

    for i in 1..=n {
        let x = read(&format!("Digite a {}º entrada:", i));
        numbers.push(x);
    }

    numbers.sort();

    let mut table: HashMap<i32, u32> = HashMap::new();
    for number in numbers {
        let count = table.entry(number).or_insert(0);
        *count += 1;
    }

    let mut max = 0;
    for (_, v) in table.iter() {
        if max < *v {
            max = *v;
        }
    }

    if max == 1 {
        println!("O conjunto não tem moda.");
        return;
    }

    let mut modes: Vec<i32> = Vec::new();
    for (number, count) in table.iter() {
        if *count == max {
            modes.push(*number);
        }
    }

    modes.sort();

    let mut line = String::new();
    line.push_str(&format!("{}", modes[0]));
    for mode in &modes[1..] {
        line.push_str(&format!(", {}", mode))
    }

    if modes.len() == 1 {
        println!("A moda é {}.", line);
    } else {
        println!("As modas são {}.", line)
    }
}
