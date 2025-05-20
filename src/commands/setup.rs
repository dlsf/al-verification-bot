use crate::utils::message;
use crate::{Context, Error};
use poise::serenity_prelude::{CreateActionRow, CreateButton, CreateEmbed, CreateMessage, GuildChannel, ReactionType};

/// Sends a message explaining the verification process
#[poise::command(
    slash_command,
    default_member_permissions = "ADMINISTRATOR"
)]
pub async fn setup(ctx: Context<'_>, #[description = "The channel to use"] channel: GuildChannel) -> Result<(), Error> {
    let _ = ctx.defer_ephemeral().await;

    let client_id = ctx.data().client_id;
    let auth_url = format!("https://anilist.co/api/v2/oauth/authorize?client_id={client_id}&redirect_uri=https://anilist.co/api/v2/oauth/pin&response_type=code");
    let _ = channel.send_message(ctx.http(), CreateMessage::new()
        .embed(CreateEmbed::new()
            .title("**Verify your account**")
            .description(format!("To combat spam, we require you to link your AniList account with your Discord account.\n
            \
            Please **get an authorization token from [here]({auth_url})** and then **click the button below** to initiate the verification process."))
            .color(message::primary_color()))
        .components(vec![CreateActionRow::Buttons(vec![
            CreateButton::new("verify_button")
                .label("Link your account")
                .emoji(ReactionType::Unicode(String::from("🔗")))
        ])])).await;

    let _ = ctx.reply("Created explanation message!").await;
    Ok(())
}
