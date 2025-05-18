use crate::utils::errors::AccountLinkError;
use crate::utils::message;
use crate::{database, Context, Error};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{CreateEmbed, Mentionable};

/// Shows a user's linked AniList account
#[poise::command(
    slash_command,
    default_member_permissions = "MODERATE_MEMBERS"
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
        .color(message::primary_color())
        .field("AniList", &account_url, false)
        .field("Discord", user.mention().to_string(), false)
        .field("Linked At", format!("<t:{linked_at}:f>"), false);

    let _ = message::send(ctx, embed).await;
    Ok(())
}
