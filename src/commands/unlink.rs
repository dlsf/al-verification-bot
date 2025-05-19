use crate::utils::message;
use crate::{database, Context, Error};
use poise::serenity_prelude::User;

/// Manually unlink a user's account
#[poise::command(
    slash_command,
    subcommands("discord", "anilist"),
    default_member_permissions = "MODERATE_MEMBERS"
)]
pub async fn unlink(_: Context<'_>) -> Result<(), Error> {
    // Never runs
    Ok(())
}

/// Manually unlink someone's account via their Discord account
#[poise::command(slash_command)]
pub async fn discord(ctx: Context<'_>, #[description = "The Discord user to unlink"] user: User) -> Result<(), Error> {
    let _ = ctx.defer_ephemeral().await;

    let database_result = database::unlink_account_discord(user.id);
    handle(ctx, database_result).await
}

/// Manually unlink someone's account via their AniList ID
#[poise::command(slash_command)]
pub async fn anilist(ctx: Context<'_>, #[description = "The AniList user to unlink"] user_id: u32) -> Result<(), Error> {
    let _ = ctx.defer_ephemeral().await;

    let database_result = database::unlink_account_anilist(user_id);
    handle(ctx, database_result).await
}

async fn handle(ctx: Context<'_>, database_result: anyhow::Result<bool>) -> Result<(), Error> {
    if database_result.is_err() {
        message::err(&ctx, "Failed to update the database!").await;
        return Ok(())
    }
    
    if !database_result? {
        message::err(&ctx, "Didn't find a link for this user!").await;
        return Ok(())
    }
    
    message::ok(&ctx, "The account has been unlinked, but their role has to be removed manually if necessary").await;
    Ok(())
}