fn main() {
    println!("Hello, world!");

    another_function();

    print_integer(five());

    // let x = (let y = 6); // 代入は文（値を返さない）
    //
    // 関数呼び出しも、マクロ呼び出しも、ブロック{}も式

    // ブロック{}が評価されて4になる
    let y = {
        let x = 3;
        x + 1
    };
    // セミコロンをつけると式が文になる

    print_labeled_measurement(y, 'h');

    print_integer(plus_one(9));
}

// lower_snake_case で変数と関数は命名
// 関数の定義された位置は関係ない
fn another_function() {
    println!("Another function.");
}

fn print_integer(x: i32) {
    // 仮引数の型は必ず宣言
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {}{}", value, unit_label);
}
// 戻り値なしの場合、空のタプル()で表現される

fn five() -> i32 {
    // 戻り値の型を->のあとに書く
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
