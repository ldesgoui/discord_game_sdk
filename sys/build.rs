fn main() {
    let bindings = bindgen::builder()
        .blacklist_type("__u?int(8|16|32|64)_t")
        .derive_default(true)
        .header("discord_game_sdk.h")
        .prepend_enum_name(false)
        .whitelist_function("Discord.+")
        .whitelist_type("[EI]?Discord.+")
        .whitelist_var("DISCORD_.+")
        .generate()
        .expect("bindgen: could not generate");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("bindgen: could not write");
}
