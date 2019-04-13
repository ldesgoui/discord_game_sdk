use crate::sys;

const INVALID_ENUM: &'static str = "(discord_game_sdk) failed to convert from C enum";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CreateFlags {
    /// Requires Discord to be running to play the game
    Default,
    /// Does not require Discord to be running, use this on other platforms
    NoRequireDiscord,
}

impl Default for CreateFlags {
    fn default() -> Self {
        CreateFlags::Default
    }
}

impl Into<sys::EDiscordCreateFlags> for CreateFlags {
    fn into(self) -> sys::EDiscordCreateFlags {
        match self {
            CreateFlags::Default => sys::DiscordCreateFlags_Default,
            CreateFlags::NoRequireDiscord => sys::DiscordCreateFlags_NoRequireDiscord,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActivityKind {
    Listening,
    Playing,
    Streaming,
    Watching,
}

impl From<sys::EDiscordActivityType> for ActivityKind {
    fn from(source: sys::EDiscordActivityType) -> Self {
        match source {
            sys::DiscordActivityType_Listening => ActivityKind::Listening,
            sys::DiscordActivityType_Playing => ActivityKind::Playing,
            sys::DiscordActivityType_Streaming => ActivityKind::Streaming,
            sys::DiscordActivityType_Watching => ActivityKind::Watching,
            _ => panic!(INVALID_ENUM),
        }
    }
}

impl Into<sys::EDiscordActivityType> for ActivityKind {
    fn into(self) -> sys::EDiscordActivityType {
        match self {
            ActivityKind::Listening => sys::DiscordActivityType_Listening,
            ActivityKind::Playing => sys::DiscordActivityType_Playing,
            ActivityKind::Streaming => sys::DiscordActivityType_Streaming,
            ActivityKind::Watching => sys::DiscordActivityType_Watching,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Join,
    Spectate,
}

impl From<sys::EDiscordActivityActionType> for Action {
    fn from(source: sys::EDiscordActivityActionType) -> Self {
        match source {
            sys::DiscordActivityActionType_Join => Action::Join,
            sys::DiscordActivityActionType_Spectate => Action::Spectate,
            _ => panic!(INVALID_ENUM),
        }
    }
}

impl Into<sys::EDiscordActivityActionType> for Action {
    fn into(self) -> sys::EDiscordActivityActionType {
        match self {
            Action::Join => sys::DiscordActivityActionType_Join,
            Action::Spectate => sys::DiscordActivityActionType_Spectate,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RequestReply {
    Yes,
    No,
    Ignore,
}

impl Into<sys::EDiscordActivityJoinRequestReply> for RequestReply {
    fn into(self) -> sys::EDiscordActivityJoinRequestReply {
        match self {
            RequestReply::Yes => sys::DiscordActivityJoinRequestReply_Yes,
            RequestReply::No => sys::DiscordActivityJoinRequestReply_No,
            RequestReply::Ignore => sys::DiscordActivityJoinRequestReply_Ignore,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageKind {
    User,
}

impl From<sys::EDiscordImageType> for ImageKind {
    fn from(source: sys::EDiscordImageType) -> Self {
        match source {
            sys::DiscordImageType_User => ImageKind::User,
            _ => panic!(INVALID_ENUM),
        }
    }
}

impl Into<sys::EDiscordImageType> for ImageKind {
    fn into(self) -> sys::EDiscordImageType {
        match self {
            ImageKind::User => sys::DiscordImageType_User,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LobbyKind {
    Public,
    Private,
}

impl From<sys::EDiscordLobbyType> for LobbyKind {
    fn from(source: sys::EDiscordLobbyType) -> Self {
        match source {
            sys::DiscordLobbyType_Public => LobbyKind::Public,
            sys::DiscordLobbyType_Private => LobbyKind::Private,
            _ => panic!(INVALID_ENUM),
        }
    }
}

impl Into<sys::EDiscordLobbyType> for LobbyKind {
    fn into(self) -> sys::EDiscordLobbyType {
        match self {
            LobbyKind::Public => sys::DiscordLobbyType_Public,
            LobbyKind::Private => sys::DiscordLobbyType_Private,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cast {
    Number,
    String,
}

impl Into<sys::EDiscordLobbySearchCast> for Cast {
    fn into(self) -> sys::EDiscordLobbySearchCast {
        match self {
            Cast::String => sys::DiscordLobbySearchCast_String,
            Cast::Number => sys::DiscordLobbySearchCast_Number,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    NotEqual,
}

impl Into<sys::EDiscordLobbySearchComparison> for Comparison {
    fn into(self) -> sys::EDiscordLobbySearchComparison {
        match self {
            Comparison::Equal => sys::DiscordLobbySearchComparison_Equal,
            Comparison::GreaterThan => sys::DiscordLobbySearchComparison_GreaterThan,
            Comparison::GreaterThanOrEqual => sys::DiscordLobbySearchComparison_GreaterThanOrEqual,
            Comparison::LessThan => sys::DiscordLobbySearchComparison_LessThan,
            Comparison::LessThanOrEqual => sys::DiscordLobbySearchComparison_LessThanOrEqual,
            Comparison::NotEqual => sys::DiscordLobbySearchComparison_NotEqual,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Distance {
    Default,
    Extended,
    Global,
    Local,
}

impl Into<sys::EDiscordLobbySearchDistance> for Distance {
    fn into(self) -> sys::EDiscordLobbySearchDistance {
        match self {
            Distance::Default => sys::DiscordLobbySearchDistance_Default,
            Distance::Extended => sys::DiscordLobbySearchDistance_Extended,
            Distance::Global => sys::DiscordLobbySearchDistance_Global,
            Distance::Local => sys::DiscordLobbySearchDistance_Local,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RelationshipKind {
    Blocked,
    Friend,
    Implicit,
    None,
    PendingIncoming,
    PendingOutgoing,
}

impl From<sys::EDiscordRelationshipType> for RelationshipKind {
    fn from(source: sys::EDiscordRelationshipType) -> Self {
        match source {
            sys::DiscordRelationshipType_Blocked => RelationshipKind::Blocked,
            sys::DiscordRelationshipType_Friend => RelationshipKind::Friend,
            sys::DiscordRelationshipType_Implicit => RelationshipKind::Implicit,
            sys::DiscordRelationshipType_None => RelationshipKind::None,
            sys::DiscordRelationshipType_PendingIncoming => RelationshipKind::PendingIncoming,
            sys::DiscordRelationshipType_PendingOutgoing => RelationshipKind::PendingOutgoing,
            _ => panic!(INVALID_ENUM),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    DoNotDisturb,
    Idle,
    Offline,
    Online,
}

impl From<sys::EDiscordStatus> for Status {
    fn from(source: sys::EDiscordStatus) -> Self {
        match source {
            sys::DiscordStatus_DoNotDisturb => Status::DoNotDisturb,
            sys::DiscordStatus_Idle => Status::Idle,
            sys::DiscordStatus_Offline => Status::Offline,
            sys::DiscordStatus_Online => Status::Online,
            _ => panic!(INVALID_ENUM),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EntitlementKind {
    DeveloperGift,
    FreePurchase,
    PremiumPurchase,
    PremiumSubscription,
    Purchase,
    TestModePurchase,
    UserGift,
}

impl From<sys::EDiscordEntitlementType> for EntitlementKind {
    fn from(source: sys::EDiscordEntitlementType) -> Self {
        match source {
            sys::DiscordEntitlementType_DeveloperGift => EntitlementKind::DeveloperGift,
            sys::DiscordEntitlementType_FreePurchase => EntitlementKind::FreePurchase,
            sys::DiscordEntitlementType_PremiumPurchase => EntitlementKind::PremiumPurchase,
            sys::DiscordEntitlementType_PremiumSubscription => EntitlementKind::PremiumSubscription,
            sys::DiscordEntitlementType_Purchase => EntitlementKind::Purchase,
            sys::DiscordEntitlementType_TestModePurchase => EntitlementKind::TestModePurchase,
            sys::DiscordEntitlementType_UserGift => EntitlementKind::UserGift,
            _ => panic!(INVALID_ENUM),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PremiumKind {
    /// Not a Nitro subscriber
    None,
    /// Nitro Classic subscriber
    Tier1,
    /// Nitro subscriber
    Tier2,
}

impl From<sys::EDiscordPremiumType> for PremiumKind {
    fn from(source: sys::EDiscordPremiumType) -> Self {
        match source {
            sys::DiscordPremiumType_None => PremiumKind::None,
            sys::DiscordPremiumType_Tier1 => PremiumKind::Tier1,
            sys::DiscordPremiumType_Tier2 => PremiumKind::Tier2,
            _ => panic!(INVALID_ENUM),
        }
    }
}
