# RUST AWS-CURL
This is my first Rust script on Github. The script is powered by libcurl and [Easy Curl library](https://docs.rs/curl/latest/curl/index.html#the-easy-api) 

The script is just another mini curl command aim to authenticate with AWS SigV4 authentication and handles most HTTP methods like GET/POST and custom HTTP header. 

It should also help you to understand how to use `structopt` to parse command line arguments which can fallback to system environment variables for AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY

In order to use AWS SigV4, you need at least `--access-key`, `--secret`, `--region`, and `--service` or you can define them using `--aws-sigv4` together with `--user` (similar to curl command)

In order to use POST method with AWS SigV4, the script also auto URL encode on POST data (if not yet) 

## Dependencies
You need at least libcurl 7.75.0 for Curl to handle AWS SigV4

## How to run
```
cargo build

./target/debug/rust-awscurl --help 

./target/debug/rust-awscurl -v --access-key "AWS_ACCESS_KEY" --secret-key "AWS_SECRET_KEY" \
   --aws-sigv4 "aws:amz:ap-southeast-1:es" -H "Content-type: application/json" -d '{"size":4}' \
   -XPOST "https://awses-xfdssdfds-fsadf.ap-southeast-1.es.amazonaws.com/esindex
```
