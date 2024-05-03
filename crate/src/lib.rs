mod methods;
mod native;
mod plugin;

pub use methods::{fetch_save_games, init, load_game, request_player, save_game};
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
        Self::Done((save_game, base64::decode(data).ok()))
    }

    fn unknown(save_game: IosGCSaveGame) -> Self {
        Self::Unknown(save_game)
    }

    fn error(e: String) -> Self {
        Self::Error(e)
    }
}
