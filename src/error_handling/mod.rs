// エラーハンドリングの学習
// Rustの堅牢なエラー処理メカニズムについて学びます

use std::fs::File;
use std::io::{self, Read};
use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;

// 1. panic!マクロ（回復不可能なエラー）
pub fn panic_example() {
    println!("panic!の例（コメントアウトを外すとプログラムが終了します）");
    
    // panic!("クラッシュしました！");
    
    // 条件付きpanic
    let v = vec![1, 2, 3];
    // v[99];  // パニック！インデックスが範囲外
    
    // 明示的なチェック
    if v.len() <= 2 {
        println!("ベクタが小さすぎます");
    } else {
        println!("3番目の要素: {}", v[2]);
    }
}

// 2. Result型（回復可能なエラー）
pub fn result_basics() {
    // ファイルを開く
    let f = File::open("hello.txt");
    
    let _f = match f {
        Ok(file) => {
            println!("ファイルを開きました");
            file
        }
        Err(error) => {
            println!("ファイルを開く際のエラー: {:?}", error);
            return;
        }
    };
    
    // 異なるエラーの処理
    use std::io::ErrorKind;
    
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("ファイルが見つからないので作成します");
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => {
                        println!("ファイル作成エラー: {:?}", e);
                        return;
                    }
                }
            }
            other_error => {
                println!("ファイルを開く際の問題: {:?}", other_error);
                return;
            }
        },
    };
}

// 3. unwrapとexpect
pub fn unwrap_expect_example() {
    // unwrap: 成功時は値を返し、失敗時はpanic!
    // let f = File::open("hello.txt").unwrap();
    
    // expect: unwrapと同じだが、カスタムエラーメッセージ
    // let f = File::open("hello.txt")
    //     .expect("hello.txtファイルを開けませんでした");
    
    // 安全な例
    let content = "42";
    let number: i32 = content.parse()
        .expect("数値の解析に失敗しました");
    println!("解析された数値: {}", number);
}

// 4. エラーの伝播
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("username.txt");
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut s = String::new();
    
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// ?演算子を使った簡潔な書き方
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// さらに短く
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 最も短い（標準ライブラリの関数を使用）
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("username.txt")
}

pub fn error_propagation_example() {
    match read_username_from_file() {
        Ok(username) => println!("ユーザー名: {}", username),
        Err(e) => println!("ユーザー名の読み取りエラー: {}", e),
    }
}

// 5. カスタムエラー型
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "ゼロによる除算"),
            MathError::NegativeSquareRoot => write!(f, "負の数の平方根"),
            MathError::Overflow => write!(f, "オーバーフロー"),
        }
    }
}

impl Error for MathError {}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

pub fn custom_error_example() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 ÷ 2 = {}", result),
        Err(e) => println!("エラー: {}", e),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("結果: {}", result),
        Err(e) => println!("エラー: {}", e),
    }
    
    match sqrt(16.0) {
        Ok(result) => println!("√16 = {}", result),
        Err(e) => println!("エラー: {}", e),
    }
    
    match sqrt(-4.0) {
        Ok(result) => println!("結果: {}", result),
        Err(e) => println!("エラー: {}", e),
    }
}

// 6. 複数のエラー型の処理
#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(std::num::ParseIntError),
    ParseFloat(ParseFloatError),
    Math(MathError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "I/Oエラー: {}", e),
            AppError::Parse(e) => write!(f, "解析エラー: {}", e),
            AppError::ParseFloat(e) => write!(f, "浮動小数点解析エラー: {}", e),
            AppError::Math(e) => write!(f, "数学エラー: {}", e),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(error: std::num::ParseIntError) -> Self {
        AppError::Parse(error)
    }
}

impl From<ParseFloatError> for AppError {
    fn from(error: ParseFloatError) -> Self {
        AppError::ParseFloat(error)
    }
}

impl From<MathError> for AppError {
    fn from(error: MathError) -> Self {
        AppError::Math(error)
    }
}

