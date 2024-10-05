mod configurator;
mod program;
pub struct TestBox;
impl TestBox {
    /// Инициализация основного модуля
    pub fn init(args: Vec<String>) {
        match configurator::args_pars(args) {
            Ok(config) => match config {
                configurator::Config::Help => {
                    Self::write_help();
                    std::process::exit(0);
                }
                configurator::Config::Path(path) => Self::run(path),
                configurator::Config::Version => {
                    println!("version: {}", env!("CARGO_PKG_VERSION"));
                    std::process::exit(0);
                }
            },
            Err(msg) => {
                println!("{}", msg);
                std::process::exit(1);
            }
        }
    }

    /// Вывод справочной информации
    fn write_help() {
        println!(env!("CARGO_PKG_NAME"));
        println!("version: {}", env!("CARGO_PKG_VERSION"));
        println!(env!("CARGO_PKG_DESCRIPTION"));
        println!("\n\tUsage: {} [key]\n", env!("CARGO_PKG_NAME"));
        println!("KEYS:");
        println!("{:20} {}", "-h | --help", "this text");
        println!("{:20} {}", "-v | --version", "version");
        println!(
            "{:20} {}",
            "-p | --path <PATH>", "take a path to test program"
        );
    }

    /// Основная функция для работы
    pub fn run(path: String) {
        let mut program = program::Program::new(path);
        let test_input = vec!["test one", "test two", "test 3"];
        for i in test_input {
            println!(
                "{:?}",
                format!("{i}\n")
                    == String::from_utf8(
                        program
                            .launch(Some("test_args".as_bytes()), Some(i.to_string()))
                            .unwrap()
                    )
                    .expect("msg")
            );
        }
    }
}
