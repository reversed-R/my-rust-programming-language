fn main() {
    let x = 5;
    let y = x;

    println!("x = {}", x); // i32などのプリミティブな型(スタックにおける固定長の型)は別の変数に代入しても値がコピーされるだけで
                           // 所有権の移動も発生せずにエラーにならない

    // しかしヒープに確保されるような型では
    let s1 = String::from("hello");
    let s2 = s1; // s1からs2への所有権のムーブが起こる

    // println!("{}, world!", s1); // 故にすでにs1は有効ではなく有効でないs1をprintln()を呼び出すために借用することはできない
    // とコンパイルエラー

    let s3 = s2.clone(); // こうすればs2が無効にならずにヒープ内でコピーができるので
    println!("{}, world!", s2); // s1をまた借用できる
                                // しかしヒープに確保される複合型のコピーをいちいちやるのは無駄

    // 代入と関数呼び出しはともに所有権がムーブする

    takes_ownership(s2); // s2の所有権がムーブする

    makes_copy(x); // xはコピーされるだけ

    let s4 = gives_ownership(); // 逆に関数内で定義された複合型の所有権がムーブされてくることもある
                                // もう二度とmalloc書かなくていい

    // でも逆に関数呼び出しに使った変数をまた使いたい時
    // 次のように関数の戻り値に引数自体を書かなければならないのだろうか

    let (s5, len) = calculate_length(s4);

    println!("The length of '{}' is {}.", s5, len);

    // 流石にこれではコンピュータとしての時間空間効率も、人間の操作の効率も悪い
    // ために、参照という概念がある
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
