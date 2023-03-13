use std::fs::File;
use std::io::{Error, Read};
use std::path::Path;
use std::str;

use assert_cmd::Command;

fn read_pdf(filepath: &str) -> Result<Vec<u8>, Error> {
    let mut file = File::open(filepath)?;
    let mut buf = vec![];

    file.read_to_end(&mut buf)?;
    let data = unsafe { str::from_utf8_unchecked(&buf) };
    Ok(data.as_bytes().to_vec())
}

#[test]
fn test_output() {
    let project_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let test_data_dir = project_dir.join("test").join("data");

    let result_pdf = test_data_dir.join("result.pdf");
    let expected = read_pdf(result_pdf.to_str().unwrap()).unwrap();

    let mut cmd = Command::cargo_bin("scenic").unwrap();
    cmd.args(&[test_data_dir.join("input.latex")])
        .assert()
        .success()
        .code(0);

    let result = cmd.unwrap().stdout;
    assert_eq!(result, expected);
}
