mod methods;
mod native;
mod plugin;
mod request;

use bevy_ecs::event::Event;
pub use methods::{
    achievement_progress, achievements_reset, authenticate, delete_savegame, fetch_save_games,
    fetch_signature, init_listeners, leaderboards_score, load_game, request_player,
    resolve_conflicting_games, save_game, trigger_view,
};
pub use plugin::{IosGamecenterEvents, IosGamecenterPlugin};
pub use request::{BevyIosGamecenter, BevyIosGamecenterSet};

/// Expected event data in response to [`init`] method call or
/// implicit on startup when registering Plugin via `IosGamecenterPlugin::new(true)`.
///
/// See Event [`IosGamecenterEvents`]
#[derive(Debug, Clone, Event)]
pub enum IosGCAuthResult {
    IsAuthenticated,
    LoginPresented,
    Error(String),
}

impl IosGCAuthResult {
    fn authenticated() -> Self {
        Self::IsAuthenticated
    }

    fn login_presented() -> Self {
        Self::LoginPresented
    }

    fn error(e: String) -> Self {
        Self::Error(e)
    }
}

#[derive(Debug, Clone, Default)]
pub struct IosGCSaveGames(pub Vec<IosGCSaveGame>);

impl IosGCSaveGames {
    fn new(items: Vec<IosGCSaveGame>) -> Self {
        Self(items)
    }

    pub fn contains(items: &Self, a: &IosGCSaveGame) -> bool {
        items.0.contains(a)
    }
}

#[derive(Debug, Clone, Event)]
pub enum IosGCResolvedConflictsResponse {
    Done(IosGCSaveGames),
    Error(String),
}

impl IosGCResolvedConflictsResponse {
    fn done(items: IosGCSaveGames) -> Self {
        Self::Done(items)
    }

    fn error(e: String) -> Self {
        Self::Error(e)
    }
}

/// Expected event data in response to [`request_player`] method call.
/// See Event [`IosGamecenterEvents`]
#[derive(Event, Debug, Clone, Default)]
pub struct IosGCPlayer {
    pub game_id: String,
    pub team_id: String,
    pub is_authenticated: bool,
    pub alias: String,
    pub display_name: String,
}

impl IosGCPlayer {
    pub fn new(
        game_id: String,
        team_id: String,
        is_authenticated: bool,
        alias: String,
        display_name: String,
    ) -> Self {
        Self {
            game_id,
            team_id,
            is_authenticated,
            alias,
            display_name,
        }
    }
}

/// Expected event data in response to [`save_game`] method call.
/// See Event [`IosGamecenterEvents`]
#[derive(Debug, Clone, Event)]
pub enum IosGCSavedGameResponse {
    Done(IosGCSaveGame),
    Error(String),
}

impl IosGCSavedGameResponse {
    fn done(item: IosGCSaveGame) -> Self {
        Self::Done(item)
    }

    fn error(e: String) -> Self {
        Self::Error(e)
    }
}

/// Expected event data in response to [`fetch_save_games`] method call.
/// See Event [`IosGamecenterEvents`]
#[derive(Debug, Clone, Event)]
pub enum IosGCSaveGamesResponse {
    Done(IosGCSaveGames),
    Error(String),
}

impl IosGCSaveGamesResponse {
    fn done(items: IosGCSaveGames) -> Self {
        Self::Done(items)
    }

    fn error(e: String) -> Self {
        Self::Error(e)
    }
}

/// Save Game meta data.
/// Expected event data in response to [`save_game`] or [`fetch_save_games`] method call.
/// See Event [`IosGamecenterEvents`]
///
/// ## Note
/// This does not contain the actual saved bytes, these have to be requested via [`load_game`]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IosGCSaveGame {
    pub name: String,
    pub device_name: String,
    pub modification_date: u64,
}

impl IosGCSaveGame {
    pub fn new(name: String, device_name: String, modification_date: u64) -> Self {
        Self {
            name,
            device_name,
            modification_date,
        }
    }

    pub fn equals(a: &IosGCSaveGame, b: &IosGCSaveGame) -> bool {
        a == b
    }
}

/// Expected event data in response to [`load_game`] method call.
/// See Event [`IosGamecenterEvents`]
#[derive(Debug, Clone, Event)]
pub enum IosGCLoadGamesResponse {
    /// Indicates a successfully loaded Save Game
    /// It will return the Save Game that was requested and the Data as a `Option<Vec<u8>>`.
    /// The `Option` will only be `None` in case of an error decoding the underlying Data back from base64 encoding.
    Done((IosGCSaveGame, Option<Vec<u8>>)),
    /// Returned if requested Save Game was not found
    Unknown(IosGCSaveGame),
    Error(String),
}

impl IosGCLoadGamesResponse {
    fn done(save_game: IosGCSaveGame, data: String) -> Self {
        use base64::Engine;
        Self::Done((
            save_game,
            base64::engine::general_purpose::STANDARD.decode(data).ok(),
        ))
    }

    fn unknown(save_game: IosGCSaveGame) -> Self {
        Self::Unknown(save_game)
    }

    fn error(e: String) -> Self {
        Self::Error(e)
    }
}

#[derive(Debug, Clone, Default)]
pub struct IosGCAchievement {
    pub identifier: String,
    pub progress: f64,
    pub is_completed: bool,
    pub last_reported_date: u64,
}

impl IosGCAchievement {
    pub fn new(
        identifier: String,
        progress: f64,
        is_completed: bool,
        last_reported_date: u64,
    ) -> Self {
        Self {
            identifier,
            progress,
            is_completed,
            last_reported_date,
        }
    }
}

