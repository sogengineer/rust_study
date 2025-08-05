// コレクションの学習
// Rustの標準的なコレクション型について学びます

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

// 1. ベクタ（Vec<T>）
pub fn vectors() {
    // ベクタの作成
    let _v1: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3];  // vec!マクロ
    
    // 可変ベクタ
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    println!("ベクタ: {:?}", v);
    
    // 要素へのアクセス
    let third: &i32 = &v[2];
    println!("3番目の要素: {}", third);
    
    // getメソッド（安全なアクセス）
    match v.get(2) {
        Some(third) => println!("3番目の要素: {}", third),
        None => println!("3番目の要素はありません"),
    }
    
    // 範囲外アクセスの処理
    let does_not_exist = v.get(100);
    println!("100番目の要素: {:?}", does_not_exist);
    
    // イテレーション
    for i in &v {
        println!("値: {}", i);
    }
    
    // 可変イテレーション
    for i in &mut v {
        *i += 50;
    }
    println!("変更後: {:?}", v);
    
    // 異なる型を格納（列挙型を使用）
    #[derive(Debug)]
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
    
    println!("スプレッドシートの行: {:?}", row);
    
    // よく使うメソッド
    let mut v = vec![1, 2, 3, 4, 5];
    
    v.pop();  // 最後の要素を削除
    println!("pop後: {:?}", v);
    
    v.insert(2, 10);  // 指定位置に挿入
    println!("insert後: {:?}", v);
    
    v.remove(1);  // 指定位置の要素を削除
    println!("remove後: {:?}", v);
    
    v.retain(|&x| x % 2 == 0);  // 条件を満たす要素のみ保持
    println!("偶数のみ: {:?}", v);
    
    v.clear();  // 全要素を削除
    println!("clear後: {:?}", v);
}

// 2. 文字列（String）
pub fn strings() {
    // 文字列の作成
    let mut _s = String::new();
    
    let data = "初期内容";
    let _s = data.to_string();
    
    let _s = "初期内容".to_string();
    
    let _s = String::from("初期内容");
    
    // UTF-8でエンコードされた文字列
    let _hello = String::from("こんにちは");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");
    
    // 文字列の更新
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("push_str: {}", s);
    
    let mut s = String::from("lo");
    s.push('l');  // 1文字を追加
    println!("push: {}", s);
    
    // 文字列の結合
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1は移動される
    println!("結合: {}", s3);
    
    // format!マクロ
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("format!: {}", s);
    
    // 文字列のインデックス（注意が必要）
    let _s = String::from("こんにちは");
    // let h = s[0];  // エラー！文字列は直接インデックスできない
    
    // 文字列のスライス（注意して使用）
    let hello = "Здравствуйте";
    let s = &hello[0..4];  // 最初の4バイト
    println!("スライス: {}", s);
    
    // 文字列の反復処理
    for c in "नमस्ते".chars() {
        println!("文字: {}", c);
    }
    
    for b in "नमस्ते".bytes() {
        println!("バイト: {}", b);
    }
    
    // 文字列の分割
    let text = "apple,banana,orange";
    let fruits: Vec<&str> = text.split(',').collect();
    println!("フルーツ: {:?}", fruits);
    
    // 文字列の検索と置換
    let s = String::from("I like rust");
    println!("含む？ {}", s.contains("rust"));
    println!("置換: {}", s.replace("rust", "Rust"));
    
    // トリミング
    let s = String::from("  hello  ");
    println!("トリム前: '{}'", s);
    println!("トリム後: '{}'", s.trim());
}

// 3. ハッシュマップ（HashMap<K, V>）
pub fn hash_maps() {
    // ハッシュマップの作成
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    println!("スコア: {:?}", scores);
    
    // タプルのベクタから作成
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("zipで作成: {:?}", scores);
    
    // 値へのアクセス
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Blueのスコア: {:?}", score);
    
    // イテレーション
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // 値の更新
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    
    // 上書き
    scores.insert(String::from("Blue"), 25);
    println!("上書き後: {:?}", scores);
    
    // エントリーAPI
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("or_insert後: {:?}", scores);
    
    // 古い値に基づいて更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("単語カウント: {:?}", map);
    
    // よく使うメソッド
    let mut scores = HashMap::new();
    scores.insert("Alice", 100);
    scores.insert("Bob", 80);
    scores.insert("Charlie", 90);
    
    // キーの存在確認
    println!("Aliceは存在？ {}", scores.contains_key("Alice"));
    
    // 削除
    scores.remove("Bob");
    println!("削除後: {:?}", scores);
    
    // 条件付き削除
    scores.retain(|_, &mut v| v >= 90);
    println!("90点以上のみ: {:?}", scores);
}

// 4. その他のコレクション
pub fn other_collections() {
    // HashSet（重複なしの集合）
    let mut books = HashSet::new();
    books.insert("プログラミングRust");
    books.insert("実践Rust入門");
    books.insert("プログラミングRust");  // 重複は無視される
    
    println!("書籍セット: {:?}", books);
    
    // 集合演算
    let a: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = vec![2, 3, 4].into_iter().collect();
    
    let union: HashSet<_> = a.union(&b).collect();
    let intersection: HashSet<_> = a.intersection(&b).collect();
    let difference: HashSet<_> = a.difference(&b).collect();
    
    println!("和集合: {:?}", union);
    println!("積集合: {:?}", intersection);
    println!("差集合: {:?}", difference);
    
    // VecDeque（両端キュー）
    let mut deque = VecDeque::new();
    
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    
    println!("VecDeque: {:?}", deque);
    
    println!("前から取り出し: {:?}", deque.pop_front());
    println!("後ろから取り出し: {:?}", deque.pop_back());
    println!("残り: {:?}", deque);
}

