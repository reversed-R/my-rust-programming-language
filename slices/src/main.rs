fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();
    // 元の文字列sがclearされて変更された
    // 故にwordというインデックスは意味を失った

    // スライスという、連続した値のうちの部分的な連続を参照する
    // 文字列スライス
    // [starting_index..ending_index]
    // ending_indexは終端のインデックスより1大きい値
    // つまり、長さは、ending_index - starting_index になる

    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let hello = &s[..5]; // 0は省略可
    let world = &s[6..11];
    let world = &s[6..s.len()]; // 終端についても
    let world = &s[6..]; // 省略可
    let hello_world = &[..]; // これで文字列全体の参照スライス

    // UTF-8文字は文字境界に合わせて範囲インデックスを置かなければならない
    // が簡単のため今はASCII文字で

    let mut s2 = String::from("hello world");

    let word2 = first_word2(&s2);

    // s2.clear();
    // clearという可変な参照を必要とする関数
    // first_word2は不変な参照のみを必要とする
    // 不変な参照をしているとき可変な参照は行えない
    // よってエラー
    // インデックスの意味が失われるバグはなくなった

    println!("the first word is: {}", word2);

    let my_string = String::from("goodbye world");

    let word3 = first_word3(&my_string);

    let my_string_literal = "hello world";

    // 文字列リテラルはそれ自身文字列スライスなので
    // 特別な記法なしに機能する
    let word4 = first_word3(my_string_literal);

    println!("the first word is: {}", word3);
    println!("the first word is: {}", word4);

    // 文字列ではないスライス
    // &[i32]型となる
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // タプル型の展開ができる
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
// しかしこの関数はもともとの文字列が変わっていない場合にのみ有効な
// インデックスしかかえせない
// もとの文字列が変更された時、インデックスは意味のない値になってしまう
//
// さらに次の単語区切りを返そうと思うと、
// 開始と終端をのインデックスを返す必要が出てくる
// 煩雑

// 文字列スライス&str でfirst_wordを書き換え
fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
// second_wordも同じ引数戻り値の型で実現できる

// 文字列スライス&str でfirst_wordを書き換え
// さらに引数も文字列スライスに
fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
