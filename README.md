# bevy_ios_gamecenter

[![crates.io][sh_crates]][lk_crates]
[![docs.rs][sh_docs]][lk_docs]
[![discord][sh_discord]][lk_discord]

[sh_crates]: https://img.shields.io/crates/v/bevy_ios_gamecenter.svg
[lk_crates]: https://crates.io/crates/bevy_ios_gamecenter
[sh_docs]: https://img.shields.io/docsrs/bevy_ios_gamecenter
[lk_docs]: https://docs.rs/bevy_ios_iap/latest/bevy_ios_gamecenter/
[sh_discord]: https://img.shields.io/discord/1176858176897953872?label=discord&color=5561E6
[lk_discord]: https://discord.gg/rQNeEnMhus

Bevy Plugin to provide access to iOS and tvOS native GameKit (Gamecenter) from inside Bevy Apps.

Talks to `GameKit` directly via [objc2](https://github.com/madsmtm/objc2) - no Swift package or XCode setup required.

On every other platform the plugin builds and does nothing, so no `cfg` is needed on the calling side.

![demo](./assets/demo.gif)

> Demo from our game using this crate: [zoolitaire.com](https://zoolitaire.com)

## Features
* authentication
* save games (based on iCloud)
* achievements
* leaderboards
* egui based debug ui crate see [bevy_ios_gamecenter_egui folder](./bevy_ios_gamecenter_egui/README.md) (iOS only)

## TODOs
* challenges, matchmaking

## Instructions

1. Add Rust dependency
2. Setup Plugin

**Note:** Game Center still has to be enabled for your app in App Store Connect, and save games
additionally require the iCloud capability. On tvOS the same entitlement applies - and it matters
more there, since an Apple TV has no keyboard worth typing an account into.

### 1. Add Rust dependency

```
cargo add bevy_ios_gamecenter
```

### 2. Setup Plugin

Initialize Bevy Plugin:

```rust
// init right on startup
app.add_plugins(IosGamecenterPlugin::new(true));
```

```rust
fn bevy_system(mut gc: BevyIosGamecenter) {

    gc.authenticate()
        .on_response(|result: On<AuthenticationResult>| match result.response {
            IosGCAuthResult::IsAuthenticated => {},
            IosGCAuthResult::LoginPresented => {},
            IosGCAuthResult::Error(e) => error!("auth error: {e}"),
        });

    // here we request the player info type for the username and more
    // Note: all requests via `gc` of type `BevyIosGamecenter`
    // allow to attach an observer to listen to the response:
    gc.request_player().on_response(on_player_response);

    // update achievement progress, 100 % will complete it
    gc.achievement_progress("id".into(),100.);

    // reset all achievements
    gc.achievements_reset();

    // save a game state as a byte slice
    gc.save_game("test".into(), vec![1, 2, 3].as_slice());

    // request list of `IosGCSaveGame`
    gc.fetch_save_games().on_response(on_response);

    // based on result of above `fetch_save_games` request
    let save_game = IosGCSaveGame {..}
    gc.load_game(save_game);

    // update leaderboard score
    gc.leaderboards_score(
        "raking id".into(),
        // score
        1,
        // context
        2,
    );

}

// opening the gamecenter view has no response, so it is a plain free function
bevy_ios_gamecenter::trigger_view(view_states::LEADERBOARDS);
```

## Our Other Crates

- [bevy_debug_log](https://github.com/rustunit/bevy_debug_log)
- [bevy_device_lang](https://github.com/rustunit/bevy_device_lang)
- [bevy_web_popups](https://github.com/rustunit/bevy_web_popups)
- [bevy_libgdx_atlas](https://github.com/rustunit/bevy_libgdx_atlas)
- [bevy_ios_review](https://github.com/rustunit/bevy_ios_review)
- [bevy_ios_iap](https://github.com/rustunit/bevy_ios_iap)
- [bevy_ios_alerts](https://github.com/rustunit/bevy_ios_alerts)
- [bevy_ios_notifications](https://github.com/rustunit/bevy_ios_notifications)
- [bevy_ios_impact](https://github.com/rustunit/bevy_ios_impact)
- [bevy_ios_safearea](https://github.com/rustunit/bevy_ios_safearea)

## Bevy version support

|bevy|bevy\_ios\_gamecenter|
|----|---|
|0.19|0.7,main|
|0.18|0.6|
|0.17|0.5|
|0.16|0.4|
|0.15|0.3|
|0.14|0.2|
|0.13|0.1|

# License

All code in this repository is dual-licensed under either:

- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer.

## Your contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
