// テストの書き方の学習
// Rustのテスト機能について学びます

// 1. 基本的なテスト
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

// テストモジュール
#[cfg(test)]
mod basic_tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }
    
    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
        assert_eq!(subtract(0, 5), -5);
    }
    
    // assert!マクロ
    #[test]
    fn test_with_assert() {
        let result = add(2, 2);
        assert!(result == 4);
        assert!(result > 0);
    }
    
    // assert_ne!マクロ
    #[test]
    fn test_not_equal() {
        assert_ne!(add(2, 2), 5);
    }
}

// 2. カスタムメッセージ付きアサート
#[derive(Debug, PartialEq)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

#[cfg(test)]
mod rectangle_tests {
    use super::*;
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        
        assert!(
            larger.can_hold(&smaller),
            "大きい長方形{}x{}は小さい長方形{}x{}を含むべき",
            larger.width, larger.height,
            smaller.width, smaller.height
        );
    }
    
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        
        assert!(!smaller.can_hold(&larger));
    }
    
    #[test]
    fn squares_are_equal() {
        let square1 = Rectangle::square(5);
        let square2 = Rectangle::square(5);
        
        assert_eq!(square1, square2, "同じサイズの正方形は等しいべき");
    }
}

// 3. パニックのテスト
pub fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("ゼロによる除算はできません！");
    }
    a / b
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guessの値は1から100の間でなければなりません。入力値: {}", value);
        }
        
        Guess { value }
    }
    
    pub fn value(&self) -> i32 {
        self.value
    }
}

#[cfg(test)]
mod panic_tests {
    use super::*;
    
    #[test]
    #[should_panic]
    fn divide_by_zero_panics() {
        divide(10.0, 0.0);
    }
    
    #[test]
    #[should_panic(expected = "Guessの値は1から100の間でなければなりません")]
    fn greater_than_100() {
        Guess::new(200);
    }
    
    // パニックしないことを確認
    #[test]
    fn valid_guess() {
        let guess = Guess::new(50);
        assert_eq!(guess.value(), 50);
    }
}

// 4. Result<T, E>を使うテスト
pub fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

#[cfg(test)]
mod result_tests {
    use super::*;
    
    #[test]
    fn test_parse_valid_number() -> Result<(), String> {
        let result = parse_number("42")?;
        assert_eq!(result, 42);
        Ok(())
    }
    
    #[test]
    fn test_parse_invalid_number() {
        let result = parse_number("abc");
        assert!(result.is_err());
    }
    
    // Result型を返すテストで複数のアサーション
    #[test]
    fn multiple_assertions() -> Result<(), String> {
        let num1 = parse_number("10").map_err(|e| e.to_string())?;
        let num2 = parse_number("20").map_err(|e| e.to_string())?;
        
        assert_eq!(num1, 10);
        assert_eq!(num2, 20);
        assert_eq!(num1 + num2, 30);
        
        Ok(())
    }
}

// 5. テストの整理と実行制御
pub fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod organization_tests {
    use super::*;
    
    // 単体テスト
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
    
    // 無視されるテスト
    #[test]
    #[ignore]
    fn expensive_test() {
        // 実行に時間がかかるテスト
        // cargo test -- --ignored で実行可能
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert!(true);
    }
    
    // テストヘルパー関数（テストではない）
    fn common_setup() -> Vec<i32> {
        vec![1, 2, 3, 4, 5]
    }
    
    #[test]
    fn test_with_setup() {
        let data = common_setup();
        assert_eq!(data.len(), 5);
    }
}

// 6. 実践的な例：電卓のテスト
pub struct Calculator;

impl Calculator {
    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }
    
    pub fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }
    
    pub fn multiply(a: f64, b: f64) -> f64 {
        a * b
    }
    
    pub fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("ゼロによる除算".to_string())
        } else {
            Ok(a / b)
        }
    }
    
    pub fn power(base: f64, exponent: i32) -> f64 {
        base.powi(exponent)
    }
}

#[cfg(test)]
mod calculator_tests {
    use super::*;
    
