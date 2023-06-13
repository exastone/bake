
pub fn check_for_bake_config() -> bool {
    let init_file = std::path::Path::new("Bake.toml");
    return init_file.exists();
}
