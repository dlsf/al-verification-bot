use crate::database::LinkedAccount;
use crate::utils::message;
use crate::{database, Context, Error};
use poise::serenity_prelude::User;
use std::time::{SystemTime, UNIX_EPOCH};

/// Manually link a user's account
#[poise::command(
    slash_command,
    default_member_permissions = "MODERATE_MEMBERS"
)]
pub async fn link(ctx: Context<'_>, user: User, anilist_id: u32) -> Result<(), Error> {
    let link_result = database::link(LinkedAccount {
        discord_id: user.id.get(),
        anilist_id,
        linked_at: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
    });

    if link_result.is_err() {
        message::err(&ctx, "Failed to update the database!").await;
        return Ok(())
    }

    if !link_result? {
        message::err(&ctx, "This user already has a linked account, please unlink it first!").await;
    }

    message::ok(&ctx, "Successfully linked the account!").await;
    Ok(())
}
