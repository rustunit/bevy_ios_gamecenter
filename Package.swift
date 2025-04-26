// swift-tools-version: 5.10
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "bevy_ios_gamecenter",
    platforms: [.iOS("15.0")],
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "bevy_ios_gamecenter",
            targets: ["bevy_ios_gamecenter"])
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .binaryTarget(
            name: "BevyIosGamecenterRust",
            //path: "BevyIosGamecenterRust.xcframework"),
            url:
                "https://github.com/rustunit/bevy_ios_gamecenter/releases/download/rs-0.4.0/BevyIosGamecenterRust.xcframework.zip",
            checksum: "ed3a813eedacd6ae8dec050e5966c9e7cbc0b577289abb7525b118e410dbe40a"),
        .target(
            name: "bevy_ios_gamecenter",
            dependencies: ["BevyIosGamecenterRust"]),
        .testTarget(
            name: "bevy_ios_gamecenterTests",
            dependencies: ["bevy_ios_gamecenter"]),
    ]
)
