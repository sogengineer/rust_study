// ジェネリクスとトレイトの学習
// Rustの型システムの強力な機能について学びます

use std::fmt::{Debug, Display};

// 1. ジェネリック関数
pub fn generics_functions() {
    // 非ジェネリックな関数（重複したコード）
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
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    
    // ジェネリックな関数
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("最大の数値: {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大の文字: {}", result);
}

// 2. ジェネリック構造体
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// 特定の型に対する実装
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> MixedPoint<T, U> {
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn generics_structs() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    let mixed = MixedPoint { x: 5, y: 10.5 };
    
    println!("整数の点: {:?}", integer_point);
    println!("浮動小数点の点: {:?}", float_point);
    println!("混合型の点: {:?}", mixed);
    
    println!("integer_point.x = {}", integer_point.x());
    println!("原点からの距離: {}", float_point.distance_from_origin());
    
    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("混ぜた結果: {:?}", p3);
}

// 3. ジェネリック列挙型
#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// カスタム列挙型
#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

impl<T> BinaryTree<T> {
    fn new() -> Self {
        BinaryTree::Empty
    }
    
    fn leaf(value: T) -> Self {
        BinaryTree::Node {
            value,
            left: Box::new(BinaryTree::Empty),
            right: Box::new(BinaryTree::Empty),
        }
    }
}

pub fn generics_enums() {
    let some_number = Option::Some(5);
    let some_string = Option::Some("a string");
    let absent_number: Option<i32> = Option::None;
    
    println!("Option列挙型: {:?}, {:?}, {:?}", 
             some_number, some_string, absent_number);
    
    let success: Result<i32, String> = Result::Ok(42);
    let failure: Result<i32, String> = Result::Err(String::from("エラーです"));
    
    println!("Result列挙型: {:?}, {:?}", success, failure);
    
    // 二分木の例
    let tree = BinaryTree::Node {
        value: 5,
        left: Box::new(BinaryTree::leaf(3)),
        right: Box::new(BinaryTree::leaf(7)),
    };
    println!("二分木: {:?}", tree);
}

// 4. トレイトの定義と実装
pub trait Summary {
    fn summarize(&self) -> String;
    
    // デフォルト実装
    fn summarize_author(&self) -> String {
        String::from("(著者不明)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} - {} ({})", self.headline, self.author, self.location)
    }
    
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
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

pub fn traits_basics() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("もちろん、ご存知のように、みなさん"),
        reply: false,
        retweet: false,
    };
    
    let article = NewsArticle {
        headline: String::from("ペンギンが勝利"),
        location: String::from("ピッツバーグ、ペンシルベニア"),
        author: String::from("アイスバーグ"),
        content: String::from("ピッツバーグ・ペンギンズが再びNHLチャンピオンに。"),
    };
    
    println!("新しいツイート: {}", tweet.summarize());
    println!("新しい記事: {}", article.summarize());
    println!("記事の著者: {}", article.summarize_author());
}

// 5. トレイト境界
pub fn notify<T: Summary>(item: &T) {
    println!("速報！ {}", item.summarize());
}

// 複数のトレイト境界
pub fn notify_multiple<T: Summary + Display>(item: &T) {
    println!("速報！ {}", item.summarize());
    println!("表示: {}", item);
}

// where句を使った見やすい書き方
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("t: {}", t);
    println!("u: {:?}", u);
    42
}

// トレイト境界を返り値に使用
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("もちろん、ご存知のように、みなさん"),
        reply: false,
        retweet: false,
    }
}

pub fn trait_bounds_example() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("最新ニュース"),
        reply: false,
        retweet: false,
    };
    
    notify(&tweet);
    
    let item = returns_summarizable();
    println!("返されたアイテム: {}", item.summarize());
}

// 6. ライフタイムとジェネリクス
pub fn lifetimes_example() {
    // ライフタイム注釈が必要な関数
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = String::from("長い文字列です");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("最も長い文字列: {}", result);
    
    // ライフタイムを持つ構造体
    #[derive(Debug)]
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
    
    let novel = String::from("むかしむかし、あるところに...");
    let first_sentence = novel.split('、').next().expect("文が見つかりません");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("重要な引用: {:?}", i);
    println!("レベル: {}", i.level());
}

// 7. 高度なトレイト
// 関連型
pub trait Iterator {
    type Item;
    
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Option::Some(self.count)
        } else {
            Option::None
        }
    }
}

// デフォルト型パラメータ
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point2D {
    x: f64,
    y: f64,
}

impl Add for Point2D {
    type Output = Point2D;
    
    fn add(self, other: Point2D) -> Point2D {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

pub fn advanced_traits() {
    // カスタムイテレータ
    let mut counter = Counter::new();
    
    println!("カウンター:");
    while let Option::Some(value) = counter.next() {
        println!("  {}", value);
    }
    
    // 演算子オーバーロード
    let p1 = Point2D { x: 1.0, y: 2.0 };
    let p2 = Point2D { x: 3.0, y: 4.0 };
    let p3 = p1 + p2;
    
    println!("{:?} + {:?} = {:?}", p1, p2, p3);
}

// 8. 実践的な例：ジェネリックなキャッシュ
use std::collections::HashMap;
use std::hash::Hash;

struct Cache<T, U>
where
    T: Fn(&U) -> U,
    U: Clone + Eq + Hash,
{
    calculation: T,
    values: HashMap<U, U>,
}

impl<T, U> Cache<T, U>
where
    T: Fn(&U) -> U,
    U: Clone + Eq + Hash,
{
    fn new(calculation: T) -> Cache<T, U> {
        Cache {
            calculation,
            values: HashMap::new(),
        }
    }
    
    fn value(&mut self, arg: U) -> U {
        match self.values.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(&arg);
                self.values.insert(arg, v.clone());
                v
            }
        }
    }
}

pub fn cache_example() {
    let mut expensive_cache = Cache::new(|num: &u32| {
        println!("高コストな計算中...");
        std::thread::sleep(std::time::Duration::from_millis(100));
        num * 2
    });
    
    println!("最初の呼び出し: {}", expensive_cache.value(5));
    println!("二回目の呼び出し（キャッシュ済み）: {}", expensive_cache.value(5));
    println!("別の値: {}", expensive_cache.value(10));
}

// メインの実行関数
pub fn run_all_generics_traits() {
    println!("\n=== ジェネリック関数 ===");
    generics_functions();
    
    println!("\n=== ジェネリック構造体 ===");
    generics_structs();
    
    println!("\n=== ジェネリック列挙型 ===");
    generics_enums();
    
    println!("\n=== トレイトの基本 ===");
    traits_basics();
    
    println!("\n=== トレイト境界 ===");
    trait_bounds_example();
    
    println!("\n=== ライフタイム ===");
    lifetimes_example();
    
    println!("\n=== 高度なトレイト ===");
    advanced_traits();
    
    println!("\n=== キャッシュの例 ===");
    cache_example();
}