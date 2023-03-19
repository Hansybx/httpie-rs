# httpie-rs

A simple [httpie](https://github.com/httpie/httpie) cli in Rust

```bash
# download
git clone git@github.com:Hansybx/httpie-rs.git
cd httpie-rs

# build
cargo build

# get request
./target/debug/httpie-rs get https://httpbin.org/get

# post request
./target/debug/httpie-rs post https://httpbin.org/post a=1 b=2

# unit test
cargo test
```