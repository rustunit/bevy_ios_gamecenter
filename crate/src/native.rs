#![allow(clippy::unnecessary_cast, unused_variables)]

use std::sync::OnceLock;

use bevy_crossbeam_event::CrossbeamEventSender;

#[allow(unused_imports)]
pub use ffi::*;

use crate::{
    plugin::IosGamecenterEvents, IosGCAchievement, IosGCAchievementProgressResponse,
    IosGCAchievementsResetResponse, IosGCAuthResult, IosGCLoadGamesResponse, IosGCPlayer,
    IosGCSaveGame, IosGCSaveGamesResponse, IosGCSavedGameResponse, IosGCScoreSubmitResponse,
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

        type IosGCAchievement;

        #[swift_bridge(associated_to = IosGCAchievement)]
        fn new(
            identifier: String,
            progress: f64,
            is_completed: bool,
            last_reported_date: u64,
        ) -> IosGCAchievement;

        type IosGCAchievementProgressResponse;

        #[swift_bridge(associated_to = IosGCAchievementProgressResponse)]
        fn done(a: IosGCAchievement) -> IosGCAchievementProgressResponse;
        #[swift_bridge(associated_to = IosGCAchievementProgressResponse)]
        fn error(e: String) -> IosGCAchievementProgressResponse;

        type IosGCAchievementsResetResponse;

        #[swift_bridge(associated_to = IosGCAchievementsResetResponse)]
        fn done() -> IosGCAchievementsResetResponse;
        #[swift_bridge(associated_to = IosGCAchievementsResetResponse)]
        fn error(e: String) -> IosGCAchievementsResetResponse;

        type IosGCScoreSubmitResponse;

        #[swift_bridge(associated_to = IosGCScoreSubmitResponse)]
        fn done() -> IosGCScoreSubmitResponse;
        #[swift_bridge(associated_to = IosGCScoreSubmitResponse)]
        fn error(e: String) -> IosGCScoreSubmitResponse;

        fn authentication(result: IosGCAuthResult);
        fn receive_player(p: IosGCPlayer);
        fn receive_load_game(response: IosGCLoadGamesResponse);
        fn receive_saved_game(response: IosGCSavedGameResponse);
        fn receive_save_games(response: IosGCSaveGamesResponse);
        fn receive_achievement_progress(response: IosGCAchievementProgressResponse);
        fn receive_achievement_reset(response: IosGCAchievementsResetResponse);
        fn receive_leaderboard_score(response: IosGCScoreSubmitResponse);
    }

    extern "Swift" {
        pub fn ios_gc_init();
        pub fn get_player();
        pub fn save_game(data: String, name: String);
        pub fn load_game(save_game: IosGCSaveGame);
        pub fn fetch_save_games();
        pub fn achievement_progress(id: String, progress: f64);
        pub fn reset_achievements();
        pub fn leaderboards_score(id: String, score: i64, context: i64);
        pub fn trigger_view(state: i32);
    }
}

static SENDER: OnceLock<Option<CrossbeamEventSender<IosGamecenterEvents>>> = OnceLock::new();

#[allow(dead_code)]
pub fn set_sender(sender: CrossbeamEventSender<IosGamecenterEvents>) {
    while SENDER.set(Some(sender.clone())).is_err() {}
}

fn authentication(result: IosGCAuthResult) {
    #[cfg(target_os = "ios")]
    {
        get_player();

        SENDER
            .get()
            .unwrap()
            .as_ref()
            .unwrap()
            .send(IosGamecenterEvents::Authentication(result));
    }
}

fn receive_player(p: IosGCPlayer) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::Player(p));
}

fn receive_saved_game(response: IosGCSavedGameResponse) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::SavedGame(response));
}

fn receive_save_games(response: IosGCSaveGamesResponse) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::SaveGames(response));
}

fn receive_load_game(response: IosGCLoadGamesResponse) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::LoadGame(response));
}

fn receive_achievement_progress(response: IosGCAchievementProgressResponse) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::AchievementProgress(response));
}

fn receive_achievement_reset(response: IosGCAchievementsResetResponse) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::AchievementsReset(response));
}

fn receive_leaderboard_score(response: IosGCScoreSubmitResponse) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::LeaderboardScoreSubmitted(response));
}
