check:
    cargo c
    cargo c --target=aarch64-apple-ios
    cargo c --target=aarch64-apple-ios-sim
    cargo c --target=aarch64-apple-tvos
    cargo c --target=aarch64-apple-tvos-sim
    cargo fmt -- --check
    cargo clippy
    cargo clippy --target=aarch64-apple-ios
    cargo clippy --target=aarch64-apple-ios-sim
    cargo clippy --target=aarch64-apple-tvos
    cargo clippy --target=aarch64-apple-tvos-sim
    cd bevy_ios_gamecenter_egui && cargo clippy
    cd bevy_ios_gamecenter_egui && cargo clippy --target=aarch64-apple-ios
    cd bevy_ios_gamecenter_egui && cargo clippy --target=aarch64-apple-ios-sim

publish:
    cargo publish
