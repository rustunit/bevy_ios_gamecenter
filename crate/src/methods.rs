#![allow(unused_variables)]

#[allow(unused_imports)]
use crate::native;
use crate::{IosGCSaveGame, IosGCViewState};

pub fn init() {
    #[cfg(target_os = "ios")]
    native::ios_gc_init();
}

pub fn request_player() {
    if cfg!(target_os = "ios") {
        native::get_player()
    }
}

pub fn save_game(name: String, data: &[u8]) {
    if cfg!(target_os = "ios") {
        use base64::Engine;
        let s = base64::engine::general_purpose::STANDARD.encode(data);
        native::save_game(s, name);
    }
}

pub fn load_game(save_game: IosGCSaveGame) {
    if cfg!(target_os = "ios") {
        native::load_game(save_game);
    }
}

pub fn fetch_save_games() {
    if cfg!(target_os = "ios") {
        native::fetch_save_games();
    }
}

pub fn achievement_progress(id: String, progress: f64) {
    if cfg!(target_os = "ios") {
        native::achievement_progress(id, progress);
    }
}

pub fn achievements_reset() {
    if cfg!(target_os = "ios") {
        native::reset_achievements();
    }
}

pub fn leaderboards_score(id: String, score: i64, context: i64) {
    if cfg!(target_os = "ios") {
        native::leaderboards_score(id, score, context);
    }
}

pub fn trigger_view(state: IosGCViewState) {
    if cfg!(target_os = "ios") {
        native::trigger_view(state);
    }
}
