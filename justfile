check:
	cd ./crate/ && cargo c --target=aarch64-apple-ios-sim
	cd ./crate/ && cargo clippy --target=aarch64-apple-ios-sim

build-rust-release:
	./crate/build-rust-release.sh

build-rust:
	./crate/build-rust.sh

copy-generated:
	sed -i '' 's/func __swift_bridge__/public func __swift_bridge__/g' crate/generated/bevy_ios_gamecenter/bevy_ios_gamecenter.swift
	echo "import BevyIosGamecenterRust "|cat - ./crate/generated/SwiftBridgeCore.swift > /tmp/out && mv /tmp/out ./crate/generated/SwiftBridgeCore.swift
	echo "import BevyIosGamecenterRust "|cat - ./crate/generated/bevy_ios_gamecenter/bevy_ios_gamecenter.swift > /tmp/out && mv /tmp/out ./crate/generated/bevy_ios_gamecenter/bevy_ios_gamecenter.swift
	mv ./crate/generated/SwiftBridgeCore.swift ./crate/generated/SwiftBridgeCoreGC.swift
	mv ./crate/generated/SwiftBridgeCore.h ./crate/generated/SwiftBridgeCoreGC.h
	cp ./crate/generated/bevy_ios_gamecenter/bevy_ios_gamecenter.h ./BevyIosGamecenterRust.xcframework/ios-arm64/Headers/BevyIosGamecenterRust/
	cp ./crate/generated/SwiftBridgeCoreGC.h ./BevyIosGamecenterRust.xcframework/ios-arm64/Headers/BevyIosGamecenterRust/
	cp ./crate/generated/bevy_ios_gamecenter/bevy_ios_gamecenter.h ./BevyIosGamecenterRust.xcframework/ios-arm64_x86_64-simulator/Headers/BevyIosGamecenterRust/
	cp ./crate/generated/SwiftBridgeCoreGC.h ./BevyIosGamecenterRust.xcframework/ios-arm64_x86_64-simulator/Headers/BevyIosGamecenterRust/
	cp ./crate/generated/bevy_ios_gamecenter/bevy_ios_gamecenter.swift ./Sources/bevy_ios_gamecenter/
	cp ./crate/generated/SwiftBridgeCoreGC.swift ./Sources/bevy_ios_gamecenter/

build: build-rust copy-generated
	cp ./crate/target/universal-ios/debug/libbevy_ios_gamecenter.a ./BevyIosGamecenterRust.xcframework/ios-arm64_x86_64-simulator/
	cp ./crate/target/aarch64-apple-ios/debug/libbevy_ios_gamecenter.a ./BevyIosGamecenterRust.xcframework/ios-arm64/

build-release: build-rust-release copy-generated
	cp ./crate/target/universal-ios/release/libbevy_ios_gamecenter.a ./BevyIosGamecenterRust.xcframework/ios-arm64_x86_64-simulator/
	cp ./crate/target/aarch64-apple-ios/release/libbevy_ios_gamecenter.a ./BevyIosGamecenterRust.xcframework/ios-arm64/
	ls -lisah ./BevyIosGamecenterRust.xcframework/ios-arm64/
	ls -lisah ./BevyIosGamecenterRust.xcframework/ios-arm64_x86_64-simulator

zip: build-release
	zip -r BevyIosGamecenterRust.xcframework.zip ./BevyIosGamecenterRust.xcframework/
	ls -lisah BevyIosGamecenterRust.xcframework.zip
	shasum -a 256 BevyIosGamecenterRust.xcframework.zip
	shasum -a 256 BevyIosGamecenterRust.xcframework.zip > BevyIosGamecenterRust.xcframework.sha256.txt