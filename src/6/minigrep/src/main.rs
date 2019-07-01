use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn main() {
    // コマンドライン引数受付
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("異常終了します。引数解析に問題が生じました。: {}", err);
        std::process::exit(1);
    });
    println!("query: {}", config.query);
    println!("filename: {}", config.filename);
    // grepを実行する（ファイルから検索する）
    run(config);
}
struct Config {
    query: String,
    filename: String,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 { return Err("引数不足。2つ必要です。第一引数に検索文字列、第二引数に検索対象ファイルパス。"); }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
fn run(config: Config) -> Result<(), Box<Error>> {
    // ファイルが見つかりませんでした
//    let mut f = File::open(config.filename).expect("file not found");
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        // ファイルの読み込み中に問題がありました
        .expect("something went wrong reading the file");

    // テキストは\n{}です
    println!("With text:\n{}", contents);
    Ok(())
}
