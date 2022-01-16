use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{channel::Message, id::RoleId},
};

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply_ping(&ctx.http, "pong").await?;
    Ok(())
}

#[command]
#[owners_only]
#[aliases("gives0raadmin", "givegabeadmin")]
async fn givemeadmin(ctx: &Context, msg: &Message) -> CommandResult {
    let role = RoleId::from(930086188721770586);
    msg.member(&ctx.http)
        .await?
        .add_role(&ctx.http, role)
        .await?;
    msg.reply_ping(&ctx.http, "Ok, you're admin").await?;
    Ok(())
}

#[command]
#[owners_only]
#[aliases(
    "gibmember",
    "removegabeadmin",
    "removes0raadmin",
    "rms0radmin",
    "rmgabeadmin"
)]
async fn removemyadmin(ctx: &Context, msg: &Message) -> CommandResult {
    let role = RoleId::from(930086188721770586);
    msg.member(&ctx.http)
        .await?
        .remove_role(&ctx.http, role)
        .await?;
    msg.reply_ping(&ctx.http, "Ok, you're admin").await?;
    Ok(())
}