    // 基本的な演算のテスト
    #[test]
    fn test_addition() {
        assert_eq!(Calculator::add(2.0, 3.0), 5.0);
        assert_eq!(Calculator::add(-1.0, 1.0), 0.0);
        assert_eq!(Calculator::add(0.0, 0.0), 0.0);
    }
    
    #[test]
    fn test_subtraction() {
        assert_eq!(Calculator::subtract(5.0, 3.0), 2.0);
        assert_eq!(Calculator::subtract(0.0, 5.0), -5.0);
    }
    
    #[test]
    fn test_multiplication() {
        assert_eq!(Calculator::multiply(3.0, 4.0), 12.0);
        assert_eq!(Calculator::multiply(-2.0, 3.0), -6.0);
        assert_eq!(Calculator::multiply(0.0, 100.0), 0.0);
    }
    
    #[test]
    fn test_division() {
        assert_eq!(Calculator::divide(10.0, 2.0), Ok(5.0));
        assert_eq!(Calculator::divide(7.0, 2.0), Ok(3.5));
        assert!(Calculator::divide(5.0, 0.0).is_err());
    }
    
    #[test]
    fn test_power() {
        assert_eq!(Calculator::power(2.0, 3), 8.0);
        assert_eq!(Calculator::power(5.0, 0), 1.0);
        assert_eq!(Calculator::power(10.0, -1), 0.1);
    }
    
    // 浮動小数点の比較（許容誤差を考慮）
    #[test]
    fn test_floating_point_precision() {
        let result = Calculator::add(0.1, 0.2);
        let expected = 0.3;
        let tolerance = 1e-10;
        
        assert!((result - expected).abs() < tolerance);
    }
    
    // 複数の操作を組み合わせたテスト
    #[test]
    fn test_complex_calculation() {
        // (2 + 3) * 4 / 2 = 10
        let sum = Calculator::add(2.0, 3.0);
        let product = Calculator::multiply(sum, 4.0);
        let result = Calculator::divide(product, 2.0);
        
        assert_eq!(result, Ok(10.0));
    }
}

// 7. プロパティベーステスト（概念的な例）
#[cfg(test)]
mod property_tests {
    use super::*;
    
    // 加法の交換法則
    #[test]
    fn addition_is_commutative() {
        let test_cases = vec![
            (1.0, 2.0),
            (3.5, 4.7),
            (-1.0, 5.0),
            (0.0, 0.0),
        ];
        
        for (a, b) in test_cases {
            assert_eq!(
                Calculator::add(a, b),
                Calculator::add(b, a),
                "加法は交換法則を満たすべき: {} + {} = {} + {}",
                a, b, b, a
            );
        }
    }
    
    // 加法の結合法則
    #[test]
    fn addition_is_associative() {
        let test_cases = vec![
            (1.0, 2.0, 3.0),
            (0.5, 1.5, 2.5),
            (-1.0, -2.0, -3.0),
        ];
        
        for (a, b, c) in test_cases {
            let result1 = Calculator::add(Calculator::add(a, b), c);
            let result2 = Calculator::add(a, Calculator::add(b, c));
            
            assert!((result1 - result2).abs() < 1e-10);
        }
    }
}

// 統合テストの例（通常は tests/ ディレクトリに配置）
pub fn integration_example() {
    println!("これは統合テストで使用される公開関数です");
}

// テスト実行のデモ関数
pub fn run_testing_demo() {
    println!("テストは以下のコマンドで実行できます:");
    println!("  cargo test                    # 全てのテストを実行");
    println!("  cargo test test_add           # 特定のテストを実行");
    println!("  cargo test basic              # 名前に'basic'を含むテストを実行");
    println!("  cargo test -- --nocapture     # printlnの出力を表示");
    println!("  cargo test -- --test-threads=1 # シングルスレッドで実行");
    println!("  cargo test -- --ignored       # 無視されたテストのみ実行");
    
    println!("\nテストの実行例:");
    println!("加算のテスト: 2 + 3 = {}", add(2, 3));
    println!("減算のテスト: 5 - 3 = {}", subtract(5, 3));
    
    let rect1 = Rectangle { width: 10, height: 20 };
    let rect2 = Rectangle { width: 5, height: 10 };
    println!("長方形の包含テスト: rect1はrect2を含む？ {}", rect1.can_hold(&rect2));
}