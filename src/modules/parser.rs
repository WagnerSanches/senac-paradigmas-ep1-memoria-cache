#[derive(Debug)]
pub enum Command {
    Add(String, String), 
    Get(String),         
    Exit,
    Error(String),
}

pub fn parse(command: &str) -> Command {

    if command.trim().is_empty() {
        return Command::Error("Input vazio!".to_string());
    }
    let mut args = command.splitn(3, ' ');

    match args.next() {
        Some("ADD") => {
            let k = args.next().unwrap_or("").to_string();
            let v = args.next().unwrap_or("").to_string();

            if k.is_empty() || v.is_empty() {
                return Command::Error("O Input ADD requer uma chave e um valor validos!".to_string());
            } 
            
            Command::Add(k, v)
        }
        Some("GET") => {
            let k = args.next().unwrap_or("").to_string();

            if k.is_empty() {
                return Command::Error("O Input GET requer uma chave valida!".to_string());
            }
            
            Command::Get(k)
        }
        Some("EXIT") => Command::Exit,
        _ => Command::Error("O Input contém um comando inválido!".to_string()),
    }
}