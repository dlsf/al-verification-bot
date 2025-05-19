use crate::Context;
use poise::serenity_prelude::{Color, CreateEmbed};
use poise::CreateReply;

pub async fn send(ctx: &Context<'_>, embed: CreateEmbed) {
    let _ = ctx.send(CreateReply::default().embed(embed).reply(true).ephemeral(true)).await;
}

pub async fn err(ctx: &Context<'_>, body: &str) {
    let embed = CreateEmbed::new()
        .title("Error")
        .description(body)
        .color(Color::RED);
    
    send(ctx, embed).await
}

pub async fn ok(ctx: &Context<'_>, body: &str) {
    let embed = CreateEmbed::new()
        .description(body)
        .color(primary_color());

    send(ctx, embed).await
}

pub fn primary_color() -> Color {
    Color::from_rgb(2, 169, 255)
}
