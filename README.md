# discord_game_sdk

Safe wrapper and bindings for the [Discord Game SDK](https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide).

## Status
This library is currently in very early stages, most of the API is missing.

## "Legal" note
This wrapper was informally allowed for publication and distribution by Discord Staff.
I cannot redistribute the SDK files until it is made open-source or is licensed for redistribution. You will have to follow some instructions when first setting up your project.
If you're a part of Discord and wish to discuss this, please email `ldesgoui@gmail.com` or contact `twiikuu#0047`. I mean no harm.

### Using the mock in a Rust project

In your `Cargo.toml`, add the lines

    [dependencies]
    discord_game_sdk = "0.1.0"
    discord_game_sdk_mock = "0.1.0"

    [dependencies.discord_game_sdk]
    features = ["no_linking"]

In your project's build.rs, add the following to your `fn main()`:

    println!("cargo:rustc-link-lib=discord_game_sdk");
    println!("cargo:rustc-link-search=./target/debug");

License: MIT OR Apache 2.0
