use clap::{Parser, Subcommand};

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

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
