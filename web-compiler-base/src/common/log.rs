use std::path::Path;

const VERBOSE_DEBUG_MODE: bool = false;

pub fn log_error(error: &dyn std::error::Error, file_path: Option<&Path>, original: Option<&Path>) {
    eprintln!("{}", format_error(error, file_path, original))
}

pub fn format_error(error: &dyn std::error::Error, file_path: Option<&Path>, original: Option<&Path>) -> String {
    let mut parts = vec![format!("{}", error)];
    let mut source = error.source();

    while let Some(err) = source {
        parts.push(format!("{}", err));
        source = err.source();
    }

    let error_chain = parts.join(" : ");

    let cwd = std::env::current_dir().unwrap();
    let leading = if VERBOSE_DEBUG_MODE {
        format!("[{}] ", cwd.display())
    } else {
        String::default()
    };

    match (file_path, original) {
        (Some(file_path), Some(original)) => {
            let file_path = file_path.display();
            let original = original.display();
            return format!("{leading}Error while processing '{file_path}' ({original}): {error_chain}");
        }
        (Some(file_path), None) => {
            let file_path = file_path.display();
            return format!("{leading}Error while processing '{file_path}': {error_chain}");
        }
        (None, Some(original)) => {
            let original = original.display();
            return format!("{leading}{original} Error: {error_chain}");
        }
        (None, None) => {
            return format!("{leading}Error: {error_chain}");
        }
    }
}
