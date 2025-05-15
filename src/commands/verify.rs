use crate::{anilist, database, Context, Error};
use crate::database::LinkedAccount;

/// Link your AniList account
#[poise::command(slash_command)]
pub async fn verify(ctx: Context<'_>, #[description = "The verification code"] token: String) -> Result<(), Error> {
    let _ = ctx.defer_ephemeral().await;

    let self_user = anilist::get_user_information(token.trim()).await;
    if self_user.is_err() {
        let _ = ctx.reply("Couldn't verify your account, please check your token or try again later!").await;
        return Ok(())
    }

    let link_result = database::link(LinkedAccount {
        discord_id: ctx.author().id.get(),
        anilist_id: self_user?.id as u32
    });

    // A database error occurred
    if link_result.is_err() {
        let _ = ctx.reply("An database error occurred, please try again later!").await;
        println!("{}", link_result.err().unwrap()); // TODO: Replace with proper logging
        return Ok(())
    }
    
    // The user already linked their account
    if !link_result.unwrap() {
        let _ = ctx.reply("You already linked your account!").await;
        return Ok(())
    }

    // Verification successful, give the user the verification Discord role
    let role_result = ctx.author_member().await.unwrap().add_role(ctx.http(), ctx.data().verified_role_id).await;
    if role_result.is_err() {
        let _ = ctx.reply("Failed to grant the verification role, please contact a moderator!").await;
        return Ok(())
    }
    
    let _ = ctx.reply("Your account has been linked!").await;
    Ok(())
}
