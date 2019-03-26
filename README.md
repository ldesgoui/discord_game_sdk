# Rust bindings for the Discord Game SDK

### And a mock library for testing (on Linux)

I don't think this currently qualifies as "open source software" as the Discord Game SDK header files are not published under open source licenses. I am not going to redistribute those files. You should see instructions on how to set that up in your first use of the crate.


## Using the mock in a Rust project

(TODO)

In your `Cargo.toml`, add the lines

    [dependencies]
    discord_game_sdk = "0.1.0"
    discord_game_sdk_mock = "0.1.0"

    [dependencies.discord_game_sdk]
    features = ["no_linking"]

Note: you might need to add git/path if the crate isn't published yet (most likely)

In your environment, set the variables

    # export DISCORD_GAME_SDK_MOCK_PATH=$PWD/target/debug/deps
