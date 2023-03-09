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