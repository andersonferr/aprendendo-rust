use std::io;

fn read_line() -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut line = String::new();

    io::stdin().read_line(&mut line).expect("fail to read line");

    for word in line.split_whitespace() {
        words.push(word.to_string());
    }

    words
}

fn pig_latin(word: &String) -> String {
    let mut pgw = String::new();

    if word.len() == 0 {
        return pgw;
    }

    let chars: Vec<char> = word.chars().collect();

    match chars[0] {
        'b'..='d'
        | 'f'..='h'
        | 'j'..='n'
        | 'p'..='t'
        | 'v'..='z'
        | 'B'..='D'
        | 'F'..='H'
        | 'J'..='N'
        | 'P'..='T'
        | 'V'..='Z' => {
            for ch in &chars[1..] {
                pgw.push(*ch);
            }

            pgw.push('-');
            pgw.push(chars[0]);
        }
        _ => {
            pgw.push_str(&word);
            pgw.push_str("-h");
        }
    }

    pgw.push_str("ay");

    pgw
}

fn main() {
    loop {
        let words = read_line();

        if words.len() == 0 {
            break;
        }

        let mut pig_latin_words: Vec<String> = Vec::new();

        for word in words {
            let pig_latin_word = pig_latin(&word);
            pig_latin_words.push(pig_latin_word);
        }

        println!("{}", pig_latin_words.join(" "));
    }
}
