use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply_ping(&ctx.http, "pong").await?;
    Ok(())
}
