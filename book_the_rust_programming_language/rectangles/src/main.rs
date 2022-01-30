use std::io;

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = read_uint32("Digite a largura do retangulo");
    let height = read_uint32("Digite a altura do retangulo");

    let rect1 = Rectangle{width, height};

    println!("A área do retangulo é {}.", area(&rect1));
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

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
