//! Rust には scanf 関数はない。したがって、この節は本来は実装できない。
//! Rust のマクロを使用すると scanf に近い実装を作ることができるので、参考程度に実装しておく。
//! マクロを自前実装する以外の選択肢としては、下記のようなクレートを使用する手もある。
//! - [text_io](https://github.com/oli-obk/rust-si): scan マクロが存在する。
//! - [scan_fmt](https://github.com/wlentz/scan_fmt): 同様に scan_fmt マクロが存在する。

/// scan! マクロ。
/// scan!("<入れた文字>", セパレータの設定, 型情報*) と値を入れていくと、scanf とほぼ同等の動きをする。
///
/// Example:
/// ```rust
/// scan!("123 text", char::is_whitespacce, i32, String);
/// ```
macro_rules! scan {
    ( $string:expr, $sep:expr, $( $x:ty ),+ ) => {{
        let mut iter = $string.split($sep);
        ($(iter.next().map(|word| word.parse::<$x>().ok().expect(&format!("couldn't parse {}", word))).unwrap(),)*)
    }}
}

const SOURCE: &str = "123 1.234 1.0e4 test";

fn main() {
    let (i, f, g, s) = scan!(SOURCE, char::is_whitespace, i32, f64, f64, String);
    println!("i={} f={} g={} s={}", i, f, g, s);
}
