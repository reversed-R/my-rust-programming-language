use std::io;

fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // タプルの要素の型は同じである必要はない

    let (x, y, z) = tup;
    // 分配

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("The value of z is {}", z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    // 添字でタプルの要素にアクセス

    println!("The value of five_hundred is {}", five_hundred);
    println!("The value of six_point_four is {}", six_point_four);
    println!("The value of one is {}", one);

    let arr = [1, 2, 3, 4, 5];
    // スタックに固定長で確保される
    println!("The value of arr[0] is {}", arr[0]);

    let arr2: [i32; 5] = [1, 2, 3, 4, 5];
    // [type; length]で要素の型と長さを定義できる
    println!("The value of arr2[3] is {}", arr2[3]);

    let arr3 = [3; 5];
    // [3, 3, 3, 3, 3]に等しい初期化
    println!("The value of arr3[3] is {}", arr3[3]);

    let mut index = String::new();

    println!("Input index number of array");

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];
    // もしインデックスが配列から溢れるならパニック(ランタイムエラー)になる

    println!("The value of arr[{}] is {}", index, element);
}
