fn main() {
    println!("cargo:rustc-link-lib=discord_game_sdk");
    println!("cargo:rustc-link-search=./target/debug");
}
