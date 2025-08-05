# Rustのコレクション

コレクションは、複数の値を格納できるデータ構造です。Rustの標準ライブラリは、効率的で安全なコレクション型を提供します。

## 1. ベクタ（Vec<T>）

ベクタは、同じ型の値を可変長で格納する最も基本的なコレクションです。

### ベクタの作成

```rust
// 空のベクタを作成
let v: Vec<i32> = Vec::new();

// vec!マクロを使用（型推論）
let v = vec![1, 2, 3];

// 初期容量を指定
let mut v = Vec::with_capacity(10);
```

### 要素の追加と削除

```rust
let mut v = Vec::new();

// 要素を追加
v.push(5);
v.push(6);
v.push(7);

// 最後の要素を削除
let last = v.pop();  // Option<T>を返す

// 特定の位置に挿入
v.insert(1, 10);

// 特定の位置から削除
let removed = v.remove(1);

// すべての要素をクリア
v.clear();
```

### 要素へのアクセス

```rust
let v = vec![1, 2, 3, 4, 5];

// インデックスアクセス（パニックの可能性）
let third = &v[2];
println!("3番目の要素: {}", third);

// getメソッド（安全）
match v.get(2) {
    Some(third) => println!("3番目の要素: {}", third),
    None => println!("3番目の要素は存在しません"),
}

// 範囲外アクセスの違い
// let does_not_exist = &v[100];  // パニック！
let does_not_exist = v.get(100);  // None
```

### イテレーション

```rust
let v = vec![100, 32, 57];

// 不変参照でイテレート
for i in &v {
    println!("{}", i);
}

// 可変参照でイテレート
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;  // 各要素に50を加算
}

// 所有権を取ってイテレート
for i in v {  // vはもう使えない
    println!("{}", i);
}
```

### 異なる型を格納する方法

```rust
// 列挙型を使用
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("青")),
    SpreadsheetCell::Float(10.12),
];
```

### 便利なメソッド

```rust
let mut v = vec![1, 2, 3, 4, 5];

// 長さと容量
println!("長さ: {}, 容量: {}", v.len(), v.capacity());

// 条件を満たす要素のみ保持
v.retain(|&x| x % 2 == 0);

// ソート
v.sort();
v.sort_by(|a, b| b.cmp(a));  // 降順

// 重複を削除
v.dedup();

// スライスに変換
let slice: &[i32] = &v;

// 別のベクタと結合
let v2 = vec![6, 7, 8];
v.extend(v2);
```

## 2. 文字列（String）

Rustの文字列は、UTF-8エンコードされたテキストを扱います。

### String vs &str

- `String`：可変長、ヒープ上に格納、所有権あり
- `&str`：不変、文字列スライス、参照

### 文字列の作成

```rust
// 空の文字列
let mut s = String::new();

// 文字列リテラルから
let s = "初期内容".to_string();
let s = String::from("初期内容");

// 容量を指定
let s = String::with_capacity(25);

// フォーマット
let s1 = String::from("Hello");
let s2 = String::from("world");
let s = format!("{}, {}!", s1, s2);
```

### 文字列の更新

```rust
let mut s = String::from("foo");

// 文字列を追加
s.push_str("bar");
s.push_str(" baz");

// 1文字を追加
s.push('!');

// 結合（+演算子）
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // s1の所有権は移動

// format!マクロ（所有権を取らない）
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);
```

### 文字列のインデックス

Rustの文字列は直接インデックスできません：

```rust
let s = String::from("hello");
// let h = s[0];  // エラー！

// 理由：UTF-8の文字は可変長
let hello = String::from("Здравствуйте");
// キリル文字は各2バイト
```

### 文字列のスライス

```rust
let hello = "Здравствуйте";

// バイト単位でスライス（注意が必要）
let s = &hello[0..4];  // "Зд"

// 文字境界でない場所でスライスするとパニック
// let s = &hello[0..1];  // パニック！
```

### 文字列の反復処理

```rust
// 文字単位で反復
for c in "नमस्ते".chars() {
    println!("{}", c);
}

// バイト単位で反復
for b in "नमस्ते".bytes() {
    println!("{}", b);
}

// グラフィームクラスタ（外部クレートが必要）
// use unicode_segmentation::UnicodeSegmentation;
// for g in "नमस्ते".graphemes(true) {
//     println!("{}", g);
// }
```

### 文字列の操作

