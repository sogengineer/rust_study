# Rustのエラーハンドリング

Rustは堅牢なエラー処理メカニズムを提供し、回復可能なエラーと回復不可能なエラーを明確に区別します。

## 1. エラーの種類

### 回復不可能なエラー
- `panic!`マクロで発生
- プログラムを即座に終了
- バグやプログラムの前提条件違反

### 回復可能なエラー
- `Result<T, E>`型で表現
- エラーを適切に処理可能
- ファイル操作、ネットワーク通信など

## 2. panic!マクロ

### 基本的な使い方

```rust
fn main() {
    panic!("crash and burn");
}
```

### パニックが発生する状況

```rust
// 配列の範囲外アクセス
let v = vec![1, 2, 3];
v[99];  // panic!

// unwrap()の失敗
let x: Option<i32> = None;
x.unwrap();  // panic!
```

### パニックのバックトレース

環境変数`RUST_BACKTRACE=1`でスタックトレースを表示：

```bash
RUST_BACKTRACE=1 cargo run
```

### panic時の動作

1. **巻き戻し（unwinding）**：デフォルト
   - スタックを遡ってデストラクタを呼び出す
   - メモリをクリーンアップ

2. **アボート（abort）**：Cargo.tomlで設定可能
   ```toml
   [profile.release]
   panic = 'abort'
   ```

## 3. Result<T, E>型

### 定義

```rust
enum Result<T, E> {
    Ok(T),   // 成功時の値
    Err(E),  // エラー時の値
}
```

### 基本的な使い方

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("ファイルを開けません: {:?}", error);
        }
    };
}
```

### エラーの種類による分岐

```rust
use std::fs::File;
use std::io::ErrorKind;

let f = File::open("hello.txt");

let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("ファイル作成失敗: {:?}", e),
            }
        }
        other_error => {
            panic!("ファイルオープン失敗: {:?}", other_error);
        }
    }
};
```

## 4. エラー処理のショートカット

### unwrap()とexpect()

```rust
// unwrap：エラー時にpanic!
let f = File::open("hello.txt").unwrap();

// expect：カスタムエラーメッセージでpanic!
let f = File::open("hello.txt")
    .expect("hello.txtファイルが見つかりません");
```

### ?演算子

エラーを呼び出し元に伝播：

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;  // エラーなら即return
    let mut s = String::new();
    f.read_to_string(&mut s)?;  // エラーなら即return
    Ok(s)
}
```

### ?演算子の連鎖

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```

### さらに短く

```rust
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

## 5. エラー型の変換

### From trait

`?`演算子は`From`トレイトを使用してエラー型を自動変換：

```rust
// カスタムエラー型
#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError::Io(error)
    }
}

impl From<ParseIntError> for AppError {
    fn from(error: ParseIntError) -> Self {
        AppError::Parse(error)
    }
}

// 使用例
fn complex_operation() -> Result<i32, AppError> {
    let contents = fs::read_to_string("number.txt")?;  // io::Error -> AppError
    let number = contents.trim().parse()?;  // ParseIntError -> AppError
    Ok(number)
}
```

## 6. カスタムエラー型

### Error trait

```rust
use std::fmt;
use std::error::Error;

#[derive(Debug)]
struct MathError {
    kind: MathErrorKind,
}

#[derive(Debug)]
enum MathErrorKind {
    DivisionByZero,
    NegativeSquareRoot,
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            MathErrorKind::DivisionByZero => write!(f, "ゼロ除算エラー"),
            MathErrorKind::NegativeSquareRoot => write!(f, "負の数の平方根"),
            MathErrorKind::Overflow => write!(f, "オーバーフロー"),
        }
    }
}

impl Error for MathError {}
```

## 7. エラー処理のベストプラクティス

### いつpanic!を使うか

1. **例やプロトタイプ**：素早い実装
2. **テスト**：テストの失敗を明示
3. **回復不可能な状態**：プログラムの前提条件違反

```rust
// 不正な状態の例
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("値は1から100の間でなければなりません");
        }
        Guess { value }
    }
}
```

### いつResultを使うか

1. **予期されるエラー**：ファイルが存在しない、ネットワークエラー
2. **ライブラリコード**：呼び出し元に判断を委ねる
3. **回復可能なエラー**：リトライや代替処理が可能

```rust
pub fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    if b == 0.0 {
        Err(MathError {
            kind: MathErrorKind::DivisionByZero,
        })
    } else {
        Ok(a / b)
    }
}
```

## 8. エラー処理パターン

### エラーの早期リターン

```rust
fn process_file() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string("data.txt")?;
    
    if contents.is_empty() {
        return Err("ファイルが空です".into());
    }
    
    // 処理を続行...
    Ok(contents.to_uppercase())
}
```

### Optionの変換

```rust
fn find_user(id: u32) -> Result<User, String> {
    let user = users.get(&id)
        .ok_or_else(|| format!("ユーザーID {} が見つかりません", id))?;
    Ok(user.clone())
}
```

### 複数のエラー型の処理

```rust
use std::error::Error;

// Box<dyn Error>を使用（トレイトオブジェクト）
fn complex_operation() -> Result<(), Box<dyn Error>> {
    let _file = File::open("test.txt")?;
    let _number: i32 = "42".parse()?;
    Ok(())
}
```

## 9. main関数でのエラー処理

```rust
use std::error::Error;
use std::fs::File;

// main関数もResultを返せる
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
```

## 10. 高度なエラー処理

### anyhowクレート（サードパーティ）

```rust
use anyhow::{Context, Result};

fn get_cluster_info() -> Result<ClusterInfo> {
    let data = fs::read_to_string("cluster.json")
        .context("クラスタ設定ファイルの読み込みに失敗")?;
    
    let info: ClusterInfo = serde_json::from_str(&data)
        .context("JSONのパースに失敗")?;
    
    Ok(info)
}
```

### thisエラー（サードパーティ）

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("データが見つかりません")]
    NotFound,
    
    #[error("データベースエラー")]
    DatabaseError(#[from] diesel::result::Error),
    
    #[error("不正なID: {0}")]
    InvalidId(String),
}
```

## 動作原理

### ゼロコスト抽象化

- `Result`は列挙型なので、追加のヒープアロケーションなし
- `?`演算子は単純な`match`式にコンパイルされる
- 最適化により、エラーパスのコードは別の場所に配置

### スタック巻き戻し

panic時の動作：
1. 現在の関数のローカル変数をドロップ
2. 呼び出し元に戻る
3. プロセスを繰り返す
4. main関数まで到達したらプログラム終了

## 実行方法

このモジュールのコードを実行するには：

```bash
cargo run -- error
```