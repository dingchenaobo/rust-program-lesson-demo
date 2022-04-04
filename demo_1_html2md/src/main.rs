use html2md;
use reqwest;
use std::fs;

fn main() {
    const URL: &str = "https://www.rust-lang.org";
    const OUT_PUT: &str = "rust-lang.rust.md";

    let response_txt = reqwest::blocking::get(URL).unwrap().text().unwrap();
    let markdown = html2md::parse_html(&response_txt);

    fs::write(OUT_PUT, markdown).unwrap();
}
