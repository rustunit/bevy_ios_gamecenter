use std::sync::OnceLock;

use bevy_crossbeam_event::CrossbeamEventSender;

#[allow(unused_imports)]
pub use ffi::*;

use crate::{
    plugin::IosGamecenterEvents, IosGCAuthResult, IosGCLoadGamesResponse, IosGCPlayer,
    IosGCSaveGame, IosGCSaveGamesResponse, IosGCSavedGameResponse,
};

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type IosGCPlayer;

        #[swift_bridge(associated_to = IosGCPlayer)]
        fn new(
            game_id: String,
            team_id: String,
            is_authenticated: bool,
            alias: String,
            display_name: String,
        ) -> IosGCPlayer;

        type IosGCAuthResult;

        #[swift_bridge(associated_to = IosGCAuthResult)]
        fn authenticated() -> IosGCAuthResult;
        #[swift_bridge(associated_to = IosGCAuthResult)]
        fn login_presented() -> IosGCAuthResult;
        #[swift_bridge(associated_to = IosGCAuthResult)]
        fn error(e: String) -> IosGCAuthResult;

        type IosGCSaveGame;

        #[swift_bridge(associated_to = IosGCSaveGame)]
        fn new(name: String, device_name: String, modification_date: u64) -> IosGCSaveGame;

        type IosGCSavedGameResponse;

        #[swift_bridge(associated_to = IosGCSavedGameResponse)]
        fn done(save: IosGCSaveGame) -> IosGCSavedGameResponse;
        #[swift_bridge(associated_to = IosGCSavedGameResponse)]
        fn error(e: String) -> IosGCSavedGameResponse;
        #[swift_bridge(associated_to = IosGCSaveGame)]
        fn equals(a: &IosGCSaveGame, b: &IosGCSaveGame) -> bool;

        type IosGCSaveGamesResponse;

        #[swift_bridge(associated_to = IosGCSaveGamesResponse)]
        fn done(items: Vec<IosGCSaveGame>) -> IosGCSaveGamesResponse;
        #[swift_bridge(associated_to = IosGCSaveGamesResponse)]
        fn error(e: String) -> IosGCSaveGamesResponse;

        type IosGCLoadGamesResponse;

        #[swift_bridge(associated_to = IosGCLoadGamesResponse)]
        fn done(save_game: IosGCSaveGame, data: String) -> IosGCLoadGamesResponse;
        #[swift_bridge(associated_to = IosGCLoadGamesResponse)]
        fn unknown(save_game: IosGCSaveGame) -> IosGCLoadGamesResponse;
        #[swift_bridge(associated_to = IosGCLoadGamesResponse)]
        fn error(e: String) -> IosGCLoadGamesResponse;

        fn authentication(result: IosGCAuthResult);
        fn receive_player(p: IosGCPlayer);
        fn receive_load_game(response: IosGCLoadGamesResponse);
        fn receive_saved_game(response: IosGCSavedGameResponse);
        fn receive_save_games(response: IosGCSaveGamesResponse);
    }

    extern "Swift" {
        pub fn ios_gc_init();
        pub fn get_player();
        pub fn save_game(data: String, name: String);
        pub fn load_game(save_game: IosGCSaveGame);
        pub fn fetch_save_games();
    }
}

static SENDER: OnceLock<Option<CrossbeamEventSender<IosGamecenterEvents>>> = OnceLock::new();

#[allow(dead_code)]
pub fn set_sender(sender: CrossbeamEventSender<IosGamecenterEvents>) {
    while !SENDER.set(Some(sender.clone())).is_ok() {}
}

fn authentication(result: IosGCAuthResult) {
    bevy_log::info!("authentication: {result:?}");

    get_player();

    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::Authentication(result));
}

fn receive_player(p: IosGCPlayer) {
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::Player(p));
}

fn receive_saved_game(response: IosGCSavedGameResponse) {
    bevy_log::info!("receive_saved_game: {:?}", response);

    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::SavedGame(response));
}

fn receive_save_games(response: IosGCSaveGamesResponse) {
    bevy_log::info!("receive_save_games: {:?}", response);

    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::SaveGames(response));
}

fn receive_load_game(response: IosGCLoadGamesResponse) {
    bevy_log::info!("receive_load_game: {:?}", response);

    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::LoadGame(response));
}
