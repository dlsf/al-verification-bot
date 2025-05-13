use crate::{Context, Error};

/// Link your AL account
#[poise::command(slash_command)]
pub async fn verify(ctx: Context<'_>, #[description = "The verification code"] code: String) -> Result<(), Error> {
    let _ = ctx.defer_ephemeral().await;
    let _ = ctx.reply(format!("The code is {code}")).await;
    println!("The code is {code}");
    let mut data = ctx.data().tokens.lock().await;
    (*data).insert(ctx.author().id.get(), code);
    Ok(())
}
