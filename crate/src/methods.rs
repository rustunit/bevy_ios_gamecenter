#![allow(unused_variables)]

#[allow(unused_imports)]
use crate::native;
use crate::IosGCSaveGame;

pub fn init() {
    #[cfg(target_os = "ios")]
    native::ios_gc_init();
}

pub fn request_player() {
    if cfg!(target_os = "ios") {
        native::get_player()
    } else {
    }
}

pub fn save_game(name: String, data: &[u8]) {
    if cfg!(target_os = "ios") {
        let s = base64::encode(data);
        native::save_game(s, name);
    } else {
    }
}

pub fn load_game(save_game: IosGCSaveGame) {
    if cfg!(target_os = "ios") {
        native::load_game(save_game);
    } else {
    }
}

pub fn fetch_save_games() {
    if cfg!(target_os = "ios") {
        native::fetch_save_games();
    } else {
    }
}
