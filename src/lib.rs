use std::env::consts::OS;
use std::fs::File;
use std::error::Error;
use std::io::{self, Read};

const NEW_LINE_WINDOWS: &str = "\r\n";
const NEW_LINE_LINUX: &str = "\n";

/// Read lines with blank lines included
///
/// # Example
/// ```norun
/// use read_lines_with_blank::read_lines_with_blank;
///
/// let lines = match read_lines_with_blank("foo_bar.txt") {
///     Ok(x) => x,
///     Err(e) => return Err(e),
/// };
/// ```
pub fn read_lines_with_blank(file: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut ret: Vec<String> = Vec::new();
    let new_line = match OS {
        "linux" => NEW_LINE_LINUX,
        "windows" => NEW_LINE_WINDOWS,
        _ => return Err(Box::from(io::Error::new(io::ErrorKind::Unsupported, "Unsupported OS"))),
    };

    let f = File::open(file)?;
    let mut reader = std::io::BufReader::new(f);

    let mut data: Vec<u8> = Vec::new();
    reader.read_to_end(&mut data)?;
    let str: String = match String::from_utf8(data) {
        Ok(s) => s,
        Err(e) => return Err(Box::from(e)),
    };

    for v in str.split(new_line) {
        ret.push(v.to_string());
    }
    Ok(ret)
}