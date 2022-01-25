#![allow(unused_variables, clippy::too_many_arguments)]

use crate::{error::Error, handle_error, util::calculate_permissions};
use codec::Decode;
use serenity::{
    http::Http,
    model::{channel::ChannelType as SerenityChannelType, prelude::*},
};
use std::{collections::HashMap, sync::Arc};
use subxt::{
    sp_runtime::AccountId32, ClientBuilder, DefaultConfig, DefaultExtra, EventSubscription,
};
use tokio::spawn;

#[subxt::subxt(runtime_metadata_path = "../blockchain/metadata.scale")]
pub mod polkadot {}

use polkadot::{
    discord::events::{
        ChannelCreated, ChannelDeleted, MemberAdded, MemberBanned, RoleAssigned, RoleCreated,
        RoleDeleted,
    },
    runtime_types::pallet_discord::primitives::ChannelType,
};

pub async fn handler(http: Arc<Http>) -> Result<(), Error> {
    env_logger::init();

    let api: polkadot::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>> =
        ClientBuilder::new()
            .set_url("ws://127.0.0.1:9944")
            .build()
            .await
            .map_err(Error::Subxt)?
            .to_runtime_api();

    spawn(async move {
        let http = http.clone();

        // TODO: Also add channels and roles hashmap, reducing Discord API usage
        let mut members: HashMap<AccountId32, u64> = HashMap::new();

        let sub = api
            .client
            .rpc()
            .subscribe_events()
            .await
            .map_err(Error::Subxt)
            .unwrap();
        let decoder = api.client.events_decoder();
        let mut sub = EventSubscription::<DefaultConfig>::new(sub, decoder);
        loop {
            if let Some(sub_result) = sub.next().await {
                match sub_result {
                    Ok(raw) => {
                        if raw.pallet == *"Discord" {
                            handle_error! {
                                match raw.variant.as_str() {
                                    "MemberBanned" => {
                                        let data = <MemberBanned as Decode>::decode(&mut &raw.data[..]);
                                        match data {
                                            Ok(data) => member_banned(&http, &mut members, data).await,
                                            Err(why) => Err(Error::Decode(why)),
                                        }
                                    }
                                    "RoleCreated" => {
                                        let data = <RoleCreated as Decode>::decode(&mut &raw.data[..]);
                                        match data {
                                            Ok(data) => role_created(&http, data).await,
                                            Err(why) => Err(Error::Decode(why)),
                                        }
                                    }
                                    "RoleAssigned" => {
                                        let data = <RoleAssigned as Decode>::decode(&mut &raw.data[..]);
                                        match data {
                                            Ok(data) => role_assigned(&http, &mut members, data).await,
                                            Err(why) => Err(Error::Decode(why)),
                                        }
                                    }
                                    "RoleDeleted" => {
                                        let data = <RoleDeleted as Decode>::decode(&mut &raw.data[..]);
                                        match data {
                                            Ok(data) => role_deleted(&http, data).await,
                                            Err(why) => Err(Error::Decode(why)),
                                        }
                                    }
                                    "MemberAdded" => {
                                        let data = <MemberAdded as Decode>::decode(&mut &raw.data[..]);
                                        match data {
                                            Ok(data) => member_added(&mut members, data).await,
                                            Err(why) => Err(Error::Decode(why)),
                                        }
                                    }
                                    "ChannelCreated" => {
                                        let data =
                                            <ChannelCreated as Decode>::decode(&mut &raw.data[..]);
                                        match data {
                                            Ok(data) => channel_created(&http, data).await,
                                            Err(why) => Err(Error::Decode(why)),
                                        }
                                    }
                                    "ChannelDeleted" => {
                                        let data =
                                            <ChannelDeleted as Decode>::decode(&mut &raw.data[..]);
                                        match data {
                                            Ok(data) => channel_deleted(&http, data).await,
                                            Err(why) => Err(Error::Decode(why)),
                                        }
                                    }
                                    _ => Err(Error::Custom(String::from(
                                        "Unhandled socket event received",
                                    ))),
                                }
                            }
                        }
                    }

                    Err(why) => eprintln!("{:?}", Error::Subxt(why)),
                }
            }
        }
    });

    Ok(())
}

// Won't actually ban until testing phase
async fn member_banned(
    http: &Http,
    members: &mut HashMap<AccountId32, u64>,
    event: MemberBanned,
) -> Result<(), Error> {
    let reason = String::from_utf8(event.2).map_err(Error::UTF8)?;

    // TEMP: substitute "ban" snippet
    ChannelId::from(930077545020407821)
        .send_message(http, |message| {
            message.content(format!(
                "Banning member: <@{}>\nwith reason: {}",
                event.1, reason,
            ))
        })
        .await
        .map_err(Error::Serenity)?;

    /*
    let guild = GuildId::from(930077545020407818);
    if let Some(member_id) = members.get(&event.0) {
        guild
            .ban_with_reason(
                http,
                UserId::from(*member_id),
                /* TODO: dmd field in pallet-discord */ 0,
                reason,
            )
            .await
            .map_err(Error::Serenity)?;
        members.remove(&event.0);
    } else {
        guild
            .ban_with_reason(http, UserId::from(event.1), 0, reason)
            .await
            .map_err(Error::Serenity)?;
    }*/

    Ok(())
}

