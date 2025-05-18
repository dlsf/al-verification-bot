use crate::utils::message;
use crate::{verification, Context, Error};

/// Link your AniList account
#[poise::command(slash_command)]
pub async fn verify(ctx: Context<'_>, #[description = "The verification code"] token: String) -> Result<(), Error> {
    let _ = ctx.defer_ephemeral().await;

    let verification_result = verification::verify(ctx.author().id.get(), token, ctx.data()).await;
    if verification_result.is_err() {
        // This should never run
        message::err(ctx, "Something really went wrong, please try again later!").await;
        println!("{}", verification_result.unwrap_err()); // TODO: Replace with proper logging
        return Ok(())
    }

    if let Some(message) = verification_result? {
        // Something failed gracefully, print the error message
        message::err(ctx, message).await;
        return Ok(())
    }

    let role_change = ctx.author_member().await.unwrap().add_role(ctx, ctx.data().verified_role_id).await;
    if role_change.is_err() {
        message::err(ctx, "Failed to grant the verification role, please contact a moderator!").await;
        return Ok(())
    }
    
    message::ok(ctx, "Your account has been successfully linked!").await;
    Ok(())
}
