use bevy_app::prelude::*;
use bevy_ecs::prelude::*;

use crate::{
    IosGCAuthResult, IosGCLoadGamesResponse, IosGCPlayer, IosGCSaveGamesResponse,
    IosGCSavedGameResponse,
};

///
#[derive(Event, Clone, Debug)]
pub enum IosGamecenterEvents {
    Authentication(IosGCAuthResult),
    SavedGame(IosGCSavedGameResponse),
    SaveGames(IosGCSaveGamesResponse),
    LoadGame(IosGCLoadGamesResponse),
    Player(IosGCPlayer),
}

///
#[allow(dead_code)]
pub struct IosGamecenterPlugin {
    auto_init: bool,
}

impl IosGamecenterPlugin {
    ///
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
