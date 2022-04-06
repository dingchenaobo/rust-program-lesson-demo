use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};

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
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    /// HTTP 请求的 body
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KVPair>,
}

fn parse_url(_url: &str) -> Result<String> {
    let url: Url = _url.parse()?;
    Ok(url.into())
}

#[derive(Debug)]
struct KVPair {
    k: String,
    v: String,
}

impl FromStr for KVPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(_s: &str) -> Result<KVPair> {
    Ok(_s.parse()?)
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
    let client = Client::new();
    let result = match opts.actiion {
        Action::GET(ref args) => get(client, args).await?,
        Action::POST(ref args) => post(client, args).await?,
    };
    Ok(result)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let res = client.get(&args.url).send().await?;
    println!("{:?}", res.text().await?);
    Ok(())
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let res = client.post(&args.url).json(&body).send().await?;
    println!("{:?}", res.text().await?);
    Ok(())
}