// 5. イテレータとクロージャ
pub fn iterators_and_closures() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    
    for val in v1_iter {
        println!("値: {}", val);
    }
    
    // map
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("map: {:?}", v2);
    
    // filter
    let v1 = vec![1, 2, 3, 4, 5, 6];
    let evens: Vec<_> = v1.iter().filter(|&&x| x % 2 == 0).collect();
    println!("偶数: {:?}", evens);
    
    // fold（畳み込み）
    let sum: i32 = vec![1, 2, 3, 4, 5].iter().fold(0, |acc, x| acc + x);
    println!("合計: {}", sum);
    
    // クロージャ
    let x = 4;
    let equal_to_x = |z| z == x;  // xをキャプチャ
    let y = 4;
    println!("equal_to_x(y): {}", equal_to_x(y));
    
    // メソッドチェーン
    let results: Vec<_> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|x| x * x)
        .take(3)
        .collect();
    println!("最初の3つの偶数の二乗: {:?}", results);
}

// 6. 実践的な例：学生の成績管理システム
#[derive(Debug, Clone)]
struct Student {
    name: String,
    id: u32,
    grades: HashMap<String, f64>,
}

struct GradeBook {
    students: HashMap<u32, Student>,
    courses: HashSet<String>,
}

impl GradeBook {
    fn new() -> Self {
        GradeBook {
            students: HashMap::new(),
            courses: HashSet::new(),
        }
    }
    
    fn add_student(&mut self, name: String, id: u32) {
        let student = Student {
            name,
            id,
            grades: HashMap::new(),
        };
        self.students.insert(id, student);
    }
    
    fn add_course(&mut self, course: String) {
        self.courses.insert(course);
    }
    
    fn add_grade(&mut self, student_id: u32, course: String, grade: f64) {
        if let Some(student) = self.students.get_mut(&student_id) {
            if self.courses.contains(&course) {
                student.grades.insert(course, grade);
            }
        }
    }
    
    fn get_average_grade(&self, student_id: u32) -> Option<f64> {
        self.students.get(&student_id).map(|student| {
            if student.grades.is_empty() {
                0.0
            } else {
                let sum: f64 = student.grades.values().sum();
                sum / student.grades.len() as f64
            }
        })
    }
    
    fn get_top_students(&self, n: usize) -> Vec<(&str, f64)> {
        let mut student_avgs: Vec<_> = self.students
            .values()
            .filter_map(|student| {
                self.get_average_grade(student.id)
                    .map(|avg| (student.name.as_str(), avg))
            })
            .collect();
        
        student_avgs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        student_avgs.into_iter().take(n).collect()
    }
}

pub fn gradebook_example() {
    let mut gradebook = GradeBook::new();
    
    // コースを追加
    gradebook.add_course("数学".to_string());
    gradebook.add_course("物理".to_string());
    gradebook.add_course("化学".to_string());
    
    // 学生を追加
    gradebook.add_student("田中太郎".to_string(), 1001);
    gradebook.add_student("山田花子".to_string(), 1002);
    gradebook.add_student("鈴木一郎".to_string(), 1003);
    
    // 成績を追加
    gradebook.add_grade(1001, "数学".to_string(), 85.0);
    gradebook.add_grade(1001, "物理".to_string(), 90.0);
    gradebook.add_grade(1001, "化学".to_string(), 88.0);
    
    gradebook.add_grade(1002, "数学".to_string(), 95.0);
    gradebook.add_grade(1002, "物理".to_string(), 92.0);
    gradebook.add_grade(1002, "化学".to_string(), 94.0);
    
    gradebook.add_grade(1003, "数学".to_string(), 78.0);
    gradebook.add_grade(1003, "物理".to_string(), 82.0);
    gradebook.add_grade(1003, "化学".to_string(), 80.0);
    
    // 平均点を表示
    for (id, student) in &gradebook.students {
        if let Some(avg) = gradebook.get_average_grade(*id) {
            println!("{} の平均点: {:.2}", student.name, avg);
        }
    }
    
    // トップ学生を表示
    println!("\nトップ2の学生:");
    for (name, avg) in gradebook.get_top_students(2) {
        println!("  {} - 平均点: {:.2}", name, avg);
    }
}

// メインの実行関数
pub fn run_all_collections() {
    println!("\n=== ベクタ ===");
    vectors();
    
    println!("\n=== 文字列 ===");
    strings();
    
    println!("\n=== ハッシュマップ ===");
    hash_maps();
    
    println!("\n=== その他のコレクション ===");
    other_collections();
    
    println!("\n=== イテレータとクロージャ ===");
    iterators_and_closures();
    
    println!("\n=== 成績管理システムの例 ===");
    gradebook_example();
}