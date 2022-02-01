use std::io;

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn main() {
    let width = read_uint32("Digite a largura do retângulo");
    let height = read_uint32("Digite a altura do retângulo");

    let rect1 = Rectangle { width, height };

    println!("A área do retângulo é {}.", rect1.area());

    let hectare = Rectangle::square(100);

    if hectare.can_hold(&rect1) {
        println!("Seu retângulo cabe em um quadrado 100x100.");
    } else {
        println!("Seu retângulo NÃO cabe em um quadrado 100x100.");
    }
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
