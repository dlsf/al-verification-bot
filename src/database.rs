use poise::serenity_prelude::UserId;
use rusqlite::Error;
use rusqlite::Connection;

/// Represents an AL account that's linked to a Discord account
pub struct LinkedAccount {
    pub discord_id: u64,
    pub anilist_id: u32
}

/// Sets up the database for initial use
pub fn init_database() -> Result<(), Error> {
    let connection = Connection::open("database.db")?;
    connection.execute("CREATE TABLE IF NOT EXISTS LinkedAccounts (discord_id INT, anilist_id INT, PRIMARY KEY discord_id)", []).expect("Failed to init database.db");
    Ok(())
}

/// Retrieves the account that's linked with the provided Discord account ID, if there is one
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

/// Creates a link between the accounts in the database.
///
/// The boolean return value indicates whether new data was able to be inserted
pub fn link(linked_account: LinkedAccount) -> Result<bool, Error> {
    let connection = Connection::open("database.db")?;
    let mut statement = connection.prepare("INSERT INTO LinkedAccounts VALUES (?, ?) ON CONFLICT DO NOTHING")?;
    
    let updated_row_count = statement.execute([linked_account.discord_id, linked_account.anilist_id as u64])?;
    Ok(updated_row_count != 0)
}
