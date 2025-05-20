use crate::database::LinkedAccount;
use crate::utils::errors::AccountLinkError;
use crate::{anilist, database, Data};
use anyhow::{Error, Result};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use log::{error, warn};

pub async fn verify(user_id: u64, code: String, data: &Data) -> Result<()> {
    let self_user_result = anilist::get_user_information(code.trim(), data).await;
    if self_user_result.is_err() {
        warn!("Couldn't verify user, this is most likely not an error: {}", self_user_result.err().unwrap());
        return Err(Error::new(AccountLinkError::Anilist))
    }
    
    let self_user = self_user_result?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    check_account_age(data, self_user.created_at.unwrap_or(0) as u64, now).await?;
    
    let link_result = database::link(LinkedAccount {
        discord_id: user_id,
        anilist_id: self_user.id as u32,
        linked_at: now
    });

    // A database error occurred
    if link_result.is_err() {
        error!("Failed to verify user due to database error: {}", link_result.err().unwrap());
        return Err(Error::new(AccountLinkError::Database))
    }

    // The user already linked their account
    if !link_result? {
        return Err(Error::new(AccountLinkError::AlreadyLinked));
    }
    
    Ok(())
}

async fn check_account_age(data: &Data, created_at: u64, now: u64) -> Result<()> {
    let account_age = Duration::from_secs(now - created_at);
    if account_age >= data.minimum_account_age {
        return Ok(())
    }

    let remaining_time_hours = (data.minimum_account_age.abs_diff(account_age).as_secs() as f64 / 60.0 / 60.0).ceil() as u32;
    Err(Error::new(AccountLinkError::AccountAge {remaining_time_hours}))
}

