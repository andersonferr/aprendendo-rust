use std::{collections::HashMap, io, process};

fn read_line(msg: &String) -> String {
    println!("{}:", msg);
    let mut line = String::new();
    let res = io::stdin().read_line(&mut line);
    match res {
        Ok(0) => {
            println!("eof");
            process::exit(1);
        }

        Ok(_) => return line.trim().to_string(),

        Err(err) => {
            println!("error {}", err);
            process::exit(2);
        }
    }
}

fn add_department(dep: &mut HashMap<String, Vec<String>>) {
    let department = read_line(&String::from("Por favor, digite o nome do departamento"));
    if !dep.contains_key(&department) {
        dep.insert(department, Vec::new());
    } else {
        println!("Departamento já existe!");
    }
    
    println!();
}

fn add_emploeet_to_department(dep: &mut HashMap<String, Vec<String>>) {
    let department = read_line(&"Digite o nome do departamento".to_string());

    match dep.get_mut(&department) {
        Some(emploees) => {
            let emploee = read_line(&"Digite o nome do funcionário".to_string());
            emploees.push(emploee);
        }

        None => {
            println!("Departamento não cadastrado.");
            println!("Por favor, cadastre o departamento.");
        }
    }

    println!();
}

fn list_all(dep: &HashMap<String, Vec<String>>) {
    println!("----------------= Departamentos =----------------");
    for (dep, emps) in dep.iter() {
        println!("Departamento {}:", dep);
        for emp in emps {
            println!(" - {}", emp);
        }
    }

    println!();
}

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("+-----------------------------------------------+");
        println!("|            Bem vindo ao Departamentos!        |");
        println!("+-----------------------------------------------+");
        println!();
        println!("---------------------= Menu =--------------------");
        println!("1 Adicionar departamento");
        println!("2 Adicionar funcionario a departamento");
        println!("3 Listar todos os departamentos e funcionários");
        println!("0 Sair");
        println!();

        loop {
            let option = read_line(&String::from(
                "Por favor selecione o número da opção desejada",
            ));

            if let Ok(option) = option.parse() {
                match option {
                    1 => {
                        add_department(&mut departments);
                        break;
                    }

                    2 => {
                        add_emploeet_to_department(&mut departments);
                        break;
                    }

                    3 => {
                        list_all(&departments);
                        break;
                    }

                    0 => return,
                    _ => (),
                }
            }

            println!("Opção inválida!");
        }
    }
}
