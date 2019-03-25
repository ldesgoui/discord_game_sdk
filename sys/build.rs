const MISSING_SDK_PATH: &str = r#"

Hello,

You are trying to compile the bindings for the Discord Game SDK.
You will have to download the SDK yourself.
Here are the links to get it:

https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide#step-1-get-the-thing
https://dl-game-sdk.discordapp.net/latest/discord_game_sdk.zip

Once you have downloaded it, extract the contents to a folder and set the environment variable `DISCORD_GAME_SDK_PATH` to its path.

# export DISCORD_GAME_SDK_PATH=$PWD/../Downloads/discord_game_sdk/

From there, everything should compile when you run `cargo build` again.

Thanks, and apologies for the inconvenience

"#;


fn main() {
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let sdk_path = std::path::PathBuf::from(std::env::var("DISCORD_GAME_SDK_PATH").expect(MISSING_SDK_PATH));

    let bindings = bindgen::builder()
        .header(sdk_path.join("c/discord_game_sdk.h").to_str().unwrap().to_string())
        .blacklist_type("__u?int(8|16|32|64)_t")
        .derive_default(true)
        .prepend_enum_name(false)
        .whitelist_function("Discord.+")
        .whitelist_type("[EI]?Discord.+")
        .whitelist_var("DISCORD_.+")
        .generate()
        .expect("bindgen: could not generate");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("bindgen: could not write");

    if std::env::var("DISCORD_GAME_SDK_MOCK").is_err() {
        println!("cargo:rustc-link-lib=discord_game_sdk.dll");
        println!("cargo:rustc-link-search={}", sdk_path.join("lib/x86_64").to_str().unwrap());
    } else {
        println!("cargo:rustc-link-lib=discord_game_sdk");
        println!("cargo:rustc-link-search=target/debug");
    }
}
