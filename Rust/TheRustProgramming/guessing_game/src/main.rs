extern crate rand;

// 標準ライブラリstdのioライブラリをスコープに導入
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1, 101);

	println!("The secret number is: {}", secret_number);

	loop {
		println!("Please input your guess.");
		// mut は可変という意味
		// ::new はnewがStringの関連関数という意味
		let mut guess = String::new();

		// もしuse std::ioをスコープしていなかった場合、std::io::stdinと書く
		io::stdin().read_line(&mut guess)
		           .expect("Failed to read line");

		// Rustは定義しない場合i32(32ビットの数字）の型となる。secret_numberはこれ。
		// このままだとguess（String)とcesret_number(i329は比較できないためparseする
		// Rustのシャドーイングの機能により、新しくguessは作られず再利用される
		// trimをしているのは、ユーザー入力決定時エンターが押されるので「5\n」となっている「\n」を除去するため
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num, // numの値を返している
			Err(_) => continue, // _は包括。Okではないすべてにマッチしたいといういみ
		};

		println!("You guessed: {}", guess);

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}