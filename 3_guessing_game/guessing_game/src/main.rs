use std::io; // io函式庫來自為標準函式庫(std), 提供處理input/output的函式庫
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("請猜一個數字!");

    // 產生1~100的數字, 也可以寫成1..=100
    let secrect_number = rand::thread_rng().gen_range(1..101);

    println!("祕密數字為: {}",secrect_number);

    loop {
        println!("請輸入你猜測的數字");

        let mut guess = String::new();
        //guess.push_str("abc");

        io::stdin()
            .read_line(&mut guess) // 輸入結果會append到guess變數中, 不是覆蓋
            .expect("讀取該行失敗");

        // string的parse方法可以將字串解析成各種數字型別
        let guess: i32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("你輸入的數字: {}", guess);

        match guess.cmp(&secrect_number) {
            Ordering::Less => println!("太小"),
            Ordering::Greater => println!("太大"),
            Ordering::Equal => {
                println!("You got it");
                break;
            }
        }
    }
}
