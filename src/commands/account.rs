use crate::database::LinkedAccount;
use crate::utils::errors::AccountLinkError;
use crate::utils::message;
use crate::{database, Context, Error};
use poise::serenity_prelude::{CreateEmbed, Mentionable, User};

/// Shows a user's linked AniList account
#[poise::command(
    slash_command,
    subcommands("discord", "anilist"),
    default_member_permissions = "MODERATE_MEMBERS"
)]
pub async fn account(_: Context<'_>) -> Result<(), Error> {
    // Never runs
    Ok(())
}

/// Shows the account that's linked to a user's Discord account
#[poise::command(slash_command)]
pub async fn discord(ctx: Context<'_>, #[description = "The Discord user to check"] user: User) -> Result<(), Error> {
    let _ = ctx.defer_ephemeral().await;

    let database_result = database::get_linked_account_discord(user.id);
    handle(ctx, database_result).await
}

/// Shows the account that's linked to a user's AniList account
#[poise::command(slash_command)]
pub async fn anilist(ctx: Context<'_>, #[description = "The AniList user to check"] user_id: u32) -> Result<(), Error> {
    let _ = ctx.defer_ephemeral().await;

    let database_result = database::get_linked_account_anilist(user_id);
    handle(ctx, database_result).await
}

async fn handle(ctx: Context<'_>, linked_account_result: anyhow::Result<LinkedAccount>) -> Result<(), Error> {
    let linked_account = match linked_account_result {
        Ok(account) => account,
        Err(error) => {
            if error.downcast_ref::<AccountLinkError>().is_some() {
                message::err(&ctx, "No linked account!").await;
            } else {
                message::err(&ctx, "Error checking the link status, try again later!").await;
            }
            return Ok(())
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
        .field("Discord", ctx.author().mention().to_string(), false)
        .field("Linked At", format!("<t:{linked_at}:f>"), false);

    message::send(&ctx, embed).await;

    Ok(())
}
