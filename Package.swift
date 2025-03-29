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
            targets: ["bevy_ios_gamecenter"]),
    ],
    targets: [
        // Targets are the basic building blocks of a package, defining a module or a test suite.
        // Targets can depend on other targets in this package and products from dependencies.
        .binaryTarget(
            name: "BevyIosGamecenterRust",
            //  path: "BevyIosGamecenterRust.xcframework"),
           url: "https://github.com/rustunit/bevy_ios_gamecenter/releases/download/rs-0.3.1/BevyIosGamecenterRust.xcframework.zip",
           checksum: "05295ada53ee5fbf9864b1050b42d78af6cabff57bc81b3b23096c33dc9a95c5"),
        .target(
            name: "bevy_ios_gamecenter",
            dependencies: ["BevyIosGamecenterRust"]),
        .testTarget(
            name: "bevy_ios_gamecenterTests",
            dependencies: ["bevy_ios_gamecenter"]),
    ]
)
