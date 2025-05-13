use crate::{account, Context, Error};
use crate::account::LinkedAccount;

/// Link your AL account
#[poise::command(slash_command)]
pub async fn verify(ctx: Context<'_>, #[description = "The verification code"] code: String) -> Result<(), Error> {
    let _ = ctx.defer_ephemeral().await;
    
    let link_result = account::link(LinkedAccount {
        discord_id: ctx.author().id.get(),
        anilist_id: 123 // TODO: Fetch and use actual anilist ID
    });

    // A database error occurred
    if link_result.is_err() {
        let _ = ctx.reply("An error occurred, please try again later!").await;
        println!("{}", link_result.err().unwrap()); // TODO: Replace with proper logging
        return Ok(())
    }
    
    // The user already linked their account
    if link_result.unwrap().is_none() {
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
