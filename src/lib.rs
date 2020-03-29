use std::error::Error;
use std::fs::File; // ファイルを扱う
use std::io::prelude::*; // ファイル入出力を含む入出力処理をするのに有用なトレイト
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // cloneメソッドで、Configインスタンスが所有するデータの総コピーが生成される
        // 文字列データへの参照を保持するより時間とメモリを消費するが、
        // 参照のライフライム管理が不要に
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // queryとfilenameが関連しており、それが設定に関連する変数だと明確に示せる
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    // ()はユニット型
    // Box<Error>はトレイトオブジェクト
    // 関数がErrorトレイトを実装する型を返すが、戻り値の型を具体的に指定しなくても良くなる

    // ?演算子は、パニックでなく、呼び出し元が処理できるようにする
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        // line() => 文字列を行ごとに繰り返すメソッド
        if line.contains(query) {
            // contains() => 行がquery文字列を含むか確認
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a
    str> {
        let query = query.to_lowercase();
        let mut results = Vec::new();

        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                results.push(line);
            }
        }

        results
    }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive"],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}