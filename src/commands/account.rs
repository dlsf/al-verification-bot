use crate::{database, Context, Error};
use poise::serenity_prelude as serenity;

/// Shows a user's linked AL account
#[poise::command(
    slash_command,
    check = "permission_check"
)]
pub async fn account(ctx: Context<'_>, #[description = "The user to check"] user: serenity::User) -> Result<(), Error> {
    let _ = ctx.defer_ephemeral().await;
    
    let database_result = database::get_linked_account(user.id);
    if database_result.is_err() {
        let _ = ctx.reply("Couldn't check the linked account!").await;
        return Ok(())
    }
    
    let account_id_option = database_result.unwrap();
    if account_id_option.is_none() {
        let _ = ctx.reply("No linked account!").await;
        return Ok(())
    }
    
    let account_id = account_id_option.unwrap().anilist_id;
    let _ = ctx.reply(format!("Linked Account: https://anilist.co/user/{account_id}")).await;
    Ok(())
}

async fn permission_check(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx.author_member().await.unwrap().permissions.unwrap().moderate_members())
}
