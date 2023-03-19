
use anyhow::{anyhow, Ok, Result};
use std::str::FromStr;

#[derive(PartialEq, Clone, Debug)]
pub struct KvPair {
    pub k: String,
    pub v: String,
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
pub fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}
