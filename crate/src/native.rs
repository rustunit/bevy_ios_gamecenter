#![allow(clippy::unnecessary_cast, unused_variables)]

use std::sync::OnceLock;

use bevy_crossbeam_event::CrossbeamEventSender;

#[allow(unused_imports)]
pub use ffi::*;

use crate::{
    plugin::IosGamecenterEvents, IosGCAchievement, IosGCAchievementProgressResponse,
    IosGCAchievementsResetResponse, IosGCAuthResult, IosGCDeleteSaveGameResponse,
    IosGCFetchItemsForSignatureVerification, IosGCFetchItemsForSignatureVerificationResponse,
    IosGCLoadGamesResponse, IosGCPlayer, IosGCResolvedConflictsResponse, IosGCSaveGame,
    IosGCSaveGames, IosGCSaveGamesResponse, IosGCSavedGameResponse, IosGCScoreSubmitResponse,
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
        fn done(items: IosGCSaveGames) -> IosGCSaveGamesResponse;
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

        type IosGCDeleteSaveGameResponse;

        #[swift_bridge(associated_to = IosGCDeleteSaveGameResponse)]
        fn done(e: String) -> IosGCDeleteSaveGameResponse;
        #[swift_bridge(associated_to = IosGCDeleteSaveGameResponse)]
        fn error(e: String) -> IosGCDeleteSaveGameResponse;

        type IosGCFetchItemsForSignatureVerification;

        #[swift_bridge(associated_to = IosGCFetchItemsForSignatureVerification)]
        fn new(
            url: String,
            signature_as_base64: String,
            salt_as_base64: String,
            timestamp: u64,
        ) -> IosGCFetchItemsForSignatureVerification;

        type IosGCFetchItemsForSignatureVerificationResponse;

        #[swift_bridge(associated_to = IosGCFetchItemsForSignatureVerificationResponse)]
        fn done(
            items: IosGCFetchItemsForSignatureVerification,
        ) -> IosGCFetchItemsForSignatureVerificationResponse;
        #[swift_bridge(associated_to = IosGCFetchItemsForSignatureVerificationResponse)]
        fn error(e: String) -> IosGCFetchItemsForSignatureVerificationResponse;

        type IosGCSaveGames;

        #[swift_bridge(associated_to = IosGCSaveGames)]
        fn new(items: Vec<IosGCSaveGame>) -> IosGCSaveGames;
        #[swift_bridge(associated_to = IosGCSaveGames)]
        fn contains(items: &IosGCSaveGames, item: &IosGCSaveGame) -> bool;

        type IosGCResolvedConflictsResponse;

        #[swift_bridge(associated_to = IosGCResolvedConflictsResponse)]
        fn done(items: IosGCSaveGames) -> IosGCResolvedConflictsResponse;
        #[swift_bridge(associated_to = IosGCResolvedConflictsResponse)]
        fn error(e: String) -> IosGCResolvedConflictsResponse;

        fn receive_authentication(request: i64, result: IosGCAuthResult);
        fn receive_player(request: i64, p: IosGCPlayer);
        fn receive_load_game(request: i64, response: IosGCLoadGamesResponse);
        fn receive_saved_game(request: i64, response: IosGCSavedGameResponse);
        fn receive_save_games(request: i64, response: IosGCSaveGamesResponse);
        fn receive_deleted_game(request: i64, response: IosGCDeleteSaveGameResponse);
        fn receive_achievement_progress(request: i64, response: IosGCAchievementProgressResponse);
        fn receive_achievement_reset(request: i64, response: IosGCAchievementsResetResponse);
        fn receive_leaderboard_score(request: i64, response: IosGCScoreSubmitResponse);
        fn receive_items_for_signature_verification(
            request: i64,
            response: IosGCFetchItemsForSignatureVerificationResponse,
        );
        fn receive_resolved_conflicts(request: i64, response: IosGCResolvedConflictsResponse);

        // no response
        fn receive_conflicting_savegames(savegames: IosGCSaveGames);
    }

    extern "Swift" {
        pub fn init_listeners();
        pub fn trigger_view(state: i32);

        pub fn authenticate(request: i64);
        pub fn get_player(request: i64);
        pub fn save_game(request: i64, data: String, name: String);
        pub fn load_game(request: i64, save_game: IosGCSaveGame);
        pub fn delete_game(request: i64, name: String);
        pub fn resolve_conflicting_games(request: i64, save_games: IosGCSaveGames, data: String);
        pub fn fetch_save_games(request: i64);
        pub fn achievement_progress(request: i64, id: String, progress: f64);
        pub fn reset_achievements(request: i64);
        pub fn leaderboards_score(request: i64, id: String, score: i64, context: i64);
        pub fn fetch_signature(request: i64);
    }
}

static SENDER: OnceLock<Option<CrossbeamEventSender<IosGamecenterEvents>>> = OnceLock::new();

#[allow(dead_code)]
pub fn set_sender(sender: CrossbeamEventSender<IosGamecenterEvents>) {
    while SENDER.set(Some(sender.clone())).is_err() {}
}

fn receive_authentication(request: i64, result: IosGCAuthResult) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::Authentication((request, result)));
}

fn receive_player(request: i64, p: IosGCPlayer) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::Player((request, p)));
}

fn receive_saved_game(request: i64, response: IosGCSavedGameResponse) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::SavedGame((request, response)));
}

fn receive_save_games(request: i64, response: IosGCSaveGamesResponse) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::SaveGames((request, response)));
}

fn receive_load_game(request: i64, response: IosGCLoadGamesResponse) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::LoadGame((request, response)));
}

fn receive_achievement_progress(request: i64, response: IosGCAchievementProgressResponse) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::AchievementProgress((
            request, response,
        )));
}

fn receive_achievement_reset(request: i64, response: IosGCAchievementsResetResponse) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::AchievementsReset((request, response)));
}

fn receive_leaderboard_score(request: i64, response: IosGCScoreSubmitResponse) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::LeaderboardScoreSubmitted((
            request, response,
        )));
}

fn receive_deleted_game(request: i64, response: IosGCDeleteSaveGameResponse) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::DeletedSaveGame((request, response)));
}

fn receive_items_for_signature_verification(
    request: i64,
    response: IosGCFetchItemsForSignatureVerificationResponse,
) {
    #[cfg(target_os = "ios")]
    SENDER.get().unwrap().as_ref().unwrap().send(
        IosGamecenterEvents::ItemsForSignatureVerification((request, response)),
    );
}

fn receive_resolved_conflicts(request: i64, response: IosGCResolvedConflictsResponse) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::ResolvedConflicts((request, response)));
}

fn receive_conflicting_savegames(savegames: IosGCSaveGames) {
    #[cfg(target_os = "ios")]
    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::ConflictingSaveGames(savegames));
}
