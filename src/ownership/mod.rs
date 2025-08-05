// 所有権システムの学習
// Rustの最も重要な概念の一つである所有権について学びます

// 1. 所有権の基本ルール
pub fn ownership_basics() {
    // 所有権のルール：
    // 1. Rustの各値は、所有者と呼ばれる変数を持つ
    // 2. 値の所有者は同時に一つだけ
    // 3. 所有者がスコープから外れると、値は破棄される
    
    {
        let s1 = String::from("hello");  // s1がStringの所有者
        println!("s1: {}", s1);
        
        let s2 = s1;  // 所有権がs2に移動（ムーブ）
        // println!("{}", s1);  // エラー！s1はもう使えない
        println!("s2: {}", s2);
    }  // ここでs2がスコープを抜け、メモリが解放される
    
    // スタックに保存される型はコピーされる
    let x = 5;
    let y = x;  // xの値がコピーされる
    println!("x = {}, y = {}", x, y);  // 両方使える！
}

// 2. 参照と借用
pub fn references_and_borrowing() {
    let s1 = String::from("hello");
    
    // 不変な参照（借用）
    let len = calculate_length(&s1);
    println!("'{}'の長さは{}です", s1, len);  // s1はまだ使える！
    
    // 可変な参照
    let mut s = String::from("hello");
    change(&mut s);
    println!("変更後: {}", s);
    
    // 参照のルール：
    // 1. 任意の時点で、1つの可変参照か、複数の不変参照のどちらか
    // 2. 参照は常に有効でなければならない
    
    let r1 = &s;
    let r2 = &s;  // OK：複数の不変参照
    println!("{} and {}", r1, r2);
    
    let r3 = &mut s;  // OK：r1とr2はもう使われない
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()  // sを借用しているだけなので、所有権は移動しない
}

fn change(s: &mut String) {
    s.push_str(", world");
}

// 3. スライス
pub fn slices() {
    let s = String::from("hello world");
    
    // 文字列スライス
    let hello = &s[0..5];  // または &s[..5]
    let world = &s[6..11]; // または &s[6..]
    let whole = &s[..];    // 文字列全体のスライス
    
    println!("前半: '{}', 後半: '{}'", hello, world);
    println!("全体: '{}'", whole);
    
    // 配列スライス
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];  // [2, 3]
    println!("配列スライス: {:?}", slice);
    
    // 実用的な例：最初の単語を見つける
    let first = first_word(&s);
    println!("最初の単語: '{}'", first);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 4. 所有権とメソッド
pub fn ownership_and_functions() {
    // 値を関数に渡す
    let s = String::from("hello");
    takes_ownership(s);  // sの所有権が関数に移動
    // println!("{}", s);  // エラー！sはもう使えない
    
    let x = 5;
    makes_copy(x);  // xはコピーされる
    println!("xはまだ使える: {}", x);  // OK
    
    // 値を返す
    let s1 = gives_ownership();  // 関数から所有権を受け取る
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // s2を渡して、s3で受け取る
    println!("s1: {}, s3: {}", s1, s3);
}

fn takes_ownership(some_string: String) {
    println!("受け取った文字列: {}", some_string);
}  // ここでsome_stringがスコープを抜け、`drop`が呼ばれる

fn makes_copy(some_integer: i32) {
    println!("受け取った整数: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string  // 所有権を呼び出し元に移動
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string  // 受け取った値をそのまま返す
}

// 5. 実践的な例：構造体と所有権
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    // selfを取る：所有権を奪う
    fn consume(self) {
        println!("{}を読み終わりました", self.title);
    }
    
    // &selfを取る：借用
    fn display(&self) {
        println!("「{}」by {} ({}ページ)", self.title, self.author, self.pages);
    }
    
    // &mut selfを取る：可変借用
    fn add_pages(&mut self, additional: u32) {
        self.pages += additional;
        println!("ページ数を{}に更新しました", self.pages);
    }
}

pub fn struct_ownership_example() {
    let mut book = Book {
        title: String::from("プログラミングRust"),
        author: String::from("Jim Blandy"),
        pages: 600,
    };
    
    book.display();  // 借用なのでbookはまだ使える
    book.add_pages(50);  // 可変借用
    book.display();  // まだ使える
    
    book.consume();  // 所有権が移動
    // book.display();  // エラー！bookはもう使えない
}

// メインの実行関数
pub fn run_all_ownership() {
    println!("\n=== 所有権の基本 ===");
    ownership_basics();
    
    println!("\n=== 参照と借用 ===");
    references_and_borrowing();
    
    println!("\n=== スライス ===");
    slices();
    
    println!("\n=== 所有権と関数 ===");
    ownership_and_functions();
    
    println!("\n=== 構造体と所有権 ===");
    struct_ownership_example();
}