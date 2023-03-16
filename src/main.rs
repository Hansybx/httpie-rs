use clap::{command, Parser, Subcommand, Args};

#[derive(Parser,Debug)]
#[command(name = "Httpie-rs")]
#[command(author = "Hansybx <hansybx@outlook.com>")]
#[command(version = "1.0")]
#[command(about = "Httpie rust version", long_about = None)]
struct Opts {
    #[command(subcommand)]
    subcmd: SubCommond,
}

#[derive(Subcommand,Debug)]
enum SubCommond {
    Get(Get),
    Post(Post),
}

// get 子命令
#[derive(Args,Debug)]
struct Get {
    url: String,
}

// post 子命令。需要输入一个 URL，和若干个可选的 key=value，用于提供 json body
#[derive(Args,Debug)]
struct Post {
    url: String,
    body: Vec<String>,
}

fn main() {
    println!("Hello, world!");
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
