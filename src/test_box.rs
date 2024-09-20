mod configurator;
mod program;

pub fn init(args: Vec<String>) {
    match configurator::args_pars(args) {
        Ok(config) => match config {
            configurator::Config::Help => write_help(),
            configurator::Config::Path(path) => run(path),
        },
        Err(msg) => {
            println!("{}", msg);
            std::process::exit(1);
        }
    }
}

fn write_help() {
    println!(env!("CARGO_PKG_NAME"));
    println!("version: {}", env!("CARGO_PKG_VERSION"));
    println!(env!("CARGO_PKG_DESCRIPTION"));
    println!("\n\tUsage: {} [key]\n", env!("CARGO_PKG_NAME"));
    println!("KEYS:");
    println!("{:20} {}", "-h | --help", "this text");
    println!(
        "{:20} {}",
        "-p | --path <PATH>", "take a path to test program"
    );
}

fn run(path: String) {
    unimplemented!();
}
