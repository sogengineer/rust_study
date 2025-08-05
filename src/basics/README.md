# Rustの基本文法

このモジュールでは、Rustプログラミング言語の基本的な文法と概念について学習します。

## 1. 変数と可変性 (Variables and Mutability)

### 変数の宣言

Rustでは、デフォルトで変数は**不変（immutable）**です。これは、一度値を代入すると変更できないことを意味します。

```rust
let x = 5;  // 不変変数
// x = 6;   // エラー！不変変数は再代入できません
```

### 可変変数

変数を可変にするには`mut`キーワードを使用します：

```rust
let mut x = 5;
x = 6;  // OK！可変変数は再代入可能
```

### 定数（Constants）

定数は`const`キーワードで宣言し、常に不変です。定数は型注釈が必須で、コンパイル時に評価される式のみ使用できます：

```rust
const MAX_POINTS: u32 = 100_000;
```

### シャドーイング（Shadowing）

同じ名前で新しい変数を宣言することで、前の変数を「隠す」ことができます：

```rust
let x = 5;
let x = x + 1;        // 新しいxが古いxを隠す
let x = x * 2;        // さらに新しいxが前のxを隠す
let x = "hello";      // 型も変更可能！
```

## 2. データ型 (Data Types)

Rustは**静的型付け言語**で、コンパイル時にすべての変数の型が決定されている必要があります。

### スカラー型

#### 整数型
- 符号付き: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
- 符号なし: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`

```rust
let x: i32 = -42;      // 32ビット符号付き整数
let y: u8 = 255;       // 8ビット符号なし整数
let z: usize = 10;     // アーキテクチャ依存のサイズ
```

#### 浮動小数点型
- `f32`: 32ビット浮動小数点
- `f64`: 64ビット浮動小数点（デフォルト）

```rust
let x = 2.0;      // f64（デフォルト）
let y: f32 = 3.0; // f32
```

#### ブール型
`bool`型は`true`または`false`の値を持ちます：

```rust
let t = true;
let f: bool = false;
```

#### 文字型
`char`型は単一のUnicodeスカラー値を表します：

```rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

### 複合型

#### タプル型
異なる型の値を固定数グループ化します：

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;  // 分解
let first = tup.0;    // インデックスアクセス
```

#### 配列型
同じ型の値を固定数グループ化します：

```rust
let a = [1, 2, 3, 4, 5];
let months = ["January", "February", "March"];
let a: [i32; 5] = [1, 2, 3, 4, 5];  // 型と長さを明示
let a = [3; 5];                      // [3, 3, 3, 3, 3]
```

## 3. 関数 (Functions)

### 関数の定義

関数は`fn`キーワードで定義します：

```rust
fn main() {
    println!("Hello, world!");
}

fn another_function() {
    println!("Another function.");
}
```

### パラメータ

関数パラメータには必ず型注釈が必要です：

```rust
fn print_value(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
```

### 戻り値

関数の戻り値は`->`の後に型を指定します。最後の式が戻り値になります：

```rust
fn five() -> i32 {
    5  // セミコロンなし！式として評価される
}

fn plus_one(x: i32) -> i32 {
    x + 1  // 最後の式が戻り値
}
```

### 式と文

Rustでは**式（expression）**と**文（statement）**の区別が重要です：

- **文**: 値を返さない（セミコロンで終わる）
- **式**: 値を返す（セミコロンで終わらない）

```rust
let y = {
    let x = 3;
    x + 1  // 式：値4を返す
};  // yは4になる

let z = {
    let x = 3;
    x + 1;  // 文：セミコロンがあるので()を返す
};  // zは()になる
```

## 4. 制御フロー (Control Flow)

### if式

`if`は式なので、値を返すことができます：

```rust
let number = 3;

if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}

// 式として使用
let condition = true;
let number = if condition { 5 } else { 6 };
```

### loop式

`loop`は無限ループを作成し、`break`で値を返すことができます：

```rust
let mut counter = 0;
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;  // 20を返す
    }
};
```

### while式

条件が真の間ループします：

```rust
let mut number = 3;
while number != 0 {
    println!("{}!", number);
    number -= 1;
}
```

### for式

コレクションの各要素に対して処理を実行します：

```rust
let a = [10, 20, 30, 40, 50];
for element in a.iter() {
    println!("the value is: {}", element);
}

// 範囲を使用
for number in (1..4).rev() {
    println!("{}!", number);
}
```

## 5. 所有権の予習

Rustの最も重要な特徴の一つが**所有権システム**です。基本的なルール：

1. Rustの各値には、その値の**所有者**と呼ばれる変数がある
2. 値の所有者は同時に1つだけ存在できる
3. 所有者がスコープから外れると、値は破棄される

```rust
{
    let s = String::from("hello");  // sがスコープに入る
    // sを使用する
}  // ここでsがスコープから外れ、メモリが自動的に解放される
```

## 動作原理

### メモリ安全性

Rustは所有権システムによって、ガベージコレクションなしでメモリ安全性を保証します。これにより：
- ダングリングポインタの防止
- データ競合の防止
- メモリリークの防止

### ゼロコスト抽象化

Rustの抽象化は実行時のオーバーヘッドがありません。高レベルの構造も低レベルのコードと同じくらい高速に実行されます。

### 型推論

Rustは強力な型推論を持ち、多くの場合型注釈を省略できます：

```rust
let x = 5;      // i32と推論される
let y = 2.0;    // f64と推論される
```

## 実行方法

このモジュールのコードを実行するには：

```bash
cargo run -- basics
```

または、すべてのセクションを実行：

```bash
cargo run -- all
```