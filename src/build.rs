use pkg_config::Config;

fn main() {
    // if std::env::var_os("CARGO_FEATURE_X11").is_some() {
    //     Config::new().probe("xkbcommon-x11").unwrap();
    // } else {
        Config::new().probe("xkbcommon").unwrap();
    // }
}
