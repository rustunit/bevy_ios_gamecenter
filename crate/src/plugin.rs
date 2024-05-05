use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use crate::{
    IosGCAchievementProgressResponse, IosGCAchievementsResetResponse, IosGCAuthResult,
    IosGCLoadGamesResponse, IosGCPlayer, IosGCSaveGamesResponse, IosGCSavedGameResponse,
    IosGCScoreSubmitResponse,
};

/// All events for communication from native iOS (Swift) side to Rust/Bevy
#[derive(Event, Clone, Debug)]
pub enum IosGamecenterEvents {
    Authentication(IosGCAuthResult),
    SavedGame(IosGCSavedGameResponse),
    SaveGames(IosGCSaveGamesResponse),
    LoadGame(IosGCLoadGamesResponse),
    Player(IosGCPlayer),
    AchievementProgress(IosGCAchievementProgressResponse),
    AchievementsReset(IosGCAchievementsResetResponse),
    LeaderboardScoreSubmitted(IosGCScoreSubmitResponse),
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
