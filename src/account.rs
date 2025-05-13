use rusqlite::Error;
use rusqlite::Connection;
use serenity::all::UserId;

pub struct LinkedAccount {
    pub discord_id: u64,
    pub anilist_id: u32
}

pub fn init_database() -> Result<(), Error> {
    let connection = Connection::open("database.db")?;
    connection.execute("CREATE TABLE IF NOT EXISTS LinkedAccounts (discord_id INT, anilist_id INT, PRIMARY KEY (discord_id, anilist_id))", []).expect("Failed to init database.db");
    Ok(())
}

pub fn get_linked_account(discord_id: UserId) -> Result<Option<LinkedAccount>, Error> {
    let connection = Connection::open("database.db")?;
    let mut statement = connection.prepare("SELECT * FROM LinkedAccounts WHERE discord_id = ?")?;
    
    let mut result = statement.query([discord_id.get()])?;
    result.next().map(|row| {
        row?;
        
        Some(LinkedAccount {
            discord_id: row.unwrap().get("discord_id").unwrap(),
            anilist_id: row.unwrap().get("anilist_id").unwrap()
        })
    })
}

pub fn link(linked_account: LinkedAccount) -> Result<Option<()>, Error> {
    let connection = Connection::open("database.db")?;
    let mut statement = connection.prepare("INSERT INTO LinkedAccounts VALUES (?, ?) ON CONFLICT DO NOTHING")?;
    
    let updated_rows = statement.execute([linked_account.discord_id, linked_account.anilist_id as u64])?;
    if updated_rows == 0 {
        Ok(None)
    } else {
        Ok(Some(()))
    }
}
