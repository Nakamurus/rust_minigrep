extern crate minigrep; // ライブラリクレートをバイナリクレートに持っていく

use std::env; // コマンドライン引数を受け付ける
use std::process;

use minigrep::Config; // Config 型をスコープに持ってくる

fn main() {
 	// collect関数を使い、文字列のベクタがほしいと明示的に注釈
    let args: Vec<String> = env::args().collect();

    // unwrap_or_elseは標準ライブラリで、独自のエラー処理を定義
    // Ok値ならOkが包む中身の値を返す
    // Err値なら、クロージャ内でコードを呼び出す
    // 静的文字列のErrの中身を、縦棒の間のerr引数のクロージャに渡している
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // run 関数にクレート名を接頭辞として付ける
    if let Err(e) = minigrep::run(config) {
        // if letとunwrap_or_elseは同じで、エラーを出力して終了する
        // runは成功時に()を返すので、エラー検出のみに興味がある
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
