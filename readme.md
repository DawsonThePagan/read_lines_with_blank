# read_lines_with_blank

Read lines from a file or string while keeping blank lines and not ending on blank lines.

## Usage

```rust
use read_lines_with_blank::*;

let f = File::open("foo.txt")?;
let mut reader = BufReader::new(f);

let lines_file = match read_lines_with_blank(&mut reader) {
    Ok(x) => x,
    Err(e) => return Err(e),
};

let str: &str = "line1\n\n\nline2\n";
let lines_str = match read_lines_with_blank_from_str(str) {
    Ok(x) => x,
    Err(e) => return Err(e),
};
```