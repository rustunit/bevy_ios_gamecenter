#![allow(unused_variables)]

#[allow(unused_imports)]
use crate::native;
use crate::IosGCPlayer;

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
