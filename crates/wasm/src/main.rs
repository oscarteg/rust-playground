use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

extern crate structopt;
extern crate pulldown_mark;

use pulldown_mark::{html, Parser}

#[derive(StructOpt, Debug)]
#[structopt(name = "rust_wasi_markdown_parser")]
struct Options {
    #[structopt(parse(from_os_str))]
    filename: PathBuf
}

fn main() {
                            let options  = Options::from_args();

    let contents = fs::read_to_string(options.filename)
        .expect("Something went wrong reading the file");

    let result = render_markdown(&contents);
    
}

fn render_markdown(markdown: &str) -> String {
    let mut html_buf = String::new();
    let parser = Parser::new(markdown);
    html::push_html(&mut html_buf, parser);
    html_buf
}
