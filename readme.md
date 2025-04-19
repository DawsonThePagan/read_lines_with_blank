# read_lines_with_blank

Read lines from a file while keeping blank lines and not ending on blank lines.

## Usage

```rust
use read_lines_with_blank::read_lines_with_blank;

let f = File::open("foo.txt")?;
let mut reader = BufReader::new(f);

let lines = match read_lines_with_blank(&mut reader) {
    Ok(x) => x,
    Err(e) => return Err(e),
};
```