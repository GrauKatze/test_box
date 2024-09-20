use std::io::Write;

pub struct Program {
    path: String,
}

impl Program {
    pub fn new(path: String) -> Self {
        if Self::is_path_valid(&path) {
            Self { path }
        } else {
            println!("Path is not valid");
            std::process::exit(1);
        }
    }

    fn is_path_valid(path: &String) -> bool {
        std::path::Path::new(&path).exists()
    }

    pub fn launch(
        &mut self,
        test_args: Option<&[u8]>,
        test_input: Option<String>,
    ) -> Result<Vec<u8>, String> {
        let local_args = test_args.unwrap_or("".as_bytes());
        let local_input = test_input.unwrap_or("default".to_string());

        let mut exe_program = std::process::Command::new(&self.path)
            .args(["args"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("msg");
        let mut exe_program_input = exe_program.stdin.take().expect("msg");
        std::thread::spawn(move || {
            exe_program_input
                .write_all(local_input.as_bytes())
                .expect("msg");
        });
        let exe_program_output = exe_program.wait_with_output().expect("msg").stdout;
        Ok(exe_program_output)
    }
}
