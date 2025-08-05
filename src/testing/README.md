# Rustのテスト

Rustは、言語とツールチェーンにテスト機能が組み込まれており、高品質なコードを書くことを支援します。

## 1. テストの基本

### テスト関数の書き方

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn another() {
        panic!("このテストは失敗します");
    }
}
```

### テストの実行

```bash
# すべてのテストを実行
cargo test

# 特定のテストのみ実行
cargo test test_name

# テスト名の一部でフィルタ
cargo test add

# 単一スレッドで実行
cargo test -- --test-threads=1

# 成功時の出力も表示
cargo test -- --show-output

# 無視されたテストも実行
cargo test -- --ignored
```

## 2. アサーションマクロ

### assert!

条件が真であることを確認：

```rust
#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle { width: 8, height: 7 };
    let smaller = Rectangle { width: 5, height: 1 };
    
    assert!(larger.can_hold(&smaller));
}
```

### assert_eq!とassert_ne!

値の等価性を確認：

```rust
#[test]
fn it_adds_two() {
    assert_eq!(add_two(2), 4);
    assert_ne!(add_two(2), 5);
}

// カスタムメッセージ
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "挨拶に名前が含まれていません。値は '{}'",
        result
    );
}
```

## 3. テストの組織化

### 単体テスト

同じファイル内の`tests`モジュールに配置：

```rust
// src/lib.rs
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;  // 親モジュールをインポート
    
    #[test]
    fn internal() {
        assert_eq!(internal_adder(2, 2), 4);
    }
}
```

### 統合テスト

`tests`ディレクトリに別ファイルとして作成：

```rust
// tests/integration_test.rs
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(adder::add_two(2), 4);
}
```

### 共通のテストヘルパー

```rust
// tests/common/mod.rs
pub fn setup() {
    // 共通のセットアップコード
}

// tests/integration_test.rs
mod common;

#[test]
fn test_with_setup() {
    common::setup();
    // テストコード
}
```

## 4. should_panicテスト

パニックすることを期待するテスト：

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("値は1から100の間でなければなりません。入力値: {}", value);
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    
    #[test]
    #[should_panic(expected = "値は1から100の間でなければなりません")]
    fn greater_than_100_with_message() {
        Guess::new(200);
    }
}
```

## 5. Result<T, E>を使うテスト

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2が4ではありません"))
        }
    }
    
    #[test]
    fn files_exist() -> Result<(), std::io::Error> {
        use std::fs::File;
        File::open("hello.txt")?;
        File::open("world.txt")?;
        Ok(())
    }
}
```

注意：`Result`を返すテストでは`#[should_panic]`は使えません。

## 6. テストの制御

### テストを無視する

```rust
#[test]
#[ignore]
fn expensive_test() {
    // 時間のかかるテスト
}

// 無視されたテストのみ実行
// cargo test -- --ignored
```

### 条件付きコンパイル

```rust
#[cfg(test)]
mod tests {
    #[test]
    #[cfg(target_os = "linux")]
    fn linux_only_test() {
        // Linuxでのみ実行
    }
    
    #[test]
    #[cfg(not(target_os = "windows"))]
    fn not_windows_test() {
        // Windows以外で実行
    }
}
```

## 7. ドキュメンテーションテスト

ドキュメントコメント内のコード例もテストされます：

```rust
/// 与えられた数値に2を加算します。
///
/// # Examples
///
/// ```
/// use adder::add_two;
///
/// assert_eq!(add_two(2), 4);
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

/// パニックする例を示すドキュメント
///
/// ```should_panic
/// # use doc_example::divide;
/// divide(10, 0);  // パニック！
/// ```
/// 
/// ```no_run
/// # use doc_example::expensive_function;
/// expensive_function();  // 実行されない
/// ```
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("ゼロで除算");
    }
    a / b
}
```

## 8. ベンチマークテスト

不安定機能（nightlyが必要）：

```rust
#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    
    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }
}
```

安定版では`criterion`クレートを使用：

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

## 9. テストのベストプラクティス

### テストの命名

```rust
#[test]
fn test_add() { /* ... */ }  // 悪い例

#[test]
fn add_two_positive_numbers() { /* ... */ }  // 良い例

#[test]
fn add_negative_to_positive_returns_correct_result() { /* ... */ }  // より詳細
```

### Arrange-Act-Assert パターン

```rust
#[test]
fn test_rectangle_area() {
    // Arrange（準備）
    let rectangle = Rectangle { width: 10, height: 20 };
    
    // Act（実行）
    let area = rectangle.area();
    
    // Assert（検証）
    assert_eq!(area, 200);
}
```

### プロパティベーステスト

`proptest`クレートを使用：

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn doesnt_crash(s: String) {
        my_function(&s);
    }
    
    #[test]
    fn addition_is_commutative(a: i32, b: i32) {
        assert_eq!(a + b, b + a);
    }
}
```

## 10. モックとテストダブル

### 手動モック

```rust
trait Database {
    fn get_user(&self, id: u32) -> Option<User>;
}

struct MockDatabase {
    users: HashMap<u32, User>,
}

impl Database for MockDatabase {
    fn get_user(&self, id: u32) -> Option<User> {
        self.users.get(&id).cloned()
    }
}

#[test]
fn test_with_mock() {
    let mut mock_db = MockDatabase {
        users: HashMap::new(),
    };
    mock_db.users.insert(1, User::new("Alice"));
    
    // テストコード
}
```

### mockallクレート

```rust
use mockall::*;

#[automock]
trait MyTrait {
    fn foo(&self, x: u32) -> u32;
}

#[test]
fn test_with_mockall() {
    let mut mock = MockMyTrait::new();
    mock.expect_foo()
        .with(eq(42))
        .times(1)
        .returning(|x| x + 1);
    
    assert_eq!(mock.foo(42), 43);
}
```

## 11. カバレッジ

tarpaulinを使用（Linux/macOS）：

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

llvm-covを使用：

```bash
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov
cargo llvm-cov --html
```

## まとめ

Rustのテストシステムは：
- 言語に組み込まれている
- 単体テストと統合テストをサポート
- ドキュメントテストで例の正確性を保証
- 豊富なアサーションマクロ
- 並列実行でテストを高速化

これにより、信頼性の高いコードを書くことができます。

## 実行方法

このモジュールのテストを実行するには：

```bash
# テストを実行
cargo test

# 特定のモジュールのテスト
cargo test testing

# ベンチマーク（nightlyが必要）
cargo +nightly bench
```