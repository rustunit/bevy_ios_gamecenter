# bevy_ios_gamecenter_egui

This helper crate is for interacting with the ios **gamecenter** api from within your bevy app. It is using [egui](https://github.com/emilk/egui) for the ui.

![demo](../assets/demo.gif)

> Demo from our game using this crate: [zoolitaire.com](https://zoolitaire.com)

# Usage

For now the crate is not published on crates, so it has to be used as a git dependency:

```toml
bevy_ios_gamecenter_egui = { git = "https://github.com/rustunit/bevy_ios_gamecenter.git" }
```

Add this to your code:

```rust
// initialize the plugin on startup
app.add_plugins(IosGamecenterEguiPlugin);

// in any of your bevy systems you can toggle the ui with the following command:
commands.trigger(IosGamecenterEguiOpen::Toggle);
```