use clap::{Parser, Subcommand};
use anyhow::Result;
use reqwest::Url;

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    actiion: Action,
}

#[derive(Subcommand, Debug)]
enum Action {
    GET(Get),
    POST(Post),
}

/// get 请求
#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

/// post 请求
#[derive(Parser, Debug)]
struct Post {
    /// HTTP 请求的 URL
    url: String,
    /// HTTP 请求的 body
    body: Vec<String>,
}

fn parse_url(_url: &str) -> Result<String> {
    let url: Url = _url.parse()?;
    Ok(url.into())
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
