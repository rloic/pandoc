# Pandoc

## Description

This library is created to create [Pandoc](https://pandoc.org/) filters with [Rust](https://www.rust-lang.org/).

To use this library add the line:

```toml
pandoc = { git = "https://github.com/rloic/pandoc", branch = "pandoc/1.22.1" }
```

To your cargo file.

## Example

Let be a filter [`examble.rs`](src/example.rs):
```rust
use std::io;
use pandoc::definition::Inline;
use pandoc::to_json_filter;

fn caps(inline: Inline) -> Inline {
    match inline {
        Inline::Str(text) => Inline::Str(text.to_uppercase()),
        _ => inline
    }
}

fn main() -> io::Result<()> {
    to_json_filter(&mut caps)
}
```

We can compile this filter with:

```shell
cargo build --bin example
```

To use the filter with Pandoc use:

```shell
pandoc -f markdown -t markdown --filter ./target/debug/example example.md
```

will print:

```markdown
# HELLO
```