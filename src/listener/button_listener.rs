use log::error;
use crate::utils::errors::AccountLinkError;
use crate::{verification, Data, Error};
use poise::serenity_prelude::{ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage, CreateQuickModal, FullEvent, QuickModalResponse};

pub async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    if let FullEvent::InteractionCreate { interaction } = event {
        if let Some(component_interaction) = interaction.as_message_component() {
            if component_interaction.data.custom_id != "verify_button" {
                return Ok(());
            }

            let modal_response = component_interaction.quick_modal(
                ctx,
                CreateQuickModal::new("Account Verification").short_field("Please enter your authorization code")
            ).await;

            if let Some(modal) = modal_response.unwrap_or(None) {
                return verify_account(modal, component_interaction, ctx, data).await
            } else {
                send_followup("Failed to get your token, please try again!", component_interaction, ctx).await
            }
        }
    }
    Ok(())
}

async fn verify_account(modal: QuickModalResponse, component_interaction: &ComponentInteraction, ctx: &Context, data: &Data) -> Result<(), Error> {
    let token = modal.inputs.first().unwrap().trim();

    let _ = modal.interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .content("Linking account...")
            .ephemeral(true)))
        .await;

    let user_id = modal.interaction.user.id;
    let mut cooldown = data.cooldown.lock().await;
    if cooldown.is_on_cooldown(user_id) {
        send_followup("You have just attempted to verify yourself. Please wait a few minutes before trying again!", component_interaction, ctx).await;
        return Ok(())
    } else {
        cooldown.apply(user_id);
    }

    let member = modal.interaction.member.ok_or(AccountLinkError::Discord)?;
    let verification_result = verification::verify(member.user.id.get(), token.to_string(), data).await;
    if let Some(error) = verification_result.err() {
        if let Some(error_type) = error.downcast_ref::<AccountLinkError>() {
            send_followup(error_type.to_string().as_str(), component_interaction, ctx).await;
        } else {
            // Should never run
            error!("Encountered unknown error while verifying user: {}", error);
            send_followup("Something really went wrong, please try again later!", component_interaction, ctx).await;
        }

        return Ok(())
    }

    let role_change = member.add_role(ctx, data.verified_role_id).await;
    if role_change.is_err() {
        error!("Failed to grant verification role: {}", role_change.unwrap_err());
        send_followup("Failed to grant the verification role, please contact a moderator!", component_interaction, ctx).await;
        return Ok(())
    }

    send_followup("Your account has been successfully linked!", component_interaction, ctx).await;
    Ok(())
}

async fn send_followup(text: &str, component_interaction: &ComponentInteraction, ctx: &Context) {
    let _ = component_interaction.create_followup(&ctx.http, CreateInteractionResponseFollowup::new()
        .content(text)
        .ephemeral(true))
        .await;
}
