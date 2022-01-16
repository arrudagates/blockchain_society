#![allow(non_camel_case_types)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct GuildMember<AccountId> {
    pub account: AccountId,
    pub roles: Vec<Vec<u8>>,
    pub deaf: bool,
    pub mute: bool,
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Role<RoleId> {
    pub id: Option<RoleId>,
    pub name: Vec<u8>,
    pub color: u64,
    pub hoist: bool,
    pub position: u64,
    pub permissions: Vec<Permissions>,
    pub managed: bool,
    pub mentionable: bool,
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Guild<DiscordId> {
    pub id: DiscordId,
    pub name: Vec<u8>,
    pub icon: Vec<u8>,
    pub owner_id: DiscordId,
    pub afk_channel_id: DiscordId,
    pub afk_timeout: u64,
    pub verification_level: u64,
    pub default_message_notifications: u64,
    pub explicit_content_filter: u64,
    pub rules_channel_id: DiscordId,
    pub system_channel_id: DiscordId,
}

#[derive(Encode, Decode, TypeInfo)]
pub struct Channel<AccountId, DiscordId> {
    pub id: DiscordId,
    pub channe_type: ChannelType,
    pub position: u64,
    pub permission_overwrites: Vec<(AccountId, Vec<Permissions>)>,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum ChannelType {
    GUILD_TEXT,
    GUILD_VOICE,
    GUILD_CATEGORY,
    GUILD_NEWS,
    GUILD_NEWS_THREAD,
    GUILD_PUBLIC_THREAD,
    GUILD_PRIVATE_THREAD,
    GUILD_STAGE_VOICE,
}

#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, Debug)]
pub enum Permissions {
    CREATE_INSTANT_INVITE,
    KICK_MEMBERS,
    BAN_MEMBERS,
    ADMINISTRATOR,
    MANAGE_CHANNELS,
    MANAGE_GUILD,
    ADD_REACTIONS,
    VIEW_AUDIT_LOG,
    PRIORITY_SPEAKER,
    STREAM,
    VIEW_CHANNEL,
    SEND_MESSAGES,
    SEND_TTS_MESSAGES,
    MANAGE_MESSAGES,
    EMBED_LINKS,
    ATTACH_FILES,
    READ_MESSAGE_HISTORY,
    MENTION_EVERYONE,
    USE_EXTERNAL_EMOJIS,
    VIEW_GUILD_INSIGHTS,
    CONNECT,
    SPEAK,
    MUTE_MEMBERS,
    DEAFEN_MEMBERS,
    MOVE_MEMBERS,
    USE_VAD,
    CHANGE_NICKNAME,
    MANAGE_NICKNAMES,
    MANAGE_ROLES,
    MANAGE_WEBHOOKS,
    MANAGE_EMOJIS_AND_STICKERS,
    USE_APPLICATION_COMMANDS,
    REQUEST_TO_SPEAK,
    MANAGE_EVENTS,
    MANAGE_THREADS,
    CREATE_PUBLIC_THREADS,
    CREATE_PRIVATE_THREADS,
    USE_EXTERNAL_STICKERS,
    SEND_MESSAGES_IN_THREADS,
    START_EMBEDDED_ACTIVITIES,
    MODERATE_MEMBERS,
}
