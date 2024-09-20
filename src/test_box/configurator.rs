pub enum Config {
    Path(String),
    Help,
}

pub fn args_pars(args: Vec<String>) -> Result<Config, String> {
    if args.len() > 0 {
        match args[0].as_str() {
            "-h" | "--help" => Ok(Config::Help),
            "-p" | "--path" => Ok(Config::Path(args[1].to_string())),
            _ => Err(format!("not found this argument\n{:?}", args)),
        }
    } else {
        Err("need more argument\n".to_string())
    }
}
