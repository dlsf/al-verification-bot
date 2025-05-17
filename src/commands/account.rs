use crate::errors::AccountLinkError;
use crate::{database, message, Context, Error};
use poise::serenity_prelude::{Color, CreateEmbed, Mentionable};
use poise::serenity_prelude as serenity;

/// Shows a user's linked AniList account
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
                let _ = message::err(ctx, "No linked account!").await;
                Ok(())
            } else {
                let _ = message::err(ctx, "Error checking the link status, try again later!").await;
                Ok(())
            }
        }
    };
    
    let account_id = linked_account.anilist_id;
    let account_url = format!("https://anilist.co/user/{account_id}");
    let linked_at = linked_account.linked_at;

    let embed = CreateEmbed::new()
        .title("**Linked Account**")
        .url(&account_url)
        .color(Color::from_rgb(2, 169, 255))
        .field("Discord", user.mention().to_string(), false)
        .field("AniList", &account_url, false)
        .field("Linked At", format!("<t:{linked_at}:f>"), false);
    
    let _ = message::send(ctx, embed).await;
    Ok(())
}

async fn permission_check(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx.author_member().await.unwrap().permissions.unwrap().moderate_members())
}
