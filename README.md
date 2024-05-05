# bevy_ios_gamecenter

[![crates.io](https://img.shields.io/crates/v/bevy_ios_gamecenter.svg)](https://crates.io/crates/bevy_ios_gamecenter)

Bevy Plugin and Swift Package to provide access to iOS native GameKit (Gamecenter) from inside Bevy Apps
It uses [Swift-Bridge](https://github.com/chinedufn/swift-bridge) to auto-generate the glue code and transport datatypes.

![demo](./assets/demo.gif)

See also [bevy_ios_iap](https://github.com/rustunit/bevy_ios_iap), [bevy_ios_notifications](https://github.com/rustunit/bevy_ios_notifications), [bevy_ios_alerts](https://github.com/rustunit/bevy_ios_alerts), [bevy_ios_review](https://github.com/rustunit/bevy_ios_review) & [bevy_ios_impact](https://github.com/rustunit/bevy_ios_impact)

## Instructions

1. Add to XCode: Add SPM (Swift Package Manager) dependency
2. Add Rust dependency
3. Setup Plugin

### 1. Add to XCode

Go to `File` -> `Add Package Dependencies` and paste `https://github.com/rustunit/bevy_ios_gamecenter.git` into the search bar on the top right:
![xcode](./assets/xcode-spm.png)

### 2. Add Rust dependency

```
cargo add bevy_ios_gamecenter
``` 

or 

```
bevy_ios_gamecenter = { version = "0.1" }
```

### 3. Setup Plugin

Initialize Bevy Plugin:

```rust
// request auth right on startup
app.add_plugins(IosGamecenterPlugin::new(true));
```

```rust
fn bevy_system() {
    bevy_ios_gamecenter::achievements_reset();
    
    // update achievement progress, 100 % will complete it
    bevy_ios_gamecenter::achievement_progress("id".into(),100.);

    bevy_ios_gamecenter::leaderboards_score(
        "raking id".into(),
        // score
        1,
        // context
        2,
    );

    // open gamecenter view (leaderboard)
    bevy_ios_gamecenter::trigger_view(view_states::LEADERBOARDS);

    // save arbitrary binary buffer as a savegame
    bevy_ios_gamecenter::save_game("test".into(), vec![1, 2, 3].as_slice());

    // request list of `IosGCSaveGame`
    bevy_ios_gamecenter::fetch_save_games();

    // based on result of above `fetch_save_games` request
    let save_game = IosGCSaveGame {..} 
    bevy_ios_gamecenter::load_game(save_game);
}
```

Process Response Events from iOS back to us in Rust:

```rust
fn process_gamecenter_events(
    mut events: EventReader<IosGamecenterEvents>,
) {
    for e in events.read() {
        match e {
            IosGamecenterEvents::SaveGames(response) => todo!(),
            IosGamecenterEvents::Player(player) => todo!(),
            IosGamecenterEvents::Authentication(response) => todo!(),
            IosGamecenterEvents::SavedGame(response) => todo!(),
            IosGamecenterEvents::LoadGame(response) => todo!(),
            IosGamecenterEvents::AchievementProgress(response) => todo!(),
            IosGamecenterEvents::AchievementsReset(response) => todo!(),
            IosGamecenterEvents::LeaderboardScoreSubmitted(response) => todo!(),
        }
    }
}
```