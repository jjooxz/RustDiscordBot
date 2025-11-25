use std::fs;
use std::io::{self, Write};

pub async fn pegar_proximo_id() -> io::Result<u32> {
    // Lê o último ID do arquivo
    let last_id = fs::read_to_string("lastid.txt").unwrap_or_else(|_| "0".to_string());
    let mut id: u32 = last_id.trim().parse().unwrap_or(0);

    // Incrementa
    id += 1;

    // Salva de volta
    let mut file = fs::File::create("lastid.txt")?;
    writeln!(file, "{}", id)?;

    Ok(id)
}