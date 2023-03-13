use std::env;
use std::fs;
use std::io::Error;
use std::path::Path;
use std::process;
use std::str;

const THEME: &str = "default";

fn build(args: Vec<String>) -> Result<(), Error> {
    if args.len() != 2 {
        // not one arg passed
        println!("usage: scenic <file>");
        process::exit(1);
    }

    let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let theme_dir = project_dir.join("src").join("theme").join(THEME);
    let preamble = fs::read_to_string(theme_dir.join("preamble.latex"))?;

    let path = &args[1];
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
    print!("{}", data);

    Ok(())
}

fn main() {
    let args = env::args().collect();
    if let Err(e) = build(args) {
        eprintln!("{e}");
        process::exit(1);
    }
}
