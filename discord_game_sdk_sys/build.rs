use std::{env, ops::Not, path::PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // DO NOT RELY ON THIS
    if cfg!(feature = "doc") {
        std::fs::copy("src/.generated.rs", out_path.join("bindings.rs")).unwrap();
        return;
    }

    let sdk_path = PathBuf::from(env::var("DISCORD_GAME_SDK_PATH").expect(MISSING_SDK_PATH));

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

    match target.as_ref() {
        "x86_64-unknown-linux-gnu" => {
            assert!(
                sdk_path.join("lib/x86_64/libdiscord_game_sdk.so").exists(),
                MISSING_SETUP
            );
        }

        "x86_64-apple-darwin" => {
            // TODO: assert SDK is in DYLD_LIBRARY_PATH
            assert!(
                sdk_path
                    .join("lib/x86_64/libdiscord_game_sdk.dylib")
                    .exists(),
                MISSING_SETUP
            );
        }

        "x86_64-pc-windows-gnu" | "x86_64-pc-windows-msvc" => {
            assert!(
                sdk_path.join("lib/x86_64/discord_game_sdk.lib").exists(),
                MISSING_SETUP
            );
        }

        "i686-pc-windows-gnu" | "i686-pc-windows-msvc" => {
            assert!(
                sdk_path.join("lib/x86/discord_game_sdk.lib").exists(),
                MISSING_SETUP
            );
        }

        _ => panic!(INCOMPATIBLE_PLATFORM),
    }

    match target.as_ref() {
        "x86_64-unknown-linux-gnu"
        | "x86_64-apple-darwin"
        | "x86_64-pc-windows-gnu"
        | "x86_64-pc-windows-msvc" => {
            println!("cargo:rustc-link-lib=discord_game_sdk");
            println!(
                "cargo:rustc-link-search={}",
                sdk_path.join("lib/x86_64").to_str().unwrap()
            );
        }

        "i686-pc-windows-gnu" | "i686-pc-windows-msvc" => {
            println!("cargo:rustc-link-lib=discord_game_sdk");
            println!(
                "cargo:rustc-link-search={}",
                sdk_path.join("lib/x86").to_str().unwrap()
            );
        }

        _ => {}
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

const MISSING_SDK_PATH: &str = r#"

discord_game_sdk_sys: Hello,

You are trying to generate the bindings for the Discord Game SDK.
You will have to download the SDK yourself.
Here are the links to get it:

https://discordapp.com/developers/docs/game-sdk/sdk-starter-guide
https://dl-game-sdk.discordapp.net/latest/discord_game_sdk.zip

Once you have downloaded it, extract the contents to a folder
and set the environment variable `DISCORD_GAME_SDK_PATH` to its path.

Example:

$ export DISCORD_GAME_SDK_PATH=$HOME/Downloads/discord_game_sdk

Please report any issues you have at:
https://github.com/ldesgoui/discord_game_sdk

Thanks, and apologies for the inconvenience

"#;

const MISSING_SETUP: &str = r#"

discord_game_sdk_sys: Hello,

You are trying to link to the Discord Game SDK.
Some additional set-up is required, namely some files need to be copied for the linker:

# Linux: prepend with `lib` and add to library search path
$ cp $DISCORD_GAME_SDK_PATH/lib/x86_64/{,lib}discord_game_sdk.so
$ export LD_LIBRARY_PATH=${LD_LIBRARY_PATH:+${LD_LIBRARY_PATH}:}$DISCORD_GAME_SDK_PATH/lib/x86_64

# Mac OS: prepend with `lib` and add to library search path
$ cp $DISCORD_GAME_SDK_PATH/lib/x86_64/{,lib}discord_game_sdk.dylib
$ export DYLD_LIBRARY_PATH=${DYLD_LIBRARY_PATH:+${DYLD_LIBRARY_PATH}:}$DISCORD_GAME_SDK_PATH/lib/x86_64

# Windows: copy `*.dll.lib` to `*.lib` (won't affect library search)
$ cp $DISCORD_GAME_SDK_PATH/lib/x86_64/discord_game_sdk.{dll.lib,lib}
$ cp $DISCORD_GAME_SDK_PATH/lib/x86/discord_game_sdk.{dll.lib,lib}

After all this, `cargo build` and `cargo run` should function as expected.

Please report any issues you have at:
https://github.com/ldesgoui/discord_game_sdk

Thanks, and apologies for the inconvenience

"#;

const INCOMPATIBLE_PLATFORM: &str = r#"

discord_game_sdk_sys: Hello,

You are trying to link to the Discord Game SDK.
Unfortunately, the platform you are trying to target is not supported.

Please report any issues you have at:
https://github.com/ldesgoui/discord_game_sdk

Thanks, and apologies for the inconvenience

"#;
