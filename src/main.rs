mod modules;
use modules::storage::Storage;
use modules::input;
use modules::lua_bridge;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let database = Rc::new(RefCell::new(Storage::new()));
    let (_lua, extensions) = lua_bridge::load_extensions(Rc::clone(&database));

    input::repl(Rc::clone(&database), extensions);

    // st.inserir(String::from("chave1"), String::from("valor1"));
    // st.inserir(String::from("chave2"), String::from("valor2"));
    // st.inserir(String::from("cpf_zezinho"), String::from("12345678909"));

    // println!("Hello, world!");

    // match st.buscar_por_valor("12345678909") {
    //     Some(chave) => println!("Valor já existe na chave: {}", chave),
    //     None => println!("Valor não encontrado"),
    // }
}