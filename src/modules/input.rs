use std::io::{self, Write};
use super::storage::Storage;
use super::parser;
use super::parser::Command;
use super::lua_bridge::Extension;
use std::rc::Rc;
use std::cell::RefCell;


pub fn repl(database: Rc::<RefCell::<Storage>>, extensions: Vec::<Extension>) {
    loop {
        print!("> ");
        io::stdout().flush().unwrap(); 
        
        let mut input = String::new();
        let read_bytes = io::stdin().read_line(&mut input).expect("Falha ao ler a entrada!");
        
        if read_bytes == 0 {
            break;
        }

        let comando = parser::parse(&input.trim());
        match comando {
            Command::Exit => break,
            Command::Add(k, v) => {
                let extension = extensions.iter().find(|ext| k.starts_with(&ext.prefixo));

                match extension {
                    Some(ext) => {
                        if let Some(insert_func) = &ext.insert {
                            let (sucesso, result): (bool, String) = insert_func.call((k.clone(), v.clone())).unwrap();

                            if sucesso {
                                database.borrow_mut().insert(k, result);
                                println!("OK");
                            } else {
                                println!("ERRO: {}", result);
                            }

                        } else {
                            println!("ERRO: Função 'insert' não encontrada na extensão para o prefixo '{}'", ext.prefixo);
                        }
                    },
                    None => {
                        database.borrow_mut().insert(k, v);
                        println!("OK");
                    }
                }
            },
            Command::Get(chave) => {
                let valor_bruto = database.borrow().select(&chave).cloned();

                match valor_bruto {
                    None => println!("ERRO: chave inexistente"),
                    Some(valor) => {
                        let extension = extensions.iter().find(|ext| chave.starts_with(&ext.prefixo));

                        match extension {
                            Some(ext) => {
                                if let Some(select_func) = &ext.select {
                                    let (sucesso, result): (bool, String) = select_func.call((valor.clone())).unwrap();

                                    if sucesso {
                                        println!("{}", result);
                                    } else {
                                        println!("ERRO: {}", result);
                                    }
                                } else {
                                    println!("{}", valor);
                                }
                            },
                            None => println!("{}", valor),
                        }
                    }
                }
            },
            Command::Error(msg) => println!("ERRO: {}", msg),
        }
    }

}