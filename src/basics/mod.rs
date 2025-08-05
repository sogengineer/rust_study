// 基本的な文法の学習
// このモジュールでは、Rustの基本的な文法要素を学びます

// 1. 変数と可変性
pub fn variables_and_mutability() {
    // 不変な変数（デフォルト）
    let x = 5;
    println!("xの値: {}", x);
    
    // 可変な変数
    let mut y = 10;
    println!("yの初期値: {}", y);
    y = 20;
    println!("yの変更後: {}", y);
    
    // シャドーイング
    let z = 5;
    let z = z + 1;  // 同じ名前で新しい変数を作成
    let z = z * 2;
    println!("シャドーイング後のz: {}", z); // 12
    
    // 定数
    const MAX_POINTS: u32 = 100_000;
    println!("定数MAX_POINTS: {}", MAX_POINTS);
}

// 2. データ型
pub fn data_types() {
    // 整数型
    let a: i32 = -42;      // 符号付き32ビット整数
    let b: u64 = 100;      // 符号なし64ビット整数
    let c: usize = 10;     // アーキテクチャ依存のサイズ
    println!("整数型: i32={}, u64={}, usize={}", a, b, c);
    
    // 浮動小数点数
    let x = 2.5;           // f64（デフォルト）
    let y: f32 = 3.14;     // f32
    println!("浮動小数点数: f64={}, f32={}", x, y);
    
    // 真偽値
    let t = true;
    let f: bool = false;
    println!("真偽値: true={}, false={}", t, f);
    
    // 文字型
    let ch = 'A';
    let emoji = '😊';
    let kanji = '漢';
    println!("文字型: {}, {}, {}", ch, emoji, kanji);
    
    // タプル
    let tup: (i32, f64, char) = (500, 6.4, 'a');
    let (x, y, z) = tup;  // 分解
    println!("タプル: ({}, {}, {})", x, y, z);
    println!("タプルの要素アクセス: {}", tup.1);
    
    // 配列
    let arr = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March"];
    let zeros = [0; 5];  // [0, 0, 0, 0, 0]
    println!("配列の最初の要素: {}", arr[0]);
    println!("月: {}", months[1]);
    println!("ゼロ配列の長さ: {}", zeros.len());
}

// 3. 関数
pub fn functions_demo() {
    fn greet(name: &str) {
        println!("こんにちは、{}さん！", name);
    }
    
    fn add(x: i32, y: i32) -> i32 {
        // 最後の式が戻り値（セミコロンなし）
        x + y
    }
    
    fn divide_and_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
        let quotient = dividend / divisor;
        let remainder = dividend % divisor;
        (quotient, remainder)  // タプルで複数の値を返す
    }
    
    greet("太郎");
    
    let sum = add(10, 20);
    println!("10 + 20 = {}", sum);
    
    let (q, r) = divide_and_remainder(17, 5);
    println!("17 ÷ 5 = 商: {}, 余り: {}", q, r);
    
    // 式と文
    let y = {
        let x = 3;
        x + 1  // セミコロンなし = 式として評価される
    };
    println!("ブロック式の結果: {}", y);
}

// 4. 制御フロー
pub fn control_flow() {
    // if式
    let number = 7;
    if number < 5 {
        println!("条件は真");
    } else if number == 5 {
        println!("数値は5");
    } else {
        println!("条件は偽");
    }
    
    // if式で値を返す
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("if式の結果: {}", result);
    
    // loop（無限ループ）
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // ループから値を返す
        }
    };
    println!("ループの結果: {}", result);
    
    // while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("発射！");
    
    // for（最も一般的なループ）
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("配列の要素: {}", element);
    }
    
    // 範囲を使ったfor
    for i in 1..4 {  // 1, 2, 3（4は含まない）
        println!("カウント: {}", i);
    }
    
    // 逆順
    for i in (1..4).rev() {
        println!("カウントダウン: {}", i);
    }
}

// 5. コメントとドキュメンテーション
/// この関数は2つの数値の最大値を返します
/// 
/// # 例
/// 
/// ```
/// let max = find_max(10, 20);
/// assert_eq!(max, 20);
/// ```
pub fn find_max(a: i32, b: i32) -> i32 {
    // 通常のコメント
    if a > b {
        a  // 行末コメント
    } else {
        b
    }
}

// メインの実行関数
pub fn run_all_basics() {
    println!("\n=== 変数と可変性 ===");
    variables_and_mutability();
    
    println!("\n=== データ型 ===");
    data_types();
    
    println!("\n=== 関数 ===");
    functions_demo();
    
    println!("\n=== 制御フロー ===");
    control_flow();
    
    println!("\n=== 最大値の検索 ===");
    let max = find_max(42, 38);
    println!("42と38の最大値: {}", max);
}