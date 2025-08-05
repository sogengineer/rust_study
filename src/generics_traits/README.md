# ジェネリクスとトレイト

Rustのジェネリクスとトレイトは、コードの重複を削減し、抽象化を可能にする強力な機能です。

## 1. ジェネリクス（Generics）

ジェネリクスは、型パラメータを使用して、複数の型で動作するコードを書くための仕組みです。

### 関数のジェネリクス

#### 重複を削除する前

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    // 同じロジック...
}
```

#### ジェネリクスを使用

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

### 構造体のジェネリクス

```rust
// 単一の型パラメータ
struct Point<T> {
    x: T,
    y: T,
}

// 複数の型パラメータ
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// 使用例
let integer = Point { x: 5, y: 10 };
let float = Point { x: 1.0, y: 4.0 };
let mixed = MixedPoint { x: 5, y: 4.0 };
```

### メソッドのジェネリクス

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 特定の型に対する実装
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 異なる型パラメータを持つメソッド
impl<T, U> MixedPoint<T, U> {
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}
```

### 列挙型のジェネリクス

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// カスタム列挙型
enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    }
}
```

## 2. トレイト（Traits）

トレイトは、型が実装すべきメソッドのセットを定義します。他の言語のインターフェースに似ています。

### トレイトの定義

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

### トレイトの実装

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

### デフォルト実装

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(もっと読む...)")
    }
}

// デフォルト実装を使用
impl Summary for NewsArticle {}

// カスタム実装で上書き
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

### トレイト境界

#### 関数の引数として

```rust
// impl Trait構文（簡潔）
pub fn notify(item: &impl Summary) {
    println!("速報！ {}", item.summarize());
}

// トレイト境界構文（明示的）
pub fn notify<T: Summary>(item: &T) {
    println!("速報！ {}", item.summarize());
}
```

#### 複数のトレイト境界

```rust
use std::fmt::Display;

// + 構文
pub fn notify(item: &(impl Summary + Display)) {
    println!("速報！ {}", item.summarize());
}

// where句（読みやすい）
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // 実装
}
```

### トレイトを返す

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("もちろん、みなさん"),
        reply: false,
        retweet: false,
    }
}
```

注意：異なる型を返すことはできません（動的ディスパッチが必要）。

## 3. ライフタイム（Lifetimes）

ライフタイムは、参照が有効な期間を表す概念です。

### ダングリング参照の防止

```rust
// コンパイルエラー
fn main() {
    let r;
    {
        let x = 5;
        r = &x;  // xはスコープを抜ける
    }
    println!("r: {}", r);  // rはダングリング参照！
}
```

### ライフタイム注釈

```rust
// ライフタイム注釈の構文
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

ライフタイム`'a`は、xとyの参照が有効な期間のうち、短い方を表します。

### 構造体のライフタイム

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意: {}", announcement);
        self.part
    }
}
```

### ライフタイム省略規則

コンパイラは以下の場合、ライフタイムを推論します：

1. 各参照パラメータは独自のライフタイムを取得
2. 入力ライフタイムが1つなら、すべての出力に適用
3. メソッドで`&self`があれば、selfのライフタイムを出力に適用

```rust
// 省略前
fn first_word<'a>(s: &'a str) -> &'a str {
    // ...
}

// 省略後（同等）
fn first_word(s: &str) -> &str {
    // ...
}
```

### 静的ライフタイム

`'static`はプログラム全体の期間を表します：

```rust
let s: &'static str = "静的な文字列リテラル";
```

## 4. 高度なトレイト機能

### 関連型

```rust
pub trait Iterator {
    type Item;  // 関連型
    
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;  // 具体的な型を指定
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

### デフォルト型パラメータ

```rust
use std::ops::Add;

// Addトレイトの定義
trait Add<Rhs = Self> {  // Rhsのデフォルトはself
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

// カスタム型での実装
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```

### スーパートレイト

```rust
use std::fmt;

// DisplayトレイトをスーパートレイトとするOutlinePrint
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();  // Displayが必要
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

## 5. パフォーマンスへの影響

### 単相化（Monomorphization）

ジェネリクスは**ゼロコスト抽象化**です：

```rust
// ジェネリック関数
fn generic_function<T>(x: T) -> T {
    x
}

// コンパイル時に以下のように展開
fn generic_function_i32(x: i32) -> i32 { x }
fn generic_function_char(x: char) -> char { x }
```

### 静的ディスパッチ vs 動的ディスパッチ

```rust
// 静的ディスパッチ（高速）
fn static_dispatch(item: &impl Summary) {
    println!("{}", item.summarize());
}

// 動的ディスパッチ（柔軟）
fn dynamic_dispatch(item: &dyn Summary) {
    println!("{}", item.summarize());
}
```

## 6. 実践的な例

### ジェネリックなデータ構造

```rust
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}
```

### トレイトオブジェクト

```rust
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

## まとめ

- **ジェネリクス**：型の重複を削除し、抽象化を提供
- **トレイト**：共通の振る舞いを定義
- **ライフタイム**：参照の有効期間を保証
- **ゼロコスト**：実行時のオーバーヘッドなし

これらの機能により、Rustは高速で安全な抽象化を実現します。

## 実行方法

このモジュールのコードを実行するには：

```bash
cargo run -- generics
```