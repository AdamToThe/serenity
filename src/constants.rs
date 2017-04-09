/// The gateway version used by the library. The gateway URI is retrieved via
/// the REST API.
pub const GATEWAY_VERSION: u8 = 6;
/// The large threshold to send on identify.
pub const LARGE_THRESHOLD: u8 = 250;
/// The maximum unicode code points allowed within a message by Discord.
pub const MESSAGE_CODE_LIMIT: u16 = 2000;
/// The [UserAgent] sent along with every request.
///
/// [UserAgent]: ../hyper/header/struct.UserAgent.html
pub const USER_AGENT: &'static str = concat!("DiscordBot (https://github.com/zeyla/serenity, ", env!("CARGO_PKG_VERSION"), ")");

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ErrorCode {
    BotsCannotUse,
    CannotSendEmptyMessage,
    CannotSendMessagesInVoice,
    CannotSendMessagesToUser,
    ChannelVerificationTooHigh,
    EditByOtherAuthor,
    EmbedDisabled,
    InvalidAccountType,
    InvalidAuthToken,
    InvalidBulkDeleteCount,
    InvalidDMChannelAction,
    InvalidOauthState,
    InvalidPinChannel,
    MaxFriendsReached,
    MaxGuildsReached,
    MaxPinsReached,
    MaxRolesReached,
    MissingAccess,
    MissingPermissions,
    NoteTooLong,
    Oauth2ApplicationLacksBot,
    Oauth2ApplicationLimitReached,
    OnlyBotsCanUse,
    ReactionBlocked,
    SearchIndexUnavailable,
    TooManyReactions,
    Unauthorized,
    UnknownAccount,
    UnknownApplication,
    UnknownChannel,
    UnknownGuild,
    UnknownEmoji,
    UnknownIntegration,
    UnknownInvite,
    UnknownMember,
    UnknownMessage,
    UnknownOverwrite,
    UnknownProvider,
    UnknownRole,
    UnknownToken,
    UnknownUser,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpCode {
    Event,
    Heartbeat,
    Identify,
    StatusUpdate,
    VoiceStateUpdate,
    VoiceServerPing,
    Resume,
    Reconnect,
    GetGuildMembers,
    InvalidSession,
    Hello,
    HeartbeatAck,
}

impl OpCode {
    pub fn from_num(num: u64) -> Option<Self> {
        match num {
            0 => Some(OpCode::Event),
            1 => Some(OpCode::Heartbeat),
            2 => Some(OpCode::Identify),
            3 => Some(OpCode::StatusUpdate),
            4 => Some(OpCode::VoiceStateUpdate),
            5 => Some(OpCode::VoiceServerPing),
            6 => Some(OpCode::Resume),
            7 => Some(OpCode::Reconnect),
            8 => Some(OpCode::GetGuildMembers),
            9 => Some(OpCode::InvalidSession),
            10 => Some(OpCode::Hello),
            11 => Some(OpCode::HeartbeatAck),
            _ => None,
        }
    }

    pub fn num(&self) -> u64 {
        match *self {
            OpCode::Event => 0,
            OpCode::Heartbeat => 1,
            OpCode::Identify => 2,
            OpCode::StatusUpdate => 3,
            OpCode::VoiceStateUpdate => 4,
            OpCode::VoiceServerPing => 5,
            OpCode::Resume => 6,
            OpCode::Reconnect => 7,
            OpCode::GetGuildMembers => 8,
            OpCode::InvalidSession => 9,
            OpCode::Hello => 10,
            OpCode::HeartbeatAck => 11,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VoiceOpCode {
    Identify,
    Heartbeat,
    Hello,
    KeepAlive,
    SelectProtocol,
    SessionDescription,
    Speaking,
}

impl VoiceOpCode {
    pub fn from_num(num: u64) -> Option<Self> {
        match num {
            0 => Some(VoiceOpCode::Identify),
            1 => Some(VoiceOpCode::SelectProtocol),
            2 => Some(VoiceOpCode::Hello),
            3 => Some(VoiceOpCode::KeepAlive),
            4 => Some(VoiceOpCode::SessionDescription),
            5 => Some(VoiceOpCode::Speaking),
            8 => Some(VoiceOpCode::Heartbeat),
            _ => None,
        }
    }

    pub fn num(&self) -> u64 {
        match *self {
            VoiceOpCode::Identify => 0,
            VoiceOpCode::SelectProtocol => 1,
            VoiceOpCode::Hello => 2,
            VoiceOpCode::KeepAlive => 3,
            VoiceOpCode::SessionDescription => 4,
            VoiceOpCode::Speaking => 5,
            VoiceOpCode::Heartbeat => 8,
        }
    }
}
