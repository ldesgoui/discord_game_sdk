# discord_game_sdk_mock

Drop-in mock replacement for the [Discord Game SDK]

### Status

This is currently a work-in-progress, most functions will accept calls
without providing meaningful or valid results.
The plan is to provide an interface for the developer to build scenarios,
allowing for a mish-mash of unit testing and integration testing, with the
major benefits that all happens in memory rather than hitting the network.

[Discord Game SDK]: https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide

License: Apache-2.0 OR MIT