fn complex_operation() -> Result<f64, AppError> {
    // ファイルから数値を読み取る
    let contents = std::fs::read_to_string("number.txt")?;
    let number: f64 = contents.trim().parse()?;
    
    // 平方根を計算
    let root = sqrt(number)?;
    
    // 10で割る
    let result = divide(root, 10.0)?;
    
    Ok(result)
}

pub fn multiple_error_types_example() {
    // テスト用のファイルを作成
    let _ = std::fs::write("number.txt", "100");
    
    match complex_operation() {
        Ok(result) => println!("複雑な操作の結果: {}", result),
        Err(e) => println!("エラーが発生しました: {}", e),
    }
    
    // クリーンアップ
    let _ = std::fs::remove_file("number.txt");
}

// 7. OptionとResultの変換
pub fn option_result_conversion() {
    // OptionをResultに変換
    let opt: Option<i32> = Some(42);
    let res: Result<i32, &str> = opt.ok_or("値がありません");
    println!("Option -> Result: {:?}", res);
    
    // ResultをOptionに変換
    let res: Result<i32, &str> = Ok(42);
    let opt: Option<i32> = res.ok();
    println!("Result -> Option: {:?}", opt);
    
    // unwrap_or_defaultの使用
    let config: Result<String, io::Error> = Err(io::Error::new(
        io::ErrorKind::NotFound,
        "設定ファイルが見つかりません"
    ));
    let config_value = config.unwrap_or_default();
    println!("設定値（デフォルト）: '{}'", config_value);
    
    // unwrap_or_elseの使用
    let backup_config = || String::from("デフォルト設定");
    let config: Result<String, io::Error> = Err(io::Error::new(
        io::ErrorKind::NotFound,
        "設定ファイルが見つかりません"
    ));
    let config_value = config.unwrap_or_else(|_| backup_config());
    println!("設定値（関数から）: '{}'", config_value);
}

// 8. 実践的な例：設定ファイルの読み込み
#[derive(Debug)]
struct Config {
    debug: bool,
    port: u16,
    host: String,
}

impl Config {
    fn from_file(path: &str) -> Result<Config, AppError> {
        let contents = std::fs::read_to_string(path)?;
        let mut debug = false;
        let mut port = 8080;
        let mut host = String::from("localhost");
        
        for line in contents.lines() {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() != 2 {
                continue;
            }
            
            let key = parts[0].trim();
            let value = parts[1].trim();
            
            match key {
                "debug" => debug = value.parse().unwrap_or(false),
                "port" => port = value.parse()?,
                "host" => host = value.to_string(),
                _ => {}
            }
        }
        
        Ok(Config { debug, port, host })
    }
    
    fn with_defaults() -> Config {
        Config {
            debug: false,
            port: 8080,
            host: String::from("localhost"),
        }
    }
}

pub fn config_example() {
    // 設定ファイルを作成
    let config_content = "debug=true\nport=3000\nhost=0.0.0.0";
    let _ = std::fs::write("config.txt", config_content);
    
    // 設定を読み込む
    let config = Config::from_file("config.txt")
        .unwrap_or_else(|e| {
            println!("設定ファイルの読み込みエラー: {}。デフォルト設定を使用します。", e);
            Config::with_defaults()
        });
    
    println!("設定: {:?}", config);
    
    // クリーンアップ
    let _ = std::fs::remove_file("config.txt");
}

// メインの実行関数
pub fn run_all_error_handling() {
    println!("\n=== Panicの例 ===");
    panic_example();
    
    println!("\n=== Result型の基本 ===");
    result_basics();
    
    println!("\n=== unwrapとexpect ===");
    unwrap_expect_example();
    
    println!("\n=== エラーの伝播 ===");
    error_propagation_example();
    
    println!("\n=== カスタムエラー型 ===");
    custom_error_example();
    
    println!("\n=== 複数のエラー型 ===");
    multiple_error_types_example();
    
    println!("\n=== OptionとResultの変換 ===");
    option_result_conversion();
    
    println!("\n=== 設定ファイルの例 ===");
    config_example();
    
    // クリーンアップ
    let _ = std::fs::remove_file("hello.txt");
}