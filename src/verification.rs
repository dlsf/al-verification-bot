use crate::database::LinkedAccount;
use crate::{anilist, database, Data};
use anyhow::Error;
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn verify(user_id: u64, code: String, data: &Data) -> Result<Option<&str>, Error> {
    let self_user = anilist::get_user_information(code.trim(), data).await;
    if self_user.is_err() {
        return Ok(Some("Couldn't verify your account, please get a new token and try again later!"))
    }

    let link_result = database::link(LinkedAccount {
        discord_id: user_id,
        anilist_id: self_user?.id as u32,
        linked_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()
    });

    // A database error occurred
    if link_result.is_err() {
        println!("{}", link_result.err().unwrap()); // TODO: Replace with proper logging
        return Ok(Some("An database error occurred, please try again later!"));
    }

    // The user already linked their account
    if !link_result? {
        return Ok(Some("You already linked your account!"));
    }
    
    Ok(None)
}