async fn member_added(
    members: &mut HashMap<AccountId32, u64>,
    event: MemberAdded,
) -> Result<(), Error> {
    members.insert(event.0, event.1);
    Ok(())
}

async fn role_created(http: &Http, event: RoleCreated) -> Result<(), Error> {
    let permissions = calculate_permissions(event.4);
    let name = String::from_utf8(event.0).map_err(Error::UTF8)?;

    GuildId::from(930077545020407818)
        .create_role(http, |role| {
            role.name(name)
                .colour(event.1)
                .hoist(event.2)
                .position(event.3)
                .permissions(permissions)
                .mentionable(event.5)
        })
        .await
        .map_err(Error::Serenity)?;
    // TODO: Send message to log/events channel
    Ok(())
}

async fn role_assigned(
    http: &Http,
    members: &mut HashMap<AccountId32, u64>,
    event: RoleAssigned,
) -> Result<(), Error> {
    let guild = GuildId::from(930077545020407818);
    let mut member = if let Some(member_id) = members.get(&event.0) {
        guild
            .member(http, UserId::from(*member_id))
            .await
            .map_err(Error::Serenity)?
    } else {
        members.insert(event.0, event.1);
        guild
            .member(http, UserId::from(event.1))
            .await
            .map_err(Error::Serenity)?
    };
    let role_name = String::from_utf8(event.2).map_err(Error::UTF8)?;
    if let Some(role) = guild
        .roles(http)
        .await
        .map_err(Error::Serenity)?
        .values()
        .find(|p| p.name == role_name)
    {
        member
            .add_role(http, role.id)
            .await
            .map_err(Error::Serenity)?;
    }

    Ok(())
}

async fn role_deleted(http: &Http, event: RoleDeleted) -> Result<(), Error> {
    let role_name = String::from_utf8(event.0).map_err(Error::UTF8)?;
    let guild = GuildId::from(930077545020407818);
    if let Some(role) = guild
        .roles(http)
        .await
        .map_err(Error::Serenity)?
        .values()
        .find(|r| r.name == role_name)
    {
        guild
            .delete_role(http, role.id)
            .await
            .map_err(Error::Serenity)?;
    } else {
        return Err(Error::Custom(String::from("404 Role not found")));
    }

    Ok(())
}

async fn channel_created(http: &Http, event: ChannelCreated) -> Result<(), Error> {
    let name = String::from_utf8(event.0).map_err(Error::UTF8)?;
    let topic = String::from_utf8(event.4).map_err(Error::UTF8)?;
    GuildId::from(930077545020407818)
        .create_channel(http, |c| {
            let mut channel = c
                .name(name)
                .kind(match event.1 {
                    ChannelType::GUILD_TEXT => SerenityChannelType::Text,
                    ChannelType::GUILD_VOICE => SerenityChannelType::Voice,
                    ChannelType::GUILD_CATEGORY => SerenityChannelType::Category,
                    ChannelType::GUILD_NEWS => SerenityChannelType::News,
                    ChannelType::GUILD_NEWS_THREAD => SerenityChannelType::NewsThread,
                    ChannelType::GUILD_PUBLIC_THREAD => SerenityChannelType::PublicThread,
                    ChannelType::GUILD_PRIVATE_THREAD => SerenityChannelType::PrivateThread,
                    ChannelType::GUILD_STAGE_VOICE => SerenityChannelType::Stage,
                })
                .position(event.2)
                // TODO: .permissions(event.3)
                .topic(topic)
                .nsfw(event.5);

            if let Some(bitrate) = event.6 {
                channel = channel.bitrate(bitrate);
            }
            if let Some(user_limit) = event.7 {
                channel = channel.user_limit(user_limit);
            }
            if let Some(rate_limit) = event.8 {
                channel = channel.rate_limit(rate_limit);
            }
            if let Some(parent) = event.9 {
                channel = channel.category(ChannelId::from(parent));
            }
            channel
        })
        .await
        .map_err(Error::Serenity)?;
    Ok(())
}

async fn channel_deleted(http: &Http, event: ChannelDeleted) -> Result<(), Error> {
    let channel_name = String::from_utf8(event.0).map_err(Error::UTF8)?;
    if let Some(channel) = GuildId::from(930077545020407818)
        .channels(http)
        .await
        .map_err(Error::Serenity)?
        .values()
        .find(|c| c.name == channel_name)
    {
        channel.delete(http).await.map_err(Error::Serenity)?;
        Ok(())
    } else {
        Err(Error::Custom(String::from("404 Channel Not Found")))
    }
}
