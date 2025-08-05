# 構造体と列挙型

Rustの構造体（struct）と列挙型（enum）は、カスタムデータ型を定義するための強力な機能です。

## 1. 構造体（Struct）

構造体は、関連するデータをグループ化するための型です。

### 通常の構造体

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// インスタンスの作成
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// フィールドへのアクセス
println!("Email: {}", user1.email);

// 可変の場合、フィールドを変更可能
let mut user = User { /* ... */ };
user.email = String::from("newemail@example.com");
```

### フィールド初期化省略記法

変数名とフィールド名が同じ場合、省略できます：

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,      // email: email の省略形
        username,   // username: username の省略形
        active: true,
        sign_in_count: 1,
    }
}
```

### 構造体更新記法

既存のインスタンスから新しいインスタンスを作成：

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1  // 残りのフィールドはuser1から取得
};
```

注意：この操作で所有権が移動する場合があります。

### タプル構造体

フィールド名なしの構造体：

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// フィールドへのアクセス
println!("Red value: {}", black.0);
```

### ユニット様構造体

フィールドを持たない構造体：

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;
```

主にトレイトの実装に使用されます。

## 2. メソッドとその実装

### メソッドの定義

`impl`ブロックを使用してメソッドを定義：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // メソッド：第一引数は常にself
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // 別のRectangleを引数に取るメソッド
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 可変メソッド
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
    
    // 所有権を取るメソッド（稀）
    fn consume(self) {
        // selfが消費される
    }
}
```

### 関連関数

`self`を取らない関数（静的メソッド）：

```rust
impl Rectangle {
    // コンストラクタとしてよく使われる
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// 呼び出し方
let rect = Rectangle::new(30, 50);
let sq = Rectangle::square(20);
```

### 複数のimplブロック

一つの型に複数の`impl`ブロックを定義可能：

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}
```

## 3. 列挙型（Enum）

列挙型は、取りうる値のバリアントを列挙する型です。

### 基本的な列挙型

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

### データを持つ列挙型

各バリアントは異なる型と量のデータを持てます：

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

### 複雑な列挙型

```rust
enum Message {
    Quit,                       // データなし
    Move { x: i32, y: i32 },    // 構造体のようなフィールド
    Write(String),              // 単一のString
    ChangeColor(i32, i32, i32), // 3つのi32
}
```

### 列挙型のメソッド

構造体と同様に`impl`でメソッドを定義：

```rust
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Text: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change color to RGB({}, {}, {})", r, g, b)
            }
        }
    }
}
```

## 4. Option<T>型

Rustには`null`がありません。代わりに`Option<T>`を使用：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

使用例：

```rust
let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;

// Option<T>とTは異なる型
let x: i8 = 5;
let y: Option<i8> = Some(5);
// let sum = x + y;  // エラー！直接は加算できない
```

## 5. パターンマッチング

### match式

すべてのパターンを網羅する必要があります：

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### バインディングを持つパターン

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

### Option<T>とのマッチング

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### ワイルドカードパターン

`_`はあらゆる値にマッチ：

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),  // その他のすべて
}
```

## 6. if let構文

単一のパターンマッチに便利：

```rust
// matchを使う場合
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}

// if letを使う場合（より簡潔）
if let Some(3) = some_u8_value {
    println!("three");
}

// elseも使える
let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

## 動作原理

### メモリレイアウト

#### 構造体
- フィールドはメモリ上に連続して配置
- コンパイラはパディングを追加して最適化

```rust
struct Example {
    a: u8,   // 1バイト
    b: u32,  // 4バイト（パディングあり）
    c: u8,   // 1バイト
}
// 実際のサイズは12バイト（パディング含む）
```

#### 列挙型
- すべてのバリアントで最大のサイズを確保
- ディスクリミナント（タグ）でバリアントを識別

```rust
enum MyEnum {
    A(u32),      // 4バイト + タグ
    B(u64),      // 8バイト + タグ
    C,           // タグのみ
}
// 全体のサイズは最大バリアント + タグ
```

### パフォーマンス

- **ゼロコスト抽象化**: 構造体とメソッドは手書きのコードと同等の性能
- **インライン化**: 小さなメソッドは自動的にインライン化
- **最適化**: enumのディスクリミナントは最小限のサイズに

### トレイト自動導出

`#[derive]`属性で一般的なトレイトを自動実装：

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}
```

## 実行方法

このモジュールのコードを実行するには：

```bash
cargo run -- structs
```