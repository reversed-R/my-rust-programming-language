use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess) // 参照&も変数と同様にデフォルトでは不変なので、&mutとして可変の参照として渡す
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            // shadowing:
            // すでにある変数名を、特に別の型で覆い隠すこと、が許されている
            // String.trim():
            // 文字列の先頭と末尾の空白を全て削除する、例えばio::stdin().read_line()は改行を含む文字列を返す
            // String.parse():
            // 代入（束縛？）先の型から変換先の型を推定してくれる？
            // matchは式、num: u32を返せる
            // _はワイルドカード
        
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) { // ここではsecret_numberに対して変更を加える必要がないので&mutではなくただの&として渡している
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too greater!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
