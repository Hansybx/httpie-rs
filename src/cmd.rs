use clap::{command, Args, Parser, Subcommand};
use colored::Colorize;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::collections::HashMap;
use anyhow::{Ok, Result};

use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

use crate::kv_pair;

#[derive(Parser, Debug)]
#[command(name = "Httpie-rs")]
#[command(author = "Hansybx <hansybx@outlook.com>")]
#[command(version = "1.0")]
#[command(about = "Httpie rust version", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: SubCommond,
}

#[derive(Subcommand, Debug)]
pub enum SubCommond {
    Get(Get),
    Post(Post),
}

// get 子命令
#[derive(Args, Debug)]
pub struct Get {
    #[arg(value_parser = parse_url)]
    url: String,
}

pub async fn get(client: Client, args: &Get) -> Result<()> {
    let res = client.get(&args.url).send().await?;
    Ok(print_response(res).await?)
}

// post 子命令。需要输入一个 URL，和若干个可选的 key=value，用于提供 json body
#[derive(Args, Debug)]
pub struct Post {
    #[arg(value_parser = parse_url)]
    url: String,
    #[arg(value_parser = kv_pair::parse_kv_pair)]
    body: Vec<kv_pair::KvPair>,
}

pub async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let res = client.post(&args.url).json(&body).send().await?;
    Ok(print_response(res).await?)
}

pub fn parse_url(url: &str) -> Result<String> {
    let _url: Url = url.parse()?;
    Ok(url.into())
}

// 打印服务器版本号 + 状态码
fn print_status(res: &Response) {
    let status = format!("{:?} {}", res.version(), res.status()).blue();
    println!("{}\n", status);
}

// 打印服务器返回的 HTTP header
fn print_headers(res: &Response) {
    for (name, value) in res.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }

    print!("\n");
}

// 打印服务器返回的 HTTP body
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            print_body_in_syntect(jsonxf::pretty_print(body).unwrap().as_str())
        }
        _ => println!("{}", body),
    }
}

fn print_body_in_syntect(body_str: &str) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension("rs").unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    // let s = "pub struct Wow { hi: u64 }\nfn blah() -> u64 {}";
    for line in LinesWithEndings::from(body_str) {
        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &ps).unwrap();
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
    println!("")
}

/// 打印整个响应
pub async fn print_response(res: Response) -> Result<()> {
    print_status(&res);
    print_headers(&res);
    let mime = get_content_type(&res);
    let body = res.text().await?;
    print_body(mime, &body);
    Ok(())
}

/// 将服务器返回的 content-type 解析成 Mime 类型
fn get_content_type(res: &Response) -> Option<Mime> {
    res.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}