# discord_game_sdk

Safe wrapper for the [Discord Game SDK]

This crate provides Rust support to the the following Discord features:

- Activities (Rich Presence)
- Users, Avatars and Relationships
- Lobbies, Matchmaking and Voice communication
- Faux-P2P Networking
- Cloud Synchronized (or not) Storage
- Store transactions


## Status

This library is currently in very early stages, most of the API is implemented but unstable.
There are currently no tests (This will change once [`discord_game_sdk_mock`] is further
developed).


## API stability

API stability is completely uncertain until Discord provides details on their update process
and how breaking changes will be introduced. The SDK documentations clearly mention that the
API is not currently stabilized.


## Safety

This crate relies on the SDK to provide correct data and behavior:
- Non-null pointers to valid memory
- UTF-8, NUL-terminated strings
- Valid enum values
- No mutation of memory it should have no ownership of
- No use of pointers after `destroy` is called


## "Legal" note

This wrapper was informally allowed for publication and distribution by Discord Staff.
I cannot redistribute the SDK files until it is made open-source or is licensed for
redistribution. You will have to follow some instructions when first setting up your project.
Apologies for the inconvenience.

If you're a part of Discord and wish to discuss this, please
email `ldesgoui@gmail.com` or contact `twiikuu#0047`. I mean no harm.


[Discord Game SDK]: https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide
[`discord_game_sdk_mock`]: https://github.com/ldesgoui/discord_game_sdk/tree/master/mock

License: Apache-2.0 OR MIT
