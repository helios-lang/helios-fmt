fn main() {
    env_logger::init();
    let mut args = std::env::args();
    args.next(); // Skip path to executable

    match args.next() {
        Some(path) => helios_fmt::format_file(&path),
        _ => println!("Formatting a directory is not supported at the moment."),
    }
}
