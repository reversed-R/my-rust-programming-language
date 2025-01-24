fn main() {
    println!("Hello, world!");

    let number = 3;

    if number % 4 == 0 {
        // if bool型の条件式 {ブロック}
        // 条件式はboolでなければならない
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // else if が使える
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 4 or 3");
    }

    let number = if number % 2 == 0 { 6 } else { 5 };
    // if は式なので値を返す。let文の右辺における。
    // if の{ブロック} も else{ブロック} も同じ型にならなければならない。違うとコンパイルエラー
    // 例えばこういうのはだめ
    // let number = if condition { 5 } else { "six" };

    println!("The value of number is {}", number);

    let mut count = 0;
    'counting_up: loop {
        // loopに対して'label: をつけることができる
        // loop, while, for は式ではない
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", arr[index]);
        index += 1;
    }

    // とするよりも、次のようにforを使うほうが、
    // ミスが少なく、毎回条件を確認する命令が含まれない分速い

    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        // (begin..end) Range型
        println!("{}!", number);
    }
}
