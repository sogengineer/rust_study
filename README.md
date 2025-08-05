# Rust学習プロジェクト

このプロジェクトは、Rustプログラミング言語の主要な概念を体系的に学習するための包括的なコード例集です。

## プロジェクト構成

```
hello_cargo/
├── src/
│   ├── main.rs              # メインエントリーポイント
│   ├── basics/              # 基本文法
│   │   ├── mod.rs
│   │   └── README.md
│   ├── ownership/           # 所有権システム
│   │   ├── mod.rs
│   │   └── README.md
│   ├── structs_enums/       # 構造体と列挙型
│   │   ├── mod.rs
│   │   └── README.md
│   ├── error_handling/      # エラーハンドリング
│   │   ├── mod.rs
│   │   └── README.md
│   ├── generics_traits/     # ジェネリクスとトレイト
│   │   ├── mod.rs
│   │   └── README.md
│   ├── collections/         # コレクション
│   │   ├── mod.rs
│   │   └── README.md
│   └── testing/             # テスト
│       ├── mod.rs
│       └── README.md
├── tests/                   # 統合テスト
├── Cargo.toml
└── README.md
```

## 使い方

### コードの実行

特定のセクションのコードを実行：

```bash
# 基本文法のコードを実行
cargo run -- basics

# 所有権システムのコードを実行
cargo run -- ownership

# すべてのセクションを実行
cargo run -- all
```

### ドキュメントの表示

各セクションの詳細な説明を表示：

```bash
# 基本文法の詳細説明を表示
cargo run -- doc basics

# 所有権システムの詳細説明を表示
cargo run -- doc ownership

# ドキュメント一覧を表示
cargo run -- doc
```

### テストの実行

```bash
# すべてのテストを実行
cargo test

# 特定のテストを実行
cargo test test_name

# テスト出力を表示
cargo test -- --show-output
```

## 学習セクション

### 1. basics - 基本文法
- 変数と可変性
- データ型（スカラー型、複合型）
- 関数と式
- 制御フロー（if、loop、while、for）

### 2. ownership - 所有権システム
- 所有権の基本ルール
- メモリとアロケーション
- 参照と借用
- スライス型

### 3. structs - 構造体と列挙型
- 構造体の定義と使用
- メソッドと関連関数
- 列挙型とパターンマッチング
- Option<T>型

### 4. error - エラーハンドリング
- panic!マクロ
- Result<T, E>型
- ?演算子
- カスタムエラー型

### 5. generics - ジェネリクスとトレイト
- ジェネリック関数と構造体
- トレイトの定義と実装
- トレイト境界
- ライフタイム

### 6. collections - コレクション
- ベクタ（Vec<T>）
- 文字列（String）
- ハッシュマップ（HashMap<K, V>）

### 7. testing - テスト
- 単体テストの書き方
- 統合テスト
- テストの組織化
- ドキュメンテーションテスト

## 学習の進め方

1. **順番に学習**: basicsから始めて、順番に各セクションを学習することをお勧めします
2. **コードを実行**: 各セクションのコードを実際に実行して動作を確認
3. **ドキュメントを読む**: `cargo run -- doc [section]`で詳細な説明を読む
4. **コードを修正**: 例を修正して実験してみる
5. **テストを書く**: 学んだ内容でテストを書いてみる

## 必要な環境

- Rust 1.70.0以上
- Cargo（Rustに同梱）

## インストール

```bash
# Rustのインストール（未インストールの場合）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# プロジェクトのクローン
git clone [repository-url]
cd hello_cargo

# ビルドと実行
cargo build
cargo run -- basics
```

## 参考資料

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust API Documentation](https://doc.rust-lang.org/std/)

## ライセンス

このプロジェクトは学習目的で作成されています。