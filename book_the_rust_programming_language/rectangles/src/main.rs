use std::io;

fn main() {
    let width = read_uint32("Digite a largura do retangulo");
    let height = read_uint32("Digite a altura do retangulo");

    let rect1 = (width, height);

    println!("A área do retangulo é {}.", area(rect1));
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

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
