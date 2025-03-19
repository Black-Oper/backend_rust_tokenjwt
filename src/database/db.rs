use rusqlite::{Connection, Result};

pub fn init_db() -> Result<Connection> {
    // Abre ou cria o arquivo do banco de dados
    let conn = Connection::open("banco.db")?;

    // Cria a tabela de usuários caso ela não exista
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}
