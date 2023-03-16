use std::{str::FromStr, collections::HashMap};

use anyhow::{anyhow, Ok, Result};
use clap::{command, Args, Parser, Subcommand};
use reqwest::{header, Client, Response, Url};

#[derive(Parser, Debug)]
#[command(name = "Httpie-rs")]
#[command(author = "Hansybx <hansybx@outlook.com>")]
#[command(version = "1.0")]
#[command(about = "Httpie rust version", long_about = None)]
struct Opts {
    #[command(subcommand)]
    subcmd: SubCommond,
}

#[derive(Subcommand, Debug)]
enum SubCommond {
    Get(Get),
    Post(Post),
}

// get 子命令
#[derive(Args, Debug)]
struct Get {
    #[arg(value_parser = parse_url)]
    url: String,
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let res = client.get(&args.url).send().await?;
    println!("{:?}", res.text().await?);
    Ok(())
}

// post 子命令。需要输入一个 URL，和若干个可选的 key=value，用于提供 json body
#[derive(Args, Debug)]
struct Post {
    #[arg(value_parser = parse_url)]
    url: String,
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
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

fn parse_url(url: &str) -> Result<String> {
    let _url: Url = url.parse()?;
    Ok(url.into())
}

#[derive(PartialEq, Clone, Debug)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 我们将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误
            // 从迭代器中取第一个结果作为 key，迭代器返回 Some(T)/None
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取第二个结果作为 value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

/// 因为我们为 KvPair 实现了 FromStr，这里可以直接 s.parse() 得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    let opts: Opts = Opts::parse();
    let client = Client::new();
    let result = match opts.subcmd {
        SubCommond::Get(ref args) => get(client, args).await?,
        SubCommond::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}
