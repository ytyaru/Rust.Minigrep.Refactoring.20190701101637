use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { return Err("引数不足。2つ必要です。第一引数に検索文字列、第二引数に検索対象ファイルパス。"); }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("With text:\n{}", contents);
    Ok(())
}
