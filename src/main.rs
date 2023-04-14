//Model Layer
struct Carro {
    placa: String,
    horas: i32,
    preco_hora: i32,
}
impl Carro {
    fn new(placa: String, horas: i32, preco_hora: f32) -> Carro {
        Carro{
            placa: placa,
            horas: horas,
            preco_hora: preco_hora,
        }
    }
    fn calcular_horas(&self) -> f32 {
        return &self.horas * &self.preco_hora;
    }
}

//Controller Layer
struct Controller;
impl Controller {
    fn insert(name: String, num1: i32, num2: i32) -> Model {
        //TODO: Check if name.len() > 0
        return Model::new(name, num1, num2);
    }
    fn list(list: Vec<Model>, name: String) -> Model {
        /*
        if list.len() > 0 {
            for i in list {
                if i.name == name {
                    return i;
                }
            }
            panic!("ERROR: NO MODEL FOUND WITH NAME {}!", name);
        } else {
            panic!("ERROR: EMPTY ARRAY!");
        }
        */

    }
    fn remove(model: Model) {

    }
}



//View Layer
struct View;
impl View {
    /*
    fn add(mut list: Vec<Model>) {
        println!("VIEW_ADD= ");

        println!("Enter name:");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        
        println!("Enter num1:");
        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line");
        
        println!("Enter num2:");
        let mut num2 = String::new();
        io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read line");
        
        let model = Model::new(name.trim().parse::<String>().unwrap(), num1.trim().parse::<i32>().unwrap(), num2.trim().parse::<i32>().unwrap());

        list.push(Controller::create(name.trim().parse::<String>().unwrap(), num1.trim().parse::<i32>().unwrap(), num2.trim().parse::<i32>().unwrap()));
        println!("VIEW_ADD_RESULT= Name: {}, Num1: {}, Num2: {}", list[0].name, list[0].num1, list[0].num2);
    }*/
    /*
    fn change() {}*/
    /*
    fn remove() {}*/
    /*
    fn calculate(list: Vec<Model>) {
        println!("VIEW_CALCULATE= ");

        println!("Enter name:");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        
        println!("VIEW_CALCULATE_RESULT= {}", Controller::calculate(list, name));
    }*/
    fn menu() {
        let mut num: i32 = 0;
        loop {
            println!("MENU: (1 = Inserir, 2 = Listar, 3 = Remover, 4 = Sair)");
            let mut entrada = String::new();
            io::stdin()
                .read_line(&mut entrada)
                .expect("Erro ao ler a linha");
            num = entrada.parse::<i32>().unwrap();
            match num{
                4=>break;
            }
        }
    }
}



//Main
use std::io;
fn main() {
    println!("START");
    println!("ESTACIONAMENTO:");
    
    let list: Vec<Carro> = Vec::new();

    View::add(list);
    //View::calculate(list);

    println!("END");
}