/// Expected event data in response to [`achievement_progress`] method call.
/// See Event [`IosGamecenterEvents`]
#[derive(Event, Debug, Clone)]
pub enum IosGCAchievementProgressResponse {
    Done(IosGCAchievement),
    Error(String),
}

impl IosGCAchievementProgressResponse {
    fn done(a: IosGCAchievement) -> Self {
        Self::Done(a)
    }

    fn error(e: String) -> Self {
        Self::Error(e)
    }
}

/// Expected event data in response to [`achievements_reset`] method call.
/// See Event [`IosGamecenterEvents`]
#[derive(Event, Debug, Clone)]
pub enum IosGCAchievementsResetResponse {
    Done,
    Error(String),
}

impl IosGCAchievementsResetResponse {
    fn done() -> Self {
        Self::Done
    }

    fn error(e: String) -> Self {
        Self::Error(e)
    }
}

/// Expected event data in response to [`leaderboards_score`] method call.
/// See Event [`IosGamecenterEvents`]
#[derive(Event, Debug, Clone)]
pub enum IosGCScoreSubmitResponse {
    Done,
    Error(String),
}

impl IosGCScoreSubmitResponse {
    fn done() -> Self {
        Self::Done
    }

    fn error(e: String) -> Self {
        Self::Error(e)
    }
}

/// Expected event data in response to [`delete_savegame`] method call.
/// See Event [`IosGamecenterEvents`]
#[derive(Event, Debug, Clone)]
pub enum IosGCDeleteSaveGameResponse {
    Done(String),
    Error(String),
}

impl IosGCDeleteSaveGameResponse {
    fn done(name: String) -> Self {
        Self::Done(name)
    }

    fn error(e: String) -> Self {
        Self::Error(e)
    }
}

#[derive(Debug, Clone, Default)]
pub struct IosGCFetchItemsForSignatureVerification {
    pub url: String,
    pub signature: Vec<u8>,
    pub salt: Vec<u8>,
    pub timestamp: u64,
}

impl IosGCFetchItemsForSignatureVerification {
    pub fn new(
        url: String,
        signature_as_base64: String,
        salt_as_base64: String,
        timestamp: u64,
    ) -> Self {
        use base64::Engine;
        let signature = base64::engine::general_purpose::STANDARD
            .decode(signature_as_base64)
            .unwrap_or_default();
        let salt = base64::engine::general_purpose::STANDARD
            .decode(salt_as_base64)
            .unwrap_or_default();
        Self {
            url,
            signature,
            salt,
            timestamp,
        }
    }
}

/// Expected event data in response to [`fetch_signature`] method call.
/// See Event [`IosGamecenterEvents`]
#[derive(Event, Debug, Clone)]
pub enum IosGCFetchItemsForSignatureVerificationResponse {
    Done(IosGCFetchItemsForSignatureVerification),
    Error(String),
}

impl IosGCFetchItemsForSignatureVerificationResponse {
    fn done(items: IosGCFetchItemsForSignatureVerification) -> Self {
        Self::Done(items)
    }

    fn error(e: String) -> Self {
        Self::Error(e)
    }
}

/// used in [`trigger_view`] to define what view to open
pub type IosGCViewState = i32;

/// used in [`trigger_view`] to define what view to open.
///
/// See values in Apple Docs: <https://developer.apple.com/documentation/gamekit/gkgamecenterviewcontrollerstate>
pub mod view_states {
    use crate::IosGCViewState;

    /// Defines what part of Gamecenter UI to open. See [`trigger_view`][crate::trigger_view]
    ///
    /// See values in Apple Docs: <https://developer.apple.com/documentation/gamekit/gkgamecenterviewcontrollerstate/default>
    pub static DEFAULT: IosGCViewState = -1;
    /// Defines what part of Gamecenter UI to open. See [`trigger_view`][crate::trigger_view]
    ///
    /// See values in Apple Docs: <https://developer.apple.com/documentation/gamekit/gkgamecenterviewcontrollerstate/leaderboards>
    pub static LEADERBOARDS: IosGCViewState = 0;
    /// Defines what part of Gamecenter UI to open. See [`trigger_view`][crate::trigger_view]
    ///
    /// See values in Apple Docs: <https://developer.apple.com/documentation/gamekit/gkgamecenterviewcontrollerstate/achievements>
    pub static ACHIEVEMENTS: IosGCViewState = 1;
    /// Defines what part of Gamecenter UI to open. See [`trigger_view`][crate::trigger_view]
    ///
    /// See values in Apple Docs: <https://developer.apple.com/documentation/gamekit/gkgamecenterviewcontrollerstate/challenges>
    pub static CHALLENGES: IosGCViewState = 2;
    /// Defines what part of Gamecenter UI to open. See [`trigger_view`][crate::trigger_view]
    ///
    /// See values in Apple Docs: <https://developer.apple.com/documentation/gamekit/gkgamecenterviewcontrollerstate/localplayerprofile>
    pub static LOCAL_PLAYER_PROFILES: IosGCViewState = 3;
    /// Defines what part of Gamecenter UI to open. See [`trigger_view`][crate::trigger_view]
    ///
    /// See values in Apple Docs: <https://developer.apple.com/documentation/gamekit/gkgamecenterviewcontrollerstate/dashboard>
    pub static DASHBOARD: IosGCViewState = 4;
    /// Defines what part of Gamecenter UI to open. See [`trigger_view`][crate::trigger_view]
    ///
    /// See values in Apple Docs: <https://developer.apple.com/documentation/gamekit/gkgamecenterviewcontrollerstate/localplayerfriendslist>
    pub static LOCAL_PLAYER_FRIENDS_LIST: IosGCViewState = 5;
}
