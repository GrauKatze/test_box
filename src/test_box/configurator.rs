//! Настройка конфигурации работы утилиты

/// Варианты работы утилиты
pub enum Config {
    Path(String),
    /// Вывод справки
    Help,
    /// Вывод версии приложения
    Version,
}

pub fn args_pars(args: Vec<String>) -> Result<Config, String> {
    if args.len() > 0 {
        match args[0].as_str() {
            "-h" | "--help" => Ok(Config::Help),
            "-v" | "--version" => Ok(Config::Version),
            "-p" | "--path" => Ok(Config::Path(args[1].to_string())),
            _ => Err(format!("not found this argument\n{:?}", args)),
        }
    } else {
        Err("need more argument\n".to_string())
    }
}
