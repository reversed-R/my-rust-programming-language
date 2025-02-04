fn main() {
    let s1 = String::from("hello");

    // &をつけることでs1の参照を渡している
    // ただしこの参照は不変参照である
    // （呼び出し元に変更が加わることがない。const char **に近い）
    // この参照を渡すことを借用といい、
    // 所有権がムーブしないので、
    // もとの変数は有効のままであり、
    // ただの不変参照(&)で渡している限り、
    // 渡した先で変更が加わることもない
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 可変な変数について
    let mut s2 = String::from("hello");

    // 不変でない参照を変更しようとするためエラー
    // change(&s1);

    // 可変参照なのでよし
    change(&mut s2);

    // 同じスコープでの複数の可変参照はできない
    // データ競合の発生を防ぐため
    // let r1 = &mut s2;
    // let r2 = &mut s2;
    // println!("{}, {}", r1, r2);

    // これなら同じスコープでないのでヨシ
    {
        let r1 = &mut s2;
    }
    let r2 = &mut s2;

    // 不変参照のみのときいくつも借用して良い
    // 変更されないので
    let r1 = &s2;
    let r2 = &s2;
    // しかし、不変参照中に可変参照はできない
    // 不変だと思っているのに変わったら困るため
    // let r3 = &mut s2;
    // println!("{}, {}, {}", r1, r2, r3);

    // つまりできることは
    //
    // 1つの可変参照
    // 複数の不変参照
    //
    // のどちらか
}

// 参照を受け取る側の型は&String
// cのようにchar ** とならないので注意
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 借り物を良いと言われてないのに破壊してはいけないように、
// 不変借用した変数を変更することはできない
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// 次のように可変な参照を受け取れば良い
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// sがスコープを抜けてドロップされるため
// その借用は返せない
// エラー
// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }

// Stringをそのまま返すことで所有権をムーブすれば良い
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
