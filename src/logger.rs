use env_logger::Env;
use log::LevelFilter;

pub fn init_logger() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .filter_level(LevelFilter::Info)
        .init();
}