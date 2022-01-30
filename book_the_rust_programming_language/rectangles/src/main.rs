use std::io;

fn main() {
    let width = read_uint32("Digite a largura do retangulo");
    let height = read_uint32("Digite a altura do retangulo");

    println!("A área do retangulo é {}.", area(width, height));
}

fn read_uint32(msg: &str) -> u32 {
    loop {
        println!("{}:", msg);
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("fail to read line");

        if let Ok(n) = line.trim().parse() {
            return n;
        }

        println!("Entrada inválida!");
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
