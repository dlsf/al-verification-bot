use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Shows a user's linked AL account
#[poise::command(
    slash_command,
    check = "permission_check"
)]
pub async fn account(ctx: Context<'_>, #[description = "The user to check"] user: serenity::User) -> Result<(), Error> {
    let _ = ctx.defer_ephemeral().await;
    let data = ctx.data().tokens.lock().await;
    let default = &String::from("None");
    let token = data.get(&user.id.get()).unwrap_or(default);
    let _ = ctx.reply(format!("User token: {token}")).await;
    Ok(())
}

async fn permission_check(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx.author_member().await.unwrap().permissions.unwrap().moderate_members())
}
