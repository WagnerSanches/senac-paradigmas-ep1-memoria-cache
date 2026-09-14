use mlua::Lua;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;
use super::storage::Storage;

pub struct Extension {
    pub prefixo: String,
    pub insert: Option<mlua::Function>,
    pub select: Option<mlua::Function>,
}

pub fn load_extensions(database: Rc::<RefCell::<Storage>>) -> (Lua, Vec::<Extension>){
    let lua = Lua::new();
    let mut extensions = Vec::<Extension>::new();

    let database_clone = Rc::clone(&database);

    let database_find_by_value = lua.create_function(move |_, chave: String| {
        let db = database_clone.borrow();
        match db.select_by_value(&chave) {
            Some(valor) => {
                Ok(Some(valor.clone()))
            },
            None => {
                Ok(None)
            },
        }
    }).unwrap();

    lua.globals().set("database_find_by_value", database_find_by_value).unwrap();

    fs::read_dir("extensions")
        .unwrap()
        .filter_map(Result::ok)
        .for_each(|entry| {
            if let Some(extension) = entry.path().extension() {
                if extension == "lua" {
                    let script = fs::read_to_string(entry.path()).unwrap();

                    let table: mlua::Table = lua.load(&script).eval().unwrap();

                    let prefixo: String = table.get("prefixo").unwrap();
                    println!("Prefixo: {}", prefixo);
                    //println!("Arquivo Lua encontrado: {:?}", entry.path());
                    extensions.push(Extension {
                        prefixo,
                        insert: table.get("insert").unwrap(),
                        select: table.get("select").unwrap(),
                    });

                    //let _ = table.get::<mlua::Function>("insert").unwrap().call::<String>(("chave_teste", "valor_teste")).unwrap();
                }
            }
        });
    println!("Total de extensões carregadas: {}", extensions.len());

    return (lua, extensions);
}