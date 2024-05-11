use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use crate::{
    IosGCAchievementProgressResponse, IosGCAchievementsResetResponse, IosGCAuthResult,
    IosGCDeleteSaveGameResponse, IosGCFetchItemsForSignatureVerificationResponse,
    IosGCLoadGamesResponse, IosGCPlayer, IosGCSaveGamesResponse, IosGCSavedGameResponse,
    IosGCScoreSubmitResponse,
};

/// All events for communication from native iOS (Swift) side to Rust/Bevy
#[derive(Event, Clone, Debug)]
pub enum IosGamecenterEvents {
    /// Triggered by calls to [`init`][crate::init] or implicit when registering [`IosGamecenterPlugin`] via `IosGamecenterPlugin::new(true)`
    Authentication(IosGCAuthResult),
    /// Triggered by calls to [`save_game`][crate::save_game]
    SavedGame(IosGCSavedGameResponse),
    /// Triggered by calls to [`fetch_save_games`][crate::fetch_save_games]
    SaveGames(IosGCSaveGamesResponse),
    /// Triggered by calls to [`load_game`][crate::load_game]
    LoadGame(IosGCLoadGamesResponse),
    /// Triggered by calls to [`delete_savegame`][crate::delete_savegame]
    DeletedSaveGame(IosGCDeleteSaveGameResponse),
    /// Triggered by calls to [`request_player`][crate::request_player]
    Player(IosGCPlayer),
    /// Triggered by calls to [`achievement_progress`][crate::achievement_progress]
    AchievementProgress(IosGCAchievementProgressResponse),
    /// Triggered by calls to [`achievements_reset`][crate::achievements_reset]
    AchievementsReset(IosGCAchievementsResetResponse),
    /// Triggered by calls to [`leaderboards_score`][crate::leaderboards_score]
    LeaderboardScoreSubmitted(IosGCScoreSubmitResponse),
    /// Triggered by calls to [`fetch_signature`][crate::fetch_signature]
    ItemsForSignatureVerification(IosGCFetchItemsForSignatureVerificationResponse),
}

/// Bevy plugin to integrate access to iOS Gamecenter
#[allow(dead_code)]
pub struct IosGamecenterPlugin {
    auto_init: bool,
}

impl IosGamecenterPlugin {
    /// create plugin and define whether it will authenticate automatically right on startup
    pub fn new(auto_init: bool) -> Self {
        Self { auto_init }
    }
}

impl Plugin for IosGamecenterPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_os = "ios"))]
        {
            app.add_event::<IosGamecenterEvents>();
        }

        #[cfg(target_os = "ios")]
        {
            use bevy_crossbeam_event::{CrossbeamEventApp, CrossbeamEventSender};

            app.add_crossbeam_event::<IosGamecenterEvents>();

            let sender = app
                .world
                .get_resource::<CrossbeamEventSender<IosGamecenterEvents>>()
                .unwrap()
                .clone();

            crate::native::set_sender(sender);

            if self.auto_init {
                crate::native::ios_gc_init();
            }
        }
    }
}
