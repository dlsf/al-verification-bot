use crate::{verification, Data, Error};
use poise::serenity_prelude::{ComponentInteraction, Context, CreateInteractionResponse, CreateInteractionResponseFollowup, CreateInteractionResponseMessage, CreateQuickModal, FullEvent};

pub async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    if let FullEvent::InteractionCreate { interaction } = event {
        if let Some(component_interaction) = interaction.as_message_component() {
            return handle_button_click(component_interaction, ctx, data).await;
        }
    }
    Ok(())
}

async fn handle_button_click(component_interaction: &ComponentInteraction, ctx: &Context, data: &Data) -> Result<(), Error> {
    if &component_interaction.data.custom_id != "verify_button" {
        return Ok(());
    }

    let modal_response = component_interaction.quick_modal(
        &ctx,
        CreateQuickModal::new("Account Verification").short_field("Please enter your authorization code")
    ).await;

    if let Some(modal) = modal_response.unwrap_or(None) {
        let token = modal.inputs.first().unwrap().trim();

        let _ = modal.interaction.create_response(&ctx.http, CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content("Linking account...")
                .ephemeral(true)))
            .await;

        let member = modal.interaction.member.unwrap();
        let verification_result = verification::verify(member.user.id.get(), token.to_string(), &data).await;
        
        if verification_result.is_err() {
            // This should never run
            send_followup("Something really went wrong, please try again later!", &component_interaction, ctx).await;
            println!("{}", verification_result.unwrap_err()); // TODO: Replace with proper logging
            return Ok(())
        }

        if let Some(message) = verification_result? {
            // Something failed gracefully, print the error message
            send_followup(message, &component_interaction, ctx).await;
            return Ok(())
        }

        let role_change = member.add_role(ctx, data.verified_role_id).await;
        if role_change.is_err() {
            send_followup("Failed to grant the verification role, please contact a moderator!", &component_interaction, ctx).await;
            return Ok(())
        }

        send_followup("Your account has been successfully linked!", component_interaction, ctx).await;
    } else {
        send_followup("Failed to get your token, please try again!", component_interaction, ctx).await;
    }

    Ok(())
}

async fn send_followup(text: &str, component_interaction: &ComponentInteraction, ctx: &Context) {
    let _ = component_interaction.create_followup(&ctx.http, CreateInteractionResponseFollowup::new()
        .content(text)
        .ephemeral(true))
        .await;
}
