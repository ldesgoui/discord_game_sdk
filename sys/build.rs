use std::ops::Not;

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
If not, please report any issues you have at:

https://github.com/ldesgoui/discord_game_sdk

Thanks, and apologies for the inconvenience

"#;

const INCOMPATIBLE_PLATFORM: &str = r#"

discord_game_sdk_sys: Hello,

You are trying to compile the bindings for the Discord Game SDK.
Unfortunately, the platform you are trying to target is not
supported by the Discord Game SDK.

You can find more information at:

https://github.com/ldesgoui/discord_game_sdk

Thanks, and apologies for the inconvenience

"#;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // DO NOT RELY ON THIS
    if cfg!(feature = "doc") {
        std::fs::copy("src/.generated.rs", out_path.join("bindings.rs")).unwrap();
        return;
    }

    let sdk_path =
        std::path::PathBuf::from(std::env::var("DISCORD_GAME_SDK_PATH").expect(MISSING_SDK_PATH));

    println!("cargo:rerun-if-env-changed=DISCORD_GAME_SDK_PATH");
    println!("cargo:rerun-if-changed={}", sdk_path.to_str().unwrap());

    bindgen::builder()
        .header(sdk_path.join("c/discord_game_sdk.h").to_str().unwrap())
        .ctypes_prefix("ctypes")
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_eq(true)
        .derive_hash(true)
        .derive_partialeq(true)
        .generate_comments(false)
        .impl_debug(true)
        .impl_partialeq(true)
        .parse_callbacks(Box::new(Callbacks))
        .prepend_enum_name(false)
        .whitelist_function("Discord.+")
        .whitelist_type("[EI]?Discord.+")
        .whitelist_var("DISCORD_.+")
        .generate()
        .expect("discord_game_sdk_sys: bindgen could not generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("discord_game_sdk_sys: could not write bindings to file");

    std::fs::copy(out_path.join("bindings.rs"), "src/.generated.rs").unwrap();

    if cfg!(feature = "link").not() {
        return;
    }

    // For each of the supported platforms, we copy the according library files to OUT_DIR
    // and point the linker there.
    // This (on Linux anyways) allows `cargo run` to properly build and run, finding the library
    // with no additional setup.
    // However, the resulting binary will depend on the library file to be in the runtime path,
    // which this crate does not modify in any way.
    // The `rustc-link-lib` explicitly points to file names because they are not named
    // conventionally (`lib...` on UNIX).
    match target.as_ref() {
        "x86_64-unknown-linux-gnu" => {
            std::fs::copy(
                sdk_path.join("lib/x86_64/discord_game_sdk.so"),
                out_path.join("discord_game_sdk.so"),
            )
            .unwrap();

            println!("cargo:rustc-link-search={}", out_path.to_str().unwrap());
            println!("cargo:rustc-link-lib=dylib=discord_game_sdk::discord_game_sdk.so");
        }

        "x86_64-apple-darwin" => {
            std::fs::copy(
                sdk_path.join("lib/x86_64/discord_game_sdk.dylib"),
                out_path.join("discord_game_sdk.dylib"),
            )
            .unwrap();

            println!("cargo:rustc-link-search={}", out_path.to_str().unwrap());
            println!("cargo:rustc-link-lib=dylib=discord_game_sdk::discord_game_sdk.dylib");
        }

        "i686-pc-windows-gnu"
        | "i686-pc-windows-msvc"
        | "x86_64-pc-windows-gnu"
        | "x86_64-pc-windows-msvc" => {
            let path = if target.starts_with("x86_64") {
                sdk_path.join("lib/x86_64")
            } else {
                sdk_path.join("lib/x86")
            };

            std::fs::copy(
                path.join("discord_game_sdk.dll.lib"),
                out_path.join("discord_game_sdk.lib"),
            )
            .unwrap();

            std::fs::copy(
                path.join("discord_game_sdk.dll"),
                out_path.join("discord_game_sdk.dll"),
            )
            .unwrap();

            println!("cargo:rustc-link-search={}", out_path.to_str().unwrap());
            println!("cargo:rustc-link-lib=dylib=discord_game_sdk:discord_game_sdk");
        }

        _ => panic!(INCOMPATIBLE_PLATFORM),
    }
}

#[derive(Debug)]
struct Callbacks;

impl bindgen::callbacks::ParseCallbacks for Callbacks {
    fn int_macro(&self, name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
        // Must match sys::DiscordVersion
        if name.ends_with("_VERSION") {
            Some(bindgen::callbacks::IntKind::I32)
        } else {
            None
        }
    }
}