```rust
let s = String::from("Hello World");

// 検索
println!("含む？ {}", s.contains("World"));
println!("開始？ {}", s.starts_with("Hello"));
println!("終了？ {}", s.ends_with("World"));

// 置換
let new_s = s.replace("World", "Rust");

// 分割
let parts: Vec<&str> = s.split_whitespace().collect();
let parts: Vec<&str> = s.split(',').collect();

// トリミング
let s = String::from("  hello  ");
let trimmed = s.trim();

// 大文字小文字変換
let upper = s.to_uppercase();
let lower = s.to_lowercase();
```

## 3. ハッシュマップ（HashMap<K, V>）

キーと値のペアを格納するコレクションです。

### ハッシュマップの作成

```rust
use std::collections::HashMap;

// 空のハッシュマップ
let mut scores = HashMap::new();

// 要素を追加
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// collectを使った作成
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

### 値へのアクセス

```rust
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

// getメソッド
let team_name = String::from("Blue");
let score = scores.get(&team_name);  // Option<&V>

// 存在チェック
if scores.contains_key(&team_name) {
    println!("チームが存在します");
}

// イテレーション
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### 値の更新

```rust
let mut scores = HashMap::new();

// 値を上書き
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);  // 10を上書き

// 値が存在しない場合のみ挿入
scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);  // 挿入されない

// 既存の値に基づいて更新
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

### 所有権とハッシュマップ

```rust
let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_nameとfield_valueはもう使えない（所有権が移動）

// 参照を格納する場合
let mut map = HashMap::new();
map.insert(&field_name, &field_value);
// field_nameとfield_valueはまだ使える
```

### カスタムハッシュ関数

```rust
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use fxhash::FxHasher;

// 高速なハッシュ関数を使用
type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

let mut map: FxHashMap<i32, &str> = FxHashMap::default();
```

## 4. その他の標準コレクション

### HashSet<T>

重複のない値の集合：

```rust
use std::collections::HashSet;

let mut books = HashSet::new();
books.insert("The Rust Programming Language");
books.insert("Programming Rust");
books.insert("The Rust Programming Language");  // 重複は無視

// 集合演算
let a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
let b: HashSet<i32> = vec![2, 3, 4].into_iter().collect();

let union: HashSet<_> = a.union(&b).collect();         // [1, 2, 3, 4]
let intersection: HashSet<_> = a.intersection(&b).collect();  // [2, 3]
let difference: HashSet<_> = a.difference(&b).collect();      // [1]
```

### VecDeque<T>

両端キュー（double-ended queue）：

```rust
use std::collections::VecDeque;

let mut deque = VecDeque::new();
deque.push_back(1);
deque.push_back(2);
deque.push_front(0);

// 両端から取り出し
let front = deque.pop_front();  // Some(0)
let back = deque.pop_back();    // Some(2)
```

### BinaryHeap<T>

優先度付きキュー（最大ヒープ）：

```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(1);
heap.push(5);
heap.push(2);

// 最大値から取り出される
assert_eq!(heap.pop(), Some(5));
assert_eq!(heap.pop(), Some(2));
assert_eq!(heap.pop(), Some(1));
```

## 5. コレクションの選択指針

### Vec<T>を使う場合
- 要素の順序が重要
- インデックスでアクセスしたい
- 要素を末尾に追加することが多い

### HashMap<K, V>を使う場合
- キーで値を検索したい
- 順序は重要でない
- 高速な検索が必要

### HashSet<T>を使う場合
- 重複を許可しない
- 要素の存在確認が主な操作
- 集合演算を行いたい

### VecDeque<T>を使う場合
- 両端への追加・削除が頻繁
- キューやスタックの実装

### BinaryHeap<T>を使う場合
- 常に最大（最小）値を取得したい
- 優先度付きの処理が必要

## 6. パフォーマンス特性

| 操作 | Vec | HashMap | HashSet | VecDeque |
|------|-----|---------|---------|----------|
| 追加（末尾） | O(1)* | O(1)* | O(1)* | O(1)* |
| 削除（末尾） | O(1) | - | - | O(1) |
| 挿入（任意） | O(n) | O(1)* | O(1)* | O(n) |
| 削除（任意） | O(n) | O(1)* | O(1)* | O(n) |
| 検索 | O(n) | O(1)* | O(1)* | O(n) |
| インデックスアクセス | O(1) | - | - | O(1) |

*平均的なケース（最悪ケースではO(n)になる可能性あり）

## 実行方法

このモジュールのコードを実行するには：

```bash
cargo run -- collections
```