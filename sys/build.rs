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
    println!("cargo:rerun-if-env-changed=DISCORD_GAME_SDK_PATH");

    let target = std::env::var("TARGET").unwrap();
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // DO NOT RELY ON THIS
    if cfg!(feature = "doc") {
        std::fs::copy("src/.generated.rs", out_path.join("bindings.rs")).unwrap();
        return;
    }

    let sdk_path =
        std::path::PathBuf::from(std::env::var("DISCORD_GAME_SDK_PATH").expect(MISSING_SDK_PATH));

    bindgen::builder()
        .header(sdk_path.join("c/discord_game_sdk.h").to_str().unwrap())
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

    if !cfg!(feature = "link") {
        return;
    }

    match target.as_ref() {
        "x86_64-unknown-linux-gnu" => {
            std::fs::copy(
                sdk_path.join("lib/x86_64/discord_game_sdk.so"),
                out_path.join("libdiscord_game_sdk.so"),
            )
            .unwrap();
        }

        "x86_64-apple-darwin" => {
            std::fs::copy(
                sdk_path.join("lib/x86_64/discord_game_sdk.dylib"),
                out_path.join("discord_game_sdk.dylib"),
            )
            .unwrap();
        }

        "i686-pc-windows-gnu"
        | "i686-pc-windows-msvc"
        | "x86_64-pc-windows-gnu"
        | "x86_64-pc-windows-msvc" => {
            let path = sdk_path.join(if target.starts_with("x86_64") {
                "lib/x86_64"
            } else {
                "lib/x86"
            });

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
        }

        _ => panic!(INCOMPATIBLE_PLATFORM),
    }

    println!("cargo:rustc-link-search={}", out_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=dylib=discord_game_sdk");
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
