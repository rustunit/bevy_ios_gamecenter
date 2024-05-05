mod methods;
mod native;
mod plugin;

pub use methods::{
    achievement_progress, achievements_reset, fetch_save_games, init, leaderboards_score,
    load_game, request_player, save_game, trigger_view,
};
pub use plugin::{IosGamecenterEvents, IosGamecenterPlugin};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum IosGCSaveGamesResponse {
    Done(Vec<IosGCSaveGame>),
    Error(String),
}

impl IosGCSaveGamesResponse {
    fn done(items: Vec<IosGCSaveGame>) -> Self {
        Self::Done(items)
    }

    fn error(e: String) -> Self {
        Self::Error(e)
    }
}

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

#[derive(Debug, Clone)]
pub enum IosGCLoadGamesResponse {
    Done((IosGCSaveGame, Option<Vec<u8>>)),
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

pub type IosGCViewState = i32;

pub mod view_states {
    use crate::IosGCViewState;

    pub static DEFAULT: IosGCViewState = -1;
    pub static LEADERBOARDS: IosGCViewState = 0;
    pub static ACHIEVEMENTS: IosGCViewState = 1;
    pub static CHALLENGES: IosGCViewState = 2;
    pub static LOCAL_PLAYER_PROFILES: IosGCViewState = 3;
    pub static DASHBOARD: IosGCViewState = 4;
    pub static LOCAL_PLAYER_FRIENDS_LIST: IosGCViewState = 5;
}
