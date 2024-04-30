use std::sync::OnceLock;

use bevy_crossbeam_event::CrossbeamEventSender;

#[allow(unused_imports)]
pub use ffi::*;

use crate::{plugin::IosGamecenterEvents, IosGCAuthResult, IosGCPlayer};

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

        fn authentication(result: IosGCAuthResult);
        fn receive_player(p: IosGCPlayer);
    }

    extern "Swift" {
        pub fn ios_gc_init();
    }

    extern "Swift" {

        fn get_player();
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
    bevy_log::info!("player: {p:?}");

    SENDER
        .get()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(IosGamecenterEvents::Player(p));
}
