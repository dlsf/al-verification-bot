use crate::utils::errors::AccountLinkError;
use anyhow::Result;
use poise::serenity_prelude::UserId;
use rusqlite::Connection;

/// Represents an AL account that's linked to a Discord account
pub struct LinkedAccount {
    pub discord_id: u64,
    pub anilist_id: u32,
    pub linked_at: u64
}

/// Sets up the database for initial use
pub fn init_database() -> Result<()> {
    let connection = Connection::open("database.db")?;
    connection.execute("CREATE TABLE IF NOT EXISTS LinkedAccounts (discord_id INT PRIMARY KEY, anilist_id INT UNIQUE, linked_at INT)", []).expect("Failed to init database.db");
    Ok(())
}

/// Retrieves the account that's linked with the provided Discord account ID,
pub fn get_linked_account_discord(discord_id: UserId) -> Result<LinkedAccount> {
    let connection = Connection::open("database.db")?;
    let mut statement = connection.prepare("SELECT * FROM LinkedAccounts WHERE discord_id = ?")?;

    let mut rows = statement.query([discord_id.get()])?;
    let row = rows.next()?.ok_or(AccountLinkError::NotLinked)?;

    Ok(LinkedAccount {
        discord_id: row.get("discord_id")?,
        anilist_id: row.get("anilist_id")?,
        linked_at: row.get("linked_at")?
    })
}

pub fn get_linked_account_anilist(anilist_id: u32) -> Result<LinkedAccount> {
    let connection = Connection::open("database.db")?;
    let mut statement = connection.prepare("SELECT * FROM LinkedAccounts WHERE anilist_id = ?")?;

    let mut rows = statement.query([anilist_id])?;
    let row = rows.next()?.ok_or(AccountLinkError::NotLinked)?;

    Ok(LinkedAccount {
        discord_id: row.get("discord_id")?,
        anilist_id: row.get("anilist_id")?,
        linked_at: row.get("linked_at")?
    })
}

/// Creates a link between the accounts in the database.
///
/// The boolean return value indicates whether new data was able to be inserted
pub fn link(linked_account: LinkedAccount) -> Result<bool> {
    let connection = Connection::open("database.db")?;
    let mut statement = connection.prepare("INSERT INTO LinkedAccounts VALUES (?, ?, ?) ON CONFLICT DO NOTHING")?;
    
    let updated_row_count = statement.execute([linked_account.discord_id, linked_account.anilist_id as u64, linked_account.linked_at])?;
    Ok(updated_row_count != 0)
}

pub fn unlink_account_discord(discord_id: UserId) -> Result<bool> {
    let connection = Connection::open("database.db")?;
    let mut statement = connection.prepare("DELETE FROM LinkedAccounts WHERE discord_id = ?")?;

    let rows = statement.execute([discord_id.get()])?;
    Ok(rows == 1)
}

pub fn unlink_account_anilist(anilist_id: u32) -> Result<bool> {
    let connection = Connection::open("database.db")?;
    let mut statement = connection.prepare("DELETE FROM LinkedAccounts WHERE anilist_id = ?")?;

    let rows = statement.execute([anilist_id])?;
    Ok(rows == 1)
}
