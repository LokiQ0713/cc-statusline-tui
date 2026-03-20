mod config;
mod i18n;
mod styles;
mod render;
mod install;
mod log;
mod cache;
mod wizard;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--render") {
        render::run();
    } else {
        wizard::run();
    }
}
