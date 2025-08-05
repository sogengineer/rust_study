# Rustの所有権システム

所有権（Ownership）は、Rustの最も独特で重要な機能です。ガベージコレクションなしでメモリ安全性を保証します。

## 1. 所有権の基本ルール

Rustの所有権には3つの基本ルールがあります：

1. **Rustの各値には、その値の所有者と呼ばれる変数がある**
2. **値の所有者は同時に1つだけ存在できる**
3. **所有者がスコープから外れると、値は破棄される**

## 2. スタックとヒープ

### スタックメモリ
- 固定サイズのデータを格納
- 高速なアクセス（LIFO: Last In, First Out）
- 整数、浮動小数点、ブール値、文字型など

### ヒープメモリ
- 可変サイズのデータを格納
- スタックより遅いアクセス
- String、Vec、Box<T>など

```rust
// スタックに格納
let x = 5;
let y = true;
let c = 'A';

// ヒープに格納
let s = String::from("hello");
let v = vec![1, 2, 3];
```

## 3. 所有権の移動（Move）

### 基本的な移動

ヒープ上のデータは、代入時に所有権が移動します：

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1の所有権がs2に移動

// println!("{}", s1);  // エラー！s1はもう有効ではない
println!("{}", s2);     // OK
```

### なぜ移動が必要か？

移動は**二重解放エラー**を防ぎます：

```rust
// もし移動がなかったら...
{
    let s1 = String::from("hello");
    let s2 = s1;  // もしコピーされたら？
}  // s1とs2の両方がメモリを解放しようとする！危険！
```

### Copy trait

スタック上のデータは`Copy`トレイトを実装しており、コピーされます：

```rust
let x = 5;
let y = x;  // xの値がコピーされる

println!("x = {}, y = {}", x, y);  // 両方使える！
```

Copy traitを持つ型：
- すべての整数型（`i32`, `u64`など）
- ブール型（`bool`）
- 浮動小数点型（`f32`, `f64`）
- 文字型（`char`）
- Copyを実装する型のみを含むタプル

## 4. 関数と所有権

### 値を関数に渡す

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);  // sの所有権が関数に移動
    // println!("{}", s);  // エラー！sはもう使えない

    let x = 5;
    makes_copy(x);  // xはコピーされる
    println!("{}", x);  // OK！xはまだ使える
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}  // ここでsome_stringがスコープを抜け、メモリが解放される

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
```

### 値を返す

関数は値の所有権を返すことができます：

```rust
fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    
    println!("{}", s1);  // OK
    // println!("{}", s2);  // エラー！
    println!("{}", s3);  // OK
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string  // 所有権が呼び出し元に移動
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // 所有権を返す
}
```

## 5. 参照と借用（References and Borrowing）

### 不変参照

参照を使うと、所有権を移動せずに値を使用できます：

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // &s1は参照
    println!("The length of '{}' is {}.", s1, len);  // s1はまだ使える！
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // sはスコープを抜けるが、参照なので値は破棄されない
```

### 可変参照

可変参照を使うと、借用した値を変更できます：

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);  // "hello, world"
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

### 参照のルール

1. **任意の時点で、1つの可変参照か、複数の不変参照のどちらかしか持てない**
2. **参照は常に有効でなければならない**

```rust
let mut s = String::from("hello");

let r1 = &s;      // OK
let r2 = &s;      // OK：複数の不変参照
// let r3 = &mut s;  // エラー！不変参照が存在する間は可変参照不可

println!("{} and {}", r1, r2);
// r1とr2はここで使われなくなる

let r3 = &mut s;  // OK：r1, r2のスコープが終了後
println!("{}", r3);
```

### データ競合の防止

Rustの参照ルールは、コンパイル時にデータ競合を防ぎます：

- 2つ以上のポインタが同時に同じデータにアクセス
- 少なくとも1つのポインタがデータに書き込み
- データへのアクセスを同期するメカニズムがない

## 6. スライス型（Slice）

スライスは、コレクションの一部への参照です：

### 文字列スライス

```rust
let s = String::from("hello world");

let hello = &s[0..5];   // "hello"
let world = &s[6..11];  // "world"

// 省略記法
let hello = &s[..5];    // 0から5まで
let world = &s[6..];    // 6から最後まで
let whole = &s[..];     // 全体
```

### スライスを使った改良

```rust
// 改良前：インデックスを返す
fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// 改良後：スライスを返す
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

### 配列のスライス

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];  // [2, 3]への参照
assert_eq!(slice, &[2, 3]);
```

## 7. 所有権の実践的な使い方

### 所有権を返すパターン

```rust
// タプルを使って複数の値を返す
fn calculate_length_tuple(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)  // 所有権とlengthを返す
}

// 参照を使う（推奨）
fn calculate_length_ref(s: &String) -> usize {
    s.len()
}
```

### Cloneを使う

高コストだが、明示的にデータをコピー：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 深いコピー

println!("s1 = {}, s2 = {}", s1, s2);  // 両方使える
```

## 動作原理

### Drop trait

所有者がスコープを抜けるとき、Rustは自動的に`drop`関数を呼び出します：

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

{
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
}  // ここでd、次にcがドロップされる（LIFO順）
```

### メモリ安全性の保証

Rustの所有権システムは以下を防ぎます：

1. **ダングリングポインタ**: 解放済みメモリへの参照
2. **二重解放**: 同じメモリを2回解放
3. **データ競合**: 並行アクセスによる競合
4. **メモリリーク**: 適切にメモリが解放されない

## パフォーマンスへの影響

- **ゼロコスト**: 所有権チェックはコンパイル時に行われる
- **最適化**: コンパイラは所有権情報を使って最適化を行う
- **予測可能**: メモリの割り当てと解放のタイミングが明確

## 実行方法

このモジュールのコードを実行するには：

```bash
cargo run -- ownership
```