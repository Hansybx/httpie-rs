use anyhow::{Ok, Result};
use clap::Parser;
use httpie_rs::cmd;

use reqwest::header;

#[tokio::main]
async fn main() -> Result<()> {
    let opts: cmd::Opts = cmd::Opts::parse();

    let mut headers = header::HeaderMap::new();
    // 为我们的 HTTP 客户端添加一些缺省的 HTTP 头
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let result = match opts.subcmd {
        cmd::SubCommond::Get(ref args) => cmd::get(client, args).await?,
        cmd::SubCommond::Post(ref args) => cmd::post(client, args).await?,
    };
    Ok(result)
}

// 仅在 cargo test 时才编译
#[cfg(test)]
mod tests {
    use httpie_rs::kv_pair;
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(cmd::parse_url("abc").is_err());
        assert!(cmd::parse_url("http://abc.xyz").is_ok());
        assert!(cmd::parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(kv_pair::parse_kv_pair("a").is_err());
        assert_eq!(
            kv_pair::parse_kv_pair("a=1").unwrap(),
            kv_pair::KvPair {
                k: "a".into(),
                v: "1".into()
            }
        );

        assert_eq!(
            kv_pair::parse_kv_pair("b=").unwrap(),
            kv_pair::KvPair {
                k: "b".into(),
                v: "".into()
            }
        );
    }
}
