// Rustの学習用プロジェクト
// 各モジュールには文法の学習用コード例が含まれています

mod basics;
mod ownership;
mod structs_enums;
mod error_handling;
mod generics_traits;
mod collections;
mod testing;

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }
    
    match args[1].as_str() {
        "doc" => {
            if args.len() < 3 {
                print_doc_help();
            } else {
                show_documentation(&args[2]);
            }
        }
        "basics" => {
            println!("=== 基本的な文法の学習 ===");
            basics::run_all_basics();
        }
        "ownership" => {
            println!("=== 所有権システムの学習 ===");
            ownership::run_all_ownership();
        }
        "structs" => {
            println!("=== 構造体と列挙型の学習 ===");
            structs_enums::run_all_structs_enums();
        }
        "error" => {
            println!("=== エラーハンドリングの学習 ===");
            error_handling::run_all_error_handling();
        }
        "generics" => {
            println!("=== ジェネリクスとトレイトの学習 ===");
            generics_traits::run_all_generics_traits();
        }
        "collections" => {
            println!("=== コレクションの学習 ===");
            collections::run_all_collections();
        }
        "testing" => {
            println!("=== テストの書き方の学習 ===");
            testing::run_testing_demo();
        }
        "all" => {
            println!("=== 全セクションを実行 ===\n");
            
            println!(">>> 基本的な文法");
            basics::run_all_basics();
            
            println!("\n>>> 所有権システム");
            ownership::run_all_ownership();
            
            println!("\n>>> 構造体と列挙型");
            structs_enums::run_all_structs_enums();
            
            println!("\n>>> エラーハンドリング");
            error_handling::run_all_error_handling();
            
            println!("\n>>> ジェネリクスとトレイト");
            generics_traits::run_all_generics_traits();
            
            println!("\n>>> コレクション");
            collections::run_all_collections();
            
            println!("\n>>> テストの書き方");
            testing::run_testing_demo();
        }
        _ => {
            println!("不明なセクション: {}", args[1]);
            print_help();
        }
    }
}

fn print_help() {
    println!("Rust学習プロジェクト");
    println!("\n使い方: cargo run -- [コマンド] [オプション]");
    println!("\nコマンド:");
    println!("  doc [セクション]  - 指定セクションの詳細なドキュメントを表示");
    println!("  [セクション]      - 指定セクションのコードを実行");
    println!("\n利用可能なセクション:");
    println!("  basics       - 基本的な文法（変数、データ型、関数、制御フロー）");
    println!("  ownership    - 所有権システム（所有権、借用、スライス）");
    println!("  structs      - 構造体と列挙型（struct、enum、パターンマッチング）");
    println!("  error        - エラーハンドリング（panic!、Result、カスタムエラー）");
    println!("  generics     - ジェネリクスとトレイト（型パラメータ、トレイト境界）");
    println!("  collections  - コレクション（Vec、String、HashMap）");
    println!("  testing      - テストの書き方（単体テスト、統合テスト）");
    println!("  all          - 全セクションを実行");
    println!("\n例:");
    println!("  cargo run -- basics      # basicsセクションを実行");
    println!("  cargo run -- doc basics  # basicsの詳細説明を表示");
    println!("  cargo run -- all         # 全セクションを実行");
    println!("  cargo run -- doc         # ドキュメント一覧を表示");
    println!("\nテストの実行:");
    println!("  cargo test");
}

fn print_doc_help() {
    println!("ドキュメント一覧");
    println!("\n使い方: cargo run -- doc [セクション]");
    println!("\n利用可能なドキュメント:");
    println!("  basics       - Rustの基本文法の詳細");
    println!("  ownership    - 所有権システムの詳細");
    println!("  structs      - 構造体と列挙型の詳細");
    println!("  error        - エラーハンドリングの詳細");
    println!("  generics     - ジェネリクスとトレイトの詳細");
    println!("  collections  - コレクションの詳細");
    println!("  testing      - テストの書き方の詳細");
}

fn show_documentation(section: &str) {
    let doc_path = match section {
        "basics" => "src/basics/README.md",
        "ownership" => "src/ownership/README.md",
        "structs" => "src/structs_enums/README.md",
        "error" => "src/error_handling/README.md",
        "generics" => "src/generics_traits/README.md",
        "collections" => "src/collections/README.md",
        "testing" => "src/testing/README.md",
        _ => {
            println!("不明なセクション: {}", section);
            print_doc_help();
            return;
        }
    };
    
    if Path::new(doc_path).exists() {
        match fs::read_to_string(doc_path) {
            Ok(content) => {
                println!("\n{}", content);
            }
            Err(e) => {
                println!("ドキュメントの読み込みエラー: {}", e);
            }
        }
    } else {
        println!("ドキュメントファイルが見つかりません: {}", doc_path);
    }
}