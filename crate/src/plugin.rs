use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use crate::{
    IosGCAchievementProgressResponse, IosGCAchievementsResetResponse, IosGCAuthResult,
    IosGCDeleteSaveGameResponse, IosGCFetchItemsForSignatureVerificationResponse,
    IosGCLoadGamesResponse, IosGCPlayer, IosGCResolvedConflictsResponse, IosGCSaveGames,
    IosGCSaveGamesResponse, IosGCSavedGameResponse, IosGCScoreSubmitResponse,
};

/// All events for communication from native iOS (Swift) side to Rust/Bevy
#[derive(Event, Clone, Debug)]
pub enum IosGamecenterEvents {
    /// Triggered by calls to [`init`][crate::init] or implicit when registering [`IosGamecenterPlugin`] via `IosGamecenterPlugin::new(true)`
    Authentication((i64, IosGCAuthResult)),
    /// Triggered by calls to [`save_game`][crate::save_game]
    SavedGame((i64, IosGCSavedGameResponse)),
    /// Triggered by calls to [`fetch_save_games`][crate::fetch_save_games]
    SaveGames((i64, IosGCSaveGamesResponse)),
    /// Triggered by calls to [`load_game`][crate::load_game]
    LoadGame((i64, IosGCLoadGamesResponse)),
    /// Triggered by calls to [`delete_savegame`][crate::delete_savegame]
    DeletedSaveGame((i64, IosGCDeleteSaveGameResponse)),
    /// Triggered by calls to [`request_player`][crate::request_player]
    Player((i64, IosGCPlayer)),
    /// Triggered by calls to [`achievement_progress`][crate::achievement_progress]
    AchievementProgress((i64, IosGCAchievementProgressResponse)),
    /// Triggered by calls to [`achievements_reset`][crate::achievements_reset]
    AchievementsReset((i64, IosGCAchievementsResetResponse)),
    /// Triggered by calls to [`leaderboards_score`][crate::leaderboards_score]
    LeaderboardScoreSubmitted((i64, IosGCScoreSubmitResponse)),
    /// Triggered by calls to [`fetch_signature`][crate::fetch_signature]
    ItemsForSignatureVerification((i64, IosGCFetchItemsForSignatureVerificationResponse)),
    /// Triggered by calls to [`fetch_save_games`][crate::fetch_save_games] or [`save_game`][crate::save_game]
    ConflictingSaveGames(IosGCSaveGames),
    /// Triggered by calls to [`resolve_conflicting_games`][crate::resolve_conflicting_games]
    ResolvedConflicts((i64, IosGCResolvedConflictsResponse)),
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
                .world()
                .get_resource::<CrossbeamEventSender<IosGamecenterEvents>>()
                .unwrap()
                .clone();

            crate::native::set_sender(sender);

            if self.auto_init {
                crate::native::init_listeners();
            }
        }
    }
}
