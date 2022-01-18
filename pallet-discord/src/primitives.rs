#![allow(non_camel_case_types)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[derive(Encode, Decode, TypeInfo, Clone)]
pub struct GuildMember<AccountId, DiscordId> {
    pub id: DiscordId,
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
    pub position: u8,
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
    CREATE_INSTANT_INVITE = 0,
    KICK_MEMBERS = 1,
    BAN_MEMBERS = 2,
    ADMINISTRATOR = 3,
    MANAGE_CHANNELS = 4,
    MANAGE_GUILD = 5,
    ADD_REACTIONS = 6,
    VIEW_AUDIT_LOG = 7,
    PRIORITY_SPEAKER = 8,
    STREAM = 9,
    VIEW_CHANNEL = 10,
    SEND_MESSAGES = 11,
    SEND_TTS_MESSAGES = 12,
    MANAGE_MESSAGES = 13,
    EMBED_LINKS = 14,
    ATTACH_FILES = 15,
    READ_MESSAGE_HISTORY = 16,
    MENTION_EVERYONE = 17,
    USE_EXTERNAL_EMOJIS = 18,
    VIEW_GUILD_INSIGHTS = 19,
    CONNECT = 20,
    SPEAK = 21,
    MUTE_MEMBERS = 22,
    DEAFEN_MEMBERS = 23,
    MOVE_MEMBERS = 24,
    USE_VAD = 25,
    CHANGE_NICKNAME = 26,
    MANAGE_NICKNAMES = 27,
    MANAGE_ROLES = 28,
    MANAGE_WEBHOOKS = 29,
    MANAGE_EMOJIS_AND_STICKERS = 30,
    USE_APPLICATION_COMMANDS = 31,
    REQUEST_TO_SPEAK = 32,
    MANAGE_EVENTS = 33,
    MANAGE_THREADS = 34,
    CREATE_PUBLIC_THREADS = 35,
    CREATE_PRIVATE_THREADS = 36,
    USE_EXTERNAL_STICKERS = 37,
    SEND_MESSAGES_IN_THREADS = 38,
    START_EMBEDDED_ACTIVITIES = 39,
    MODERATE_MEMBERS = 40,
}
