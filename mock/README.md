# discord\_game\_sdk\_mock

Drop-in mock replacement for the [Discord Game SDK](https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide)


## Status

This is currently a work-in-progress, most functions will accept calls without providing meaningful or valid results.
The plan is to provide an interface for the developer to build scenarios, allowing for a mish-mash of unit testing and integration testing, with the major benefits that all happens in memory rather than hitting the network.


## Using the mock in a Rust project

In your `Cargo.toml`, add the lines

    [dependencies]
    discord_game_sdk = { version = "0.1.0", features = ["no_linking"] }
    discord_game_sdk_mock = "0.1.0"

In your project's build.rs, add the following to your `fn main()`:

    println!("cargo:rustc-link-lib=discord_game_sdk");
    println!("cargo:rustc-link-search=./target/debug");

License: MIT OR Apache-2.0
