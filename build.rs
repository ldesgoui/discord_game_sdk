fn main() {
    println!("cargo:rustc-link-lib=discord_game_sdk");
    println!("cargo:rustc-link-search=/home/ldesgoui/discord-game-sdk/target/debug");
}
