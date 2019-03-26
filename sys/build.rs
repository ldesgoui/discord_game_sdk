const MISSING_SDK_PATH: &str = r#"

discord_game_sdk_sys: Hello,

You are trying to compile the bindings for the Discord Game SDK.
You will have to download the SDK yourself.
Here are the links to get it:

https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide
https://dl-game-sdk.discordapp.net/latest/discord_game_sdk.zip

Once you have downloaded it, extract the contents to a folder
and set the environment variable `DISCORD_GAME_SDK_PATH` to its path.

Example:

# export DISCORD_GAME_SDK_PATH=$HOME/Downloads/discord_game_sdk/

From there, everything should compile when you run `cargo build` again.
If not, please report any issues you have to

https://github.com/ldesgoui/discord_game_sdk

Thanks, and apologies for the inconvenience

"#;

fn main() {
    println!("cargo:rerun-if-env-changed=DISCORD_GAME_SDK_PATH");

    let target = std::env::var("TARGET").unwrap();
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let sdk_path =
        std::path::PathBuf::from(std::env::var("DISCORD_GAME_SDK_PATH").expect(MISSING_SDK_PATH));

    bindgen::builder()
        .header(sdk_path.join("c/discord_game_sdk.h").to_string_lossy())
        .blacklist_type("__u?int(8|16|32|64)_t")
        .derive_default(true)
        .parse_callbacks(Box::new(Callbacks))
        .prepend_enum_name(false)
        .whitelist_function("Discord.+")
        .whitelist_type("[EI]?Discord.+")
        .whitelist_var("DISCORD_.+")
        .generate()
        .expect("discord_game_sdk_sys: bindgen could not generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("discord_game_sdk_sys: could not write bindings to file");

    if cfg!(feature = "no_linking") {
        return;
    }

    match target.as_ref() {
        "i686-pc-windows-gnu" | "i686-pc-windows-msvc" => {
            println!("cargo:rustc-link-search={:?}", sdk_path.join("lib/x86"));
            println!("cargo:rustc-link-lib=discord_game_sdk.dll");
        }
        "x86_64-pc-windows-gnu" | "x86_64-pc-windows-msvc" => {
            println!("cargo:rustc-link-search={:?}", sdk_path.join("lib/x86_64"));
            println!("cargo:rustc-link-lib=discord_game_sdk.dll");
        }
        "x86_64-apple-darwin" => {
            println!("cargo:rustc-link-search={:?}", sdk_path.join("lib/x86_64"));
            println!("cargo:rustc-link-lib=discord_game_sdk");
        }
        _ => {
            println!(
                "cargo:warning=discord_game_sdk_sys: {}",
                "The target you are building for is not supported by the Discord Game SDK"
            );
            println!(
                "cargo:warning=discord_game_sdk_sys: {}{}",
                "However, a mock library is available, read more at ",
                "https://github.com/ldesgoui/discord_game_sdk"
            );
        }
    }
}

#[derive(Debug)]
struct Callbacks;

impl bindgen::callbacks::ParseCallbacks for Callbacks {
    fn int_macro(&self, name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
        if name.ends_with("_VERSION") {
            Some(bindgen::callbacks::IntKind::I32)
        } else {
            None
        }
    }
}
