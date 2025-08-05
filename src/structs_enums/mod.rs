// 構造体と列挙型の学習
// Rustのカスタムデータ型について学びます

// 1. 構造体の定義と使用
#[derive(Debug)]  // Debug出力を可能にする
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// タプル構造体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// ユニット様構造体（フィールドなし）
struct AlwaysEqual;

pub fn structs_basics() {
    // 構造体のインスタンス化
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("ユーザー名: {}", user1.username);
    
    // 可変な構造体
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,  // フィールドの値をコピー
        sign_in_count: user1.sign_in_count,
    };
    
    user2.email = String::from("anotheremail@example.com");
    
    // 構造体更新記法
    let _user3 = User {
        email: String::from("another@example.com"),
        ..user1  // 残りのフィールドはuser1から取得
    };
    
    // タプル構造体
    let black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    
    println!("黒色: ({}, {}, {})", black.0, black.1, black.2);
    
    // Debug表示
    println!("{:?}", user2);  // 1行で表示
    println!("{:#?}", user2); // 見やすく整形して表示
}

// ビルダー関数
fn build_user(email: String, username: String) -> User {
    User {
        email,      // フィールド初期化省略記法
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 2. 構造体とメソッド
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // メソッド（&selfを取る）
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 関連関数（selfを取らない）
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // 可変メソッド
    fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}

// 複数のimplブロックも可能
impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

pub fn methods_example() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("長方形の面積: {}", rect1.area());
    println!("長方形の周囲: {}", rect1.perimeter());
    
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    println!("rect1はrect2を含む？ {}", rect1.can_hold(&rect2));
    println!("rect1はrect3を含む？ {}", rect1.can_hold(&rect3));
    
    // 関連関数の呼び出し
    let square = Rectangle::square(20);
    println!("正方形: {:?}", square);
    
    // 可変メソッド
    let mut rect4 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("変更前: {:?}", rect4);
    rect4.double_size();
    println!("変更後: {:?}", rect4);
}

// 3. 列挙型（Enum）
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// データを持つ列挙型
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// より複雑な列挙型
#[derive(Debug)]
enum Message {
    Quit,                       // データなし
    Move { x: i32, y: i32 },    // 名前付きフィールド
    Write(String),              // Stringを持つ
    ChangeColor(i32, i32, i32), // 3つのi32を持つ
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("終了メッセージ"),
            Message::Move { x, y } => println!("({}, {})へ移動", x, y),
            Message::Write(text) => println!("メッセージ: {}", text),
            Message::ChangeColor(r, g, b) => println!("色変更: RGB({}, {}, {})", r, g, b),
        }
    }
}

pub fn enums_basics() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    println!("IPアドレスの種類: {:?}, {:?}", four, six);
    
    // データを持つ列挙型
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("ホーム: {:?}", home);
    println!("ループバック: {:?}", loopback);
    
    // メッセージの使用
    let m1 = Message::Write(String::from("hello"));
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::ChangeColor(255, 0, 0);
    let m4 = Message::Quit;
    
    m1.call();
    m2.call();
    m3.call();
    m4.call();
}

// 4. Option<T>型
pub fn option_example() {
    // Option<T>は標準ライブラリで定義されている
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    
    let some_number = Some(5);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = None;
    
    // Optionの使用
    let _x: i8 = 5;
    let y: Option<i8> = Some(5);
    
    // let sum = x + y;  // エラー！Option<i8>とi8は足せない
    
    // Optionから値を取り出す
    match y {
        Some(value) => println!("yの値: {}", value),
        None => println!("yは値を持たない"),
    }
    
    // if letを使った簡潔な書き方
    if let Some(value) = some_number {
        println!("値がある: {}", value);
    }
}

// 5. パターンマッチング
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),  // 州の名前を持つ
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {}", state);
            25
        }
    }
}

pub fn pattern_matching() {
    let coin1 = Coin::Quarter(String::from("Alaska"));
    let coin2 = Coin::Penny;
    
    println!("コイン1の価値: {} cents", value_in_cents(coin1));
    println!("コイン2の価値: {} cents", value_in_cents(coin2));
    
    // Option<T>とのマッチング
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("5 + 1 = {:?}", six);
    println!("None + 1 = {:?}", none);
    
    // _プレースホルダー
    let some_value = 0u8;
    match some_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),  // その他のケースは何もしない
    }
}

// 6. 実践的な例：ゲームの状態管理
#[derive(Debug, PartialEq)]
enum GameState {
    Menu,
    Playing { score: u32, level: u8 },
    Paused,
    GameOver { final_score: u32 },
}

struct Game {
    state: GameState,
    player_name: String,
}

impl Game {
    fn new(name: String) -> Self {
        Game {
            state: GameState::Menu,
            player_name: name,
        }
    }
    
    fn start(&mut self) {
        self.state = GameState::Playing { score: 0, level: 1 };
        println!("ゲーム開始！");
    }
    
    fn pause(&mut self) {
        if let GameState::Playing { .. } = self.state {
            self.state = GameState::Paused;
            println!("ゲーム一時停止");
        }
    }
    
    fn update_score(&mut self, points: u32) {
        if let GameState::Playing { ref mut score, .. } = self.state {
            *score += points;
            println!("スコア更新: {}", score);
        }
    }
    
    fn game_over(&mut self) {
        if let GameState::Playing { score, .. } = self.state {
            self.state = GameState::GameOver { final_score: score };
            println!("ゲームオーバー！最終スコア: {}", score);
        }
    }
}

pub fn game_example() {
    let mut game = Game::new(String::from("プレイヤー1"));
    
    game.start();
    game.update_score(100);
    game.update_score(50);
    game.pause();
    game.game_over();
    
    println!("最終状態: {:?}", game.state);
}

// メインの実行関数
pub fn run_all_structs_enums() {
    println!("\n=== 構造体の基本 ===");
    structs_basics();
    
    println!("\n=== メソッドの例 ===");
    methods_example();
    
    println!("\n=== 列挙型の基本 ===");
    enums_basics();
    
    println!("\n=== Option型 ===");
    option_example();
    
    println!("\n=== パターンマッチング ===");
    pattern_matching();
    
    println!("\n=== ゲームの例 ===");
    game_example();
}