fn main() {
    // let x = 5; // mut ではないため再代入時(x = 6)で怒られる
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
        // const:
        // letと異なり、mut可変化もできず常に不変
        // 型注釈を常に要求
        // グローバルスコープにも定義可能
        // 定数式しかセットできない
        //
        // 命名規則はUPPER_SNAKE_CASE

    let y = 6;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y is: {}", y);
    }
        //  スコープを抜けるとスコープ内のシャドウイングは消える
        //  シャドウイングはletで再宣言する瞬間のみ変更を加えることができるだけで
        //  それ以降は不変になると言っていい
    println!("The value of y is: {}", y);

    let spaces = "    ";
    println!("The value of spaces is: \"{}\"", spaces);
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
        // シャドウイングは型の変更もできる
}
