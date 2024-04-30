mod methods;
mod native;
mod plugin;

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
