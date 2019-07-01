use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // コマンドライン引数受付
    let args: Vec<String> = std::env::args().collect();
    let query, filename = parse_config(&args);
    println!("query: {}", query);
    println!("filename: {}", filename);

    // ファイルが見つかりませんでした
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        // ファイルの読み込み中に問題がありました
        .expect("something went wrong reading the file");

    // テキストは\n{}です
    println!("With text:\n{}", contents);
}
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}
