use std::env;
use std::fs;
use std::io::Error;
use std::path::Path;
use std::process;
use std::str;

const THEME: &str = "default";

fn get_preamble(theme: &str) -> Result<String, Error> {
    let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let theme_dir = project_dir.join("src").join("theme").join(theme);
    let preamble = fs::read_to_string(theme_dir.join("preamble.latex"))?;
    Ok(preamble)
}

fn build(path: &str, theme: &str) -> Result<String, Error> {
    let preamble = get_preamble(theme)?;
    let content = fs::read_to_string(path)?;

    let latex = format!(
        r#"{}
\begin{{document}}
{}
\end{{document}}
"#,
        preamble, content
    );

    let pdf_buf: Vec<u8> = tectonic::latex_to_pdf(latex)?;

    // NOTE:
    // Due to the content of the bytes slice, it seems that `str::from_utf8()`
    // returns an error for invalid UTF-8 sequence, and
    // `String::from_utf8_lossy()` generates invalid result.
    let data = unsafe { str::from_utf8_unchecked(&pdf_buf) };
    Ok(data.to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        // not one arg passed
        println!("usage: scenic <file>");
        process::exit(1);
    }

    let path = &args[1];
    match build(path, THEME) {
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
        Ok(data) => {
            print!("{}", data);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::io::ErrorKind;
    use std::path::Path;

    #[test]
    fn test_preamble_not_found() {
        let result = get_preamble("unknown");
        assert!(result.is_err());

        let err = result.err().unwrap();
        assert_eq!(err.kind(), ErrorKind::NotFound);
    }

    #[test]
    fn test_preamble() {
        let expected = "\\documentclass{beamer}";
        let result = get_preamble(THEME);

        assert!(result.unwrap().contains(expected));
    }

    #[test]
    fn test_build_invalid_theme() {
        let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let test_data_dir = project_dir.join("test").join("data");

        let input = test_data_dir.join("input.latex");
        let path = input.to_str().unwrap();

        let result = build(path, "unknown");
        assert!(result.is_err());

        let err = result.err().unwrap();
        assert_eq!(err.kind(), ErrorKind::NotFound);
    }

    #[test]
    fn test_build_missing_input_file_path() {
        let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let test_data_dir = project_dir.join("test").join("data");

        let input = test_data_dir.join("missing.latex");
        let path = input.to_str().unwrap();

        let result = build(path, THEME);
        assert!(result.is_err());

        let err = result.err().unwrap();
        assert_eq!(err.kind(), ErrorKind::NotFound);
    }

    #[test]
    fn test_build_broken_latex_input() {
        let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let test_data_dir = project_dir.join("test").join("data");

        let input = test_data_dir.join("invalid.latex");
        let path = input.to_str().unwrap();

        let result = build(path, THEME);
        assert!(result.is_err());

        let err = result.err().unwrap();
        assert_eq!(err.kind(), ErrorKind::Other);
    }
}
