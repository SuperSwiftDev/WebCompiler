use std::path::Path;
use colored::Colorize;

// ————————————————————————————————————————————————————————————————————————————
// LOGGER
// ————————————————————————————————————————————————————————————————————————————
pub trait Logger {
    fn log(&mut self, msg: &str);
}

// ————————————————————————————————————————————————————————————————————————————
// FILE LOGGER
// ————————————————————————————————————————————————————————————————————————————

#[derive(Debug)]
pub struct FileLogger {
    pub file: std::fs::File,
}

impl FileLogger {
    pub fn new(file_path: impl AsRef<Path>) -> Self {
        let file_path = file_path.as_ref();
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)
            .unwrap();
        FileLogger { file }
    }
}

impl Logger for FileLogger {
    fn log(&mut self, msg: &str) {
        use std::io::Write;
        let _ = write!(&self.file, "{msg}").unwrap();
    }
}


// ————————————————————————————————————————————————————————————————————————————
// STDOUT
// ————————————————————————————————————————————————————————————————————————————
#[derive(Debug, Clone)]
pub struct StdOutLogger {
    pub colorize: bool,
}

impl StdOutLogger {
    pub fn new() -> Self { Self::default() }
    pub fn with_colorize(mut self, colorize: bool) -> Self {
        self.colorize = colorize;
        self
    }
}

impl Default for StdOutLogger {
    fn default() -> Self {
        Self { colorize: true }
    }
}


impl Logger for StdOutLogger {
    fn log(&mut self, msg: &str) {
        if self.colorize {
            let msg = msg.bright_red();
            print!("{msg}");
            return
        }
        print!("{msg}");
    }
}

// ————————————————————————————————————————————————————————————————————————————
// STDOUT
// ————————————————————————————————————————————————————————————————————————————
#[derive(Debug, Clone)]
pub struct StdErrLogger {
    pub colorize: bool,
}

impl StdErrLogger {
    pub fn new() -> Self { Self::default() }
    pub fn with_colorize(mut self, colorize: bool) -> Self {
        self.colorize = colorize;
        self
    }
}

impl Default for StdErrLogger {
    fn default() -> Self {
        Self { colorize: true }
    }
}


impl Logger for StdErrLogger {
    fn log(&mut self, msg: &str) {
        if self.colorize {
            let msg = msg.bright_red();
            print!("{msg}");
            return
        }
        eprint!("{msg}");
    }
}

