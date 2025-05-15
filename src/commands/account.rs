use crate::errors::AccountLinkError;
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
    let linked_account = match database_result {
        Ok(account) => account,
        Err(error) => {
            return if error.downcast_ref::<AccountLinkError>().is_some() {
                let _ = ctx.reply("No linked account!").await;
                Ok(())
            } else {
                let _ = ctx.reply("Error checking the link status, try again later!").await;
                Ok(())
            }
        }
    };
    
    let account_id = linked_account.anilist_id;
    let _ = ctx.reply(format!("Linked Account: https://anilist.co/user/{account_id}")).await;
    Ok(())
}

async fn permission_check(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx.author_member().await.unwrap().permissions.unwrap().moderate_members())
}
