use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // コマンドライン引数受付
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args);
    println!("query: {}", config.query);
    println!("filename: {}", config.filename);

    // ファイルが見つかりませんでした
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        // ファイルの読み込み中に問題がありました
        .expect("something went wrong reading the file");

    // テキストは\n{}です
    println!("With text:\n{}", contents);
}
struct Config {
    query: String,
    filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < 3 { panic!("引数不足。2つ必要です。第一引数に検索文字列、第二引数に検索対象ファイルパス。"); }
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}

