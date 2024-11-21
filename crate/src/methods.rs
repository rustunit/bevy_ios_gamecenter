#![allow(unused_variables)]

#[allow(unused_imports)]
use crate::native;
use crate::{IosGCSaveGame, IosGCSaveGames, IosGCViewState};

/// Init internal listener that allows us to receive conflicting save game notifications
pub fn init_listeners() {
    #[cfg(target_os = "ios")]
    native::init_listeners();
}

/// Authenticate
/// Expected to be confirmed with [`IosGamecenterEvents::Authentication`][crate::IosGamecenterEvents::Authentication] event
pub fn authenticate(request: i64) {
    #[cfg(target_os = "ios")]
    native::authenticate(request);
}

/// Request Player Infos
/// Expected to be confirmed with [`IosGamecenterEvents::Player`][crate::IosGamecenterEvents::Player] event
pub fn request_player(request: i64) {
    #[cfg(target_os = "ios")]
    native::get_player(request)
}

/// Save Game under `name`
/// Expected to be confirmed with [`IosGamecenterEvents::SavedGame`][crate::IosGamecenterEvents::SavedGame] event
///
/// ## Note
/// This will base64 encode the data to save it
pub fn save_game(request: i64, name: String, data: &[u8]) {
    #[cfg(target_os = "ios")]
    {
        use base64::Engine;
        let s = base64::engine::general_purpose::STANDARD.encode(data);
        native::save_game(request, s, name);
    }
}

/// Requests the Data inside a given [`IosGCSaveGame`]
/// Expected to be confirmed with [`IosGamecenterEvents::SavedGame`][crate::IosGamecenterEvents::LoadGame] event
pub fn load_game(request: i64, save_game: IosGCSaveGame) {
    #[cfg(target_os = "ios")]
    native::load_game(request, save_game);
}

/// Delete Save Game by name
/// Expected to be confirmed with [`IosGamecenterEvents::DeletedSaveGame`][crate::IosGamecenterEvents::DeletedSaveGame] event
/// See <https://developer.apple.com/documentation/gamekit/gklocalplayer/1520951-deletesavedgameswithname>
pub fn delete_savegame(request: i64, name: String) {
    #[cfg(target_os = "ios")]
    native::delete_game(request, name);
}

/// Resolve conflicting save games.
/// Conflicts will be reported via the [`IosGamecenterEvents::ConflictingSaveGames`][crate::IosGamecenterEvents::ConflictingSaveGames] event.
/// Define the resolved save game via `data`, the conflicting save games will share the same name and the resolved save game will have a new timestamp.
/// Expected to be confirmed with [`IosGamecenterEvents::ResolvedConflicts`][crate::IosGamecenterEvents::ResolvedConflicts] event.
/// See <https://developer.apple.com/documentation/gamekit/gklocalplayer/1521116-resolveconflictingsavedgames>
pub fn resolve_conflicting_games(request: i64, save_games: IosGCSaveGames, data: &[u8]) {
    #[cfg(target_os = "ios")]
    {
        use base64::Engine;
        let s = base64::engine::general_purpose::STANDARD.encode(data);
        native::resolve_conflicting_games(request, save_games, s);
    }
}

/// Fetch Items for Signature Verification
/// Expected to be confirmed with [`IosGamecenterEvents::ItemsForSignatureVerification`][crate::IosGamecenterEvents::ItemsForSignatureVerification] event
/// See <https://developer.apple.com/documentation/gamekit/gklocalplayer/3516283-fetchitemsforidentityverificatio>
pub fn fetch_signature(request: i64) {
    #[cfg(target_os = "ios")]
    native::fetch_signature(request);
}

/// Requests a list of all available SaveGames
/// Expected to be confirmed with [`IosGamecenterEvents::SaveGames`][crate::IosGamecenterEvents::SaveGames] event
pub fn fetch_save_games(request: i64) {
    #[cfg(target_os = "ios")]
    native::fetch_save_games(request);
}

/// Updates progress on an Achievement.
/// Expected to be confirmed with [`IosGamecenterEvents::AchievementProgress`][crate::IosGamecenterEvents::AchievementProgress] event
pub fn achievement_progress(request: i64, id: String, progress: f64) {
    #[cfg(target_os = "ios")]
    native::achievement_progress(request, id, progress);
}

/// Resets all achievements.
/// Expected to be confirmed with [`IosGamecenterEvents::AchievementsReset`][crate::IosGamecenterEvents::AchievementsReset] event
pub fn achievements_reset(request: i64) {
    #[cfg(target_os = "ios")]
    native::reset_achievements(request);
}

/// Submits score to a leaderboard
/// Expected to be confirmed with [`IosGamecenterEvents::LeaderboardScoreSubmitted`][crate::IosGamecenterEvents::LeaderboardScoreSubmitted] event
pub fn leaderboards_score(request: i64, id: String, score: i64, context: i64) {
    #[cfg(target_os = "ios")]
    native::leaderboards_score(request, id, score, context);
}

/// Opens Gamecenter View to a specific [`IosGCViewState`]
///
/// ## Note
/// Unlike most other methods this will not trigger a response
///
/// See Apple Docs: <https://developer.apple.com/documentation/gamekit/gkaccesspoint/3606333-trigger>
pub fn trigger_view(state: IosGCViewState) {
    #[cfg(target_os = "ios")]
    native::trigger_view(state);
}
