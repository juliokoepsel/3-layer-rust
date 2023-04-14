//Model Layer
struct Carro {
    placa: String,
    horas: i32,
    preco_hora: f32,
}
impl Carro {
    fn new(placa: String, horas: i32, preco_hora: f32) -> Carro {
        Carro{
            placa: placa,
            horas: horas,
            preco_hora: preco_hora,
        }
    }
    fn calcular_preco(&self) -> f32 {
        return self.horas as f32 * &self.preco_hora;
    }
    fn to_string(&self) -> String {
        return format!("Placa: {}, Horas: {}, Preço da Hora: {}, Valor Total: {}", &self.placa, &self.horas, &self.preco_hora, &self.calcular_preco());
    }
}

//Controller Layer
struct Controller;
impl Controller {
    fn insert(lista: &mut Vec<Carro>, placa: String, horas: i32, preco_hora: f32) -> bool {
        if placa.len() > 0 && horas > 0 && preco_hora > 0.0 {
            lista.push(Carro::new(placa, horas, preco_hora));
            return true;
        } else {
            return false;
        }
    }
    fn list(lista: &mut Vec<Carro>) -> bool {
        if lista.len() > 0 {
            return true;
        } else {
            return false;
        }
    }
    fn remove(lista: &mut Vec<Carro>, placa: String) -> bool {
        if placa.len() > 0 && lista.len() > 0 {
            let index = lista.iter().position(|x| x.placa == placa).unwrap();
            lista.remove(index);
            return true;
        } else {
            return false;
        }
    }
}

//View Layer
struct View;
impl View {
    fn menu(lista: &mut Vec<Carro>) {
        loop {
            println!("----------");

            println!("Menu Estacionamento: (1 = Inserir, 2 = Listar, 3 = Remover, 4 = Sair)");
            let mut entrada = String::new();
            io::stdin()
                .read_line(&mut entrada)
                .expect("Erro ao ler a linha");

            let num = entrada.parse::<i32>().unwrap();
            match num {
                1=>View::insert(lista),
                2=>View::list(lista),
                3=>View::remove(lista),
                4=>break,
                i32::MIN..=0_i32 | 5_i32..=i32::MAX => println!("Comando desconhecido."),
            }
        }
    }
    fn insert(lista: &mut Vec<Carro>) {
        println!("----------");
        println!("Inserir carro.");

        println!("Placa do carro:");
        let mut placa = String::new();
        io::stdin()
            .read_line(&mut placa)
            .expect("Erro ao ler a linha");
        let placa = placa.trim().parse::<String>().unwrap();
        
        println!("Quantidade de horas:");
        let mut horas = String::new();
        io::stdin()
            .read_line(&mut horas)
            .expect("Erro ao ler a linha");
        let horas = horas.trim().parse::<i32>().unwrap();
        
        println!("Preço da hora:");
        let mut preco_hora = String::new();
        io::stdin()
            .read_line(&mut preco_hora)
            .expect("Erro ao ler a linha");
        let preco_hora = preco_hora.trim().parse::<f32>().unwrap();

        if Controller::insert(lista, placa, horas, preco_hora) {
            println!("Objeto inserido com sucesso!");
        } else {
            println!("Erro ao inserir objeto!");
        }
    }
    fn list(lista: &mut Vec<Carro>) {
        println!("----------");
        println!("Listar carros.");

        if Controller::list(lista) {
            for elem in lista.iter() {
                println!("{}", elem.to_string());
            }
        } else {
            println!("A lista de objetos está vazia!");
        }
    }
    fn remove(lista: &mut Vec<Carro>) {
        println!("----------");
        println!("Remover carro.");

        println!("Placa do carro:");
        let mut placa = String::new();
        io::stdin()
            .read_line(&mut placa)
            .expect("Erro ao ler a linha");
        let placa = placa.trim().parse::<String>().unwrap();

        if Controller::remove(lista, placa) {
            println!("Objeto removido com sucesso!");
        } else {
            println!("Erro ao remover objeto!");
        }
    }
}

//Main
use std::io;
fn main() {
    println!("START");
    
    let mut lista: Vec<Carro> = Vec::new();

    View::menu(&mut lista);

    println!("END");
}