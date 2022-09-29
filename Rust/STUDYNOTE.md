## はじめに
こちらのドキュメントより学んだことのメモです。

https://doc.rust-jp.rs/book-ja/title-page.html

## パッケージ
クレートをビルドし、テストし、共有することができるCargoの機能

- ライブラリクレートは0個か1個を持つ
- バイナリクレートはいくらでも持って良い
- 少なくとも（ライブラリでもバイナリでも）1つのクレートを持っていないといけない
- Cargo.tomlという、それらのクレートをどのようにビルドするかを説明するファイルを持つ

```bash:
$ cargo new my-project // "src/main.rc"が作成
$ cargo new --libsmy-project // "src/lib.rc"が作成
```

## クレート
ライブラリか実行可能ファイルを生成する、木構造をしたモジュール群。
慣習として、以下となっており、Cargo.tomlには明記されていない。
`src/main.rs`がバイナリクレートのクレートルート
`src/lib.rs`がライブラリクレートのクレートルート

## モジュール
宣言はmod、命名規則は、snake_caseです。
pubをつけることで公開、つけないことで非公開。

```rust:src/lib.rc
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
    }
}
```

```rust:モジュールツリー
crate // crateモジュール（src/lib.rsから形成）
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         └── take_order
```

```rust:呼び出し方（servingから呼ぶと仮定）
// 絶対パス
crate::front_of_house::hosting::add_to_waitlist();
// 相対パス（絶対パスのほうが好ましい）
hosting::add_to_waitlist();

// super（呼び出し元の1つ上の階層からの相対パス）
super::hosting::add_to_waitlist();
// self（呼び出し元自身からの相対パス）
self::take_order();
```
##### モジュールを複数ファイルに分割する
mod {モジュール名}の後にブロックではなくセミコロンを使うと、Rustにモジュールの中身をモジュールと同じ名前をした別のファイルから読み込むように命令します。 

```rust:src/lib.rs
mod front_of_house; // モジュールを宣言（中身は別ファイル）

pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```
```rust:src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```
```rust:src/front_of_house.rs
pub mod hosting {
    pub fn add_to_waitlist() {}
}
// もしくは、別途"src/front_of_house/hosting.rs"を作成し
pub mod hosting;
```
```rust:src/front_of_house/hosting.rs
#![allow(unused)]
fn main() {
   pub fn add_to_waitlist() {}
}
```

## 関数（メソッドとは異なりますので注意）
宣言はfn、関数名の命名規則は、snake_caseです。

```Rust:main.rs
fn main() {
  println!("Hello, world!");
  let sum = another_function(5, 6); // 11
}

fn another_function(x: i32, y: i32) -> i32 {
  x + y  // 返したい値。;をつけると文になり値が返らないので付けない（詳細は後述）
}
```

##### Rustは式指向言語であるということ
- 式とは、結果値に評価されます。
- 文とは、なんらかの動作をして値を返さない命令です。

```Rust
let x = (let y = 6) //error: expected expression, found statement (`let`)
```
この`let y = 6`という文は値を返さないので、xに束縛するものがなくエラーとなる。なお、CやRubyでは、代入は代入値を返します。これらの言語では、 `x = y = 6`と書いて、xもyも値6になります。

では式とは、`let y = 6`という文の6は値6に評価される式です。関数呼び出しも式です。マクロ呼び出しも式です。 新しいスコープを作る際に使用するブロック`{}`も式です。

```Rust
fn main() {
  let x = 1;    // x = 1
  let y = {     // y = 3
    let x = 2;
    x + 1       // ;をつけると文になるので注意
  };
  
  let z = if y < 5 { // z = 4
    4
  } else {
    5
  };
```

## 変数
Rustは変数の不変性を推奨しており、標準で不変です。
ただし、mutキーワードを付与することで可変にする手段も残されている。

```Rust:不変な変数と可変な変数
let x = 5; // 不変な変数（xが5に束縛されている状態）
let mut y = 5; // 可変な変数

x = 6; // コンパイルエラー
y = 6; // y = 6

let x = x + 1; // x=6となる。これはシャドーイング（xを再利用）
```
##### 不変な変数のメリット

- 不具合予防
- mutにより、どこか別の箇所で値が変更されることを意識させることができる

##### 可変な変数とのトレードオフ

- 大きなデータ構造を使う場合などです。 インスタンスを可変にして変更できるようにする方が、いちいちインスタンスをコピーして新しくメモリ割り当てされたインスタンスを返すよりも速くなります。 

##### 変数と定数(constants)の違い

- 定数はグローバルスコープ含め、どんなスコープでも定義できます。なので、 いろんなところで使用される可能性のある値を定義するのに役に立ちます。また、管理者にとってハードコードされた値を変更する時は一箇所を変えるだけで良くなります。

- 定数は定数式にしかセットできないため、関数呼び出し結果や、実行時に評価される値にはセットできません。

```Rust:定数式
const MAX_POINTS: u32 = 100_000;
```

## 型
Rustは、静的型付言語です。コンパイル時に全ての型が決まっている必要があります。

```Rust
// 整数
let x = 2 // 基準のi32型
let x: u32 = "42".parse().expect("Not a number!"); //u32型
let x = "42".parse().expect("Not a number!"); //error[E0282]: type annotations needed

// 浮動小数点
let y = 2.0 // 基準のf64型
let y: f32 = 2.0; // f32型
let y: f32 = 3;   // error[E0308]: mismatched types

// 論理値
let t = true;        // bool型
let f: bool = false; // bool型

// 文字列
let s = "Z";  // str型
let s = 'Z';  // char型
let s = 'ZZ'; // エラー
let s = String::from("Z"); // String型

```
Rustの基準型とは、例えばf64はf32とほぼ同スピードにもかかわらず、より精度が高くなるなど、一般的にいい選択肢になります。

```Rust:複合型
// タプル（複数の型の何らかの値を一つの複合型にまとめ上げる一般的な手段）
let tup: (i32, f64, u8) = (500, 6.4, 1); // 500,6.4,1
let (x, y, z) = tup;                     // 500,6.4,1
let i = tup.0                            // 500

// 配列（配列の全要素は、 同じ型でなければなりません）
let x = [1, 2, 3, 4, 5];
let y = ["1", "2", "3", "4", "5"];
let i = 0;
let first = x[i]; // 0
```
## ジェネリックなデータ型
型名宣言を山カッコ(<>)内に記述。Tが慣習。

```rust
// ジェネリックな構造体（xとyは同じ型である必要がある）
struct Point<T> {
    x: T,
    y: T,
}
let integer1 = Point { x: 5, y: 10 };
let integer2 = Point { x: 5, y: 1.0 }; // error

// 以下ならxとyは異なる型での指定が可能
struct Point<T, U> {
    x: T,
    y: U,
}

// ジェネリックな関数
fn largest<T>(list: &[T]) -> T {
}

// Point<T>構造体にxというメソッドを実装
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
let p = Point { x: 5, y: 10 };
println!("p.x = {}", p.x());

// Point<f32>だけにメソッドを実装
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// 構造体のジェネリックな引数データ型とメソッドの引数データ型は必ずしも一致しない
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
let p1 = Point { x: 5, y: 10.4 };
let p2 = Point { x: "Hello", y: 'c'};
let p3 = p1.mixup(p2);
println!("{}, {}", p3.x, p3.y);　 // 5, c
```

## 新しい型を定義する構造体
構造体を定義するには、structキーワードを付与し構造体全体に名前を付けます。
{}の中はフィールドと呼び、異なる型を定義できます。

```rust
// 定義
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
構造体を使用するには、具体的な値を指定してインスタンスを生成します。

```rust:
// kye:value形式なので順番通りである必要はない
let email = String::from("someone@example.com");
let user1 = User {
    email, // フィールドと変数名が同名の場合省略可
    username: String::from("someusername123"),
    active: true, // 順番は問わない
    sign_in_count: 1,
};
// 明示的にセットされていない残りのフィールドが与えられたインスタンスと同じ値になる
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
// ドット記法で構造体から特定の値を得て、変更（可変であれば）
user1.email = String::from("anotheremail@example.com");
```
##### 異なる型を生成する名前付きフィールドのないタプル構造体

```rust
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```

## 構造体のプログラム例とメソッド
Rustにはクラスが存在しませんが概念としては構造体がクラス変数、メソッドがクラスメソッドとすると捉えやすい。
selfがないものは関連関数。構造体の新規インスタンスを返すコンストラクタによく使用されます。

```rust:リファクタ前（width1、height1に関係性は見て取れるがプログラム上に現れない）
fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```
```rust:リファクタ後（構造体でリファクタリングすることで意味付けする）
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of the rectangle is {} square pixels.", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

```rust:関数ではなくメソッドを使う
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 複数のメソッドが定義でき、引数も複数指定可能
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // selfを引数に取らないものは、対象インスタンスを持たないためメソッドではなく関連関数と呼ばれる
    // 関連関数は、構造体の新規インスタンスを返すコンストラクタによく使用されます。
    fn new(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 }; // インスタンス
    let rect2 = Rectangle::new ( size: 50 ); // こちらもインスタンス
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}
```
## 新しい方を定義するEnumとパターンマッチング
Enumの定義の仕方は構造体に似ていますが、構造体ではなくEnumを使う場面は、同時に両方にはなり得ないものを定義する時です。Enumは、とりうる値を列挙します。

```rust:列挙型（Enum）
enum IpAddrKind { // IPアドレスはx4かv6になり得ますが、同時に両方にはなりません
    V4(u8, u8, u8, u8),
    V6(String),
}
// IpAddrKindの各列挙子のインスタンス生成
let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```
Rustにはnullがないですが、値が存在するかしないかの概念はOption<T>が定義されています。

```rust
enum Option<T> { // <T>はジェネリック型引数
    Some(T),
    None,
}
// インスタンス生成
let some_number = Some(5);
let some_string = Some("a string");
// NoneはOption<T>の型が何になるか指定する必要がある
let absent_number: Option<i32> = None;

// `{integer}`と`Option<{integer}>`で型が違うのでエラー
// Option<{integer}>はNoneが想定されintegerではない。
let x = 5;
let sum = x + some_number;
```
matchはJAVAなどでいうところのswitchだが、取りうる値を全て定義する必要がある。

```rust:match
// Option<T>とのマッチ
// matchには、SomeとNoneが必要
let x = Some(5);
match x {
    Some(x) if x % 2 == 0 => println!("偶数です"),
    Some(x) => println!("奇数です"),
    None => println!("値がありません"), // 削除したらコンパイルエラー
}
// 全ての可能性を列挙したくない時
match x {
    Some(x) if x % 2 == 0 => println!("偶数です"),
    _ => println!("奇数もしくは、値がありません"), // _というパターンは、どんな値にもマッチする
}
```

## コレクション（ベクタ型、文字列、ハッシュマップ）
##### ベクタ
```rust
let v1: Vec<i32> = Vec::new(); // 空のベクタ生成。型注釈が必要
let v2 = vec![1, 2, 3]; // こちらの生成方法が一般的。型は推論される。
let mut v3 = vec![1, 2]; // 可変

// 参照
let s1 = &v3[0] // 1
let s2: Option<&i32> = v2.get(0); // Some(1)
let s3: Option<&i32> = v2.get(3); // None 3は存在しない添字

//　更新
v3.push(3);
println!("v3[0] is : {}", s1); // エラー
```
ベクタの更新（push）は、終端に追加しますが、隣り合った領域が十分ではなかった場合、新しい領域を確保して古い要素を新しい領域にコピーします。最後のエラーは、`s1`が何もない古い領域を参照している可能性がありエラーとなっています。

##### 文字列

```rust
 // 生成
let mut s1 = String::new();
s1.push_str("aaa"); // 3バイト
let s2 = String::from("あああ"); // 6バイト

let a = s1[0]; // Rustの文字列は添字をサポートしていないのでエラー
let b = s1[0..1]; // a
let b = s2[0..1]; // バイト数がたりずエラー
let b = s2[0..2]; // あ

// +はadd(self, &str)なので以下の形にする必要がある（+の前は所有権、＋のあとは参照件）
let c = s1 + &s2;
```

##### ハッシュマップ
```rust
// 生成
let mut scores1 = HashMap::new();

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

// 参照
let score = scores2.get("Blue"); // Some(&10)

for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// 更新
scores1.insert(String::from("Blue"), 10); // 更新
scores1.insert(String::from("Blue"), 25); // 上書き
scores1.entry(String::from("Yellow")).or_insert(50); // キーに値がなかった時のみ値を挿入
scores1.entry(String::from("Blue")).or_insert(50); //Blueは25のまま
```


## ?演算子
Resultの値がOkなら、Okの中身がこの式から返ってきて、プログラムは継続します。値がErrなら、 エラー値は呼び出し元のコードに委譲されます。

```rust
fn foo() -> Result<Value, Error>
// ?演算子は、Resultを返す関数でしか使用できない
let mut result1 = foo()?; // resultにはValue型が代入
let mut result2 = foo();  // resultにはResult<Value, Error>型が代入
// try!マクロを使う場合
let mut result = try!(foo());
```
## フロー制御：if文
```rust
if number % 4 == 0 {
    // 数値は4で割り切れます
} else if number % 3 == 0 {
    // 数値は3で割り切れます
} else {
    // 数値は4、3で割り切れません
}

// letの中に書くif
let condition = true;
let number = if condition { // number is 5
    5
} else {
    6
};
```

## フロー制御：ループ処理
Rustには`loop`、`while`、`for`の3種類が存在します。

```Rust
let a = [10, 20, 30, 40, 50];
let mut num = 0;

// 条件なく繰り返す（以下のようにbreakがない場合、Ctrl+Cなどでの終了が必要）
loop {
  println!("again!");
}

// 条件にマッチしない場合breakを呼び出し終了（breakを記述する必要はない）
while num < 5 {
  println!("{}", a[num]);
  num = num + 1;
}

// コレクション参照（上のwhileの方法は事故が起きやすいです）
for element in a.iter() {
  println!("{}", element);
}

//範囲を逆順に実行する
for number in (1..4).rev() {
  println!("{}!", number);
}
```

## 所有権
まずは、前提としてスタックとヒープについて

#####スタックとヒープ
スタックは、pushで得た順番に値を並べ、popで逆の順で値を取り除いていきます。データを置く場所も、データを取得する場所も、常に一番上、かつ固定サイズなので、高速です。 

コンパイル時にサイズがわからなかったり、サイズが可変のデータについては、ヒープに格納することができます。 ヒープにデータを置く時、OSはヒープ上に十分な大きさの空の領域を見つけ、使用中にし、ポインタを返します。ポインタを追って目的の場所に到達しなければならないため、スタックよりも低速です。 

コードが関数を呼び出すと、関数に渡された値(ヒープのデータへのポインタも含まれる可能性あり)と、 関数のローカル変数がスタックに載ります。関数の実行が終了すると、それらの値はスタックから取り除かれます。

##### メモリと確保
変数は、宣言された地点から、現在のスコープの終わりまで有効になります。

```rust:変数の寿命
{ // 宣言前。xもs1はまだ有効ではない。
    let s1 = String::from("hello"); // s1有効。
    let x = 5; // x有効。
} // xもs1は有効ではない。スコープの終わり。
```
String型では、コンパイル時には不明な量のメモリを ヒープに確保して内容を保持します。メモリは、実行時にOSに要求され、String型を使用し終わったら、OSにこのメモリを返還する方法が必要です。

##### 所有権の移動
整数は既知の固定サイズの単純な体なのでスタックに積まれる。

```rust:まず数値型
{
    let x = 5; // 値5をxに束縛
    let y = x; // xの値をyにコピーして、yに束縛
    println!("{}", x); // 5
}
```
String型は、ヒープまでコピーすることは処理の実行時性能がとても悪くなる可能性があるため、コピーではなくムーブが起こります。

```rust:String型
{
    let s1 = String::from("hello");
    let s2 = s1; // s1の値はs2にムーブ。s1は無効。
    println!("{}", s1); // エラー
}
```
文字型はヒープを確保し、ポインタをスタックで持ちます。
`let s1 = String::from("hello");`では、s1スタックに"hello"が格納されたヒープのポインタが格納されます。
`let s2 = s1;`では、s2はs1と同じポインタを格納します。つまり同一のヒープを参照するスタックs1,s2が有効になります。
このままs2とs1がスコープを抜けたら、 両方とも同じメモリを解放しようとします。これは二重解放エラーになります。
Rustは、`let s2 = s1;`の時点で、s1を無効としました（s1はs2にムーブされた）。無効なのでスコープを抜けた時にメモリ解放の必要がありません。
`println!("{}", s1);`では、無効な変数を使ったのでエラーが起きました。

s1とs2のヒープデータをそれぞれ持つ必要があるのであれば高コストですが`clone()`が使えます。

```rust:String型
{
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}", s1); // "hello"
}
```

##### 所有権の参照
関数呼び出しもムーブです。

```rust:
fn takes_ownership(s: String) {
}

let s1 = String::from("hello");
takes_ownership(s1); // sの値が関数にムーブされ有効ではなくなる
println!("{}", s1); // エラー
```
煩わしいですね。`&`をつけることで参照が使えます。

```rust:
fn calculate_length(s: &String) {
}// ここで、sはスコープ外。参照しているものの所有権を持っているわけではないので何も起こらない

let s1 = String::from("hello");
calculate_length(&s1);
let s2 = &s1; // 関数以外にも＆で参照可能
println!("{}", s1); // "hello"
```
ただし、参照は借用のため変更しようとするとエラーになります。

```rust:
fn change(some_string: &String) {
    some_string.push_str(", world"); // エラー
}
```
`&mut`にすることで可変な借用が使えます。しかし一回しか変更できない制約があり2回変更するとエラーになります。

```rust:
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
calculate_length(&mut s);
```
```rust:エラーになる参照
// 2回可変な借用をしている
let r1 = &mut s; // 問題なし
let r2 = &mut s; // 大問題！

//不変で借用されているのものを、可変で借用
let r1 = &s; // 問題なし
let r2 = &s; // 問題なし
let r3 = &mut s; // 大問題！

// 関数ないスコープの参照の戻り
fn dangle() -> &String {
    let s = String::from("hello");
    &s
} // スコープの終わりでsが有効ではなくなるのに参照が戻っている
```
##### 文字列スライス（&str）
文字列スライスとは、Stringの一部への参照

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];

// 文字列全体のスライスを得られます。
let len = s.len();
let slice = &s[0..len];
let slice = &s[..];
let slice = "hello world"; // 文字列リテラルはスライス(&str）
```
以下は`word`は`my_string`あってこその値なのでエラーになる良いコードです。

```rust
fn first_word(s: &str) -> &str {
    &s[..]
}
let s = String::from("hello world");
let word = first_word(&s[..]); // 戻り後もwordがmy_stringを不変借用続行
my_string.clear(); // error! （不変借用された変数を可変借用エラー！） 

```
以下はエラーにならないため、バグ原因の可能性があります。

```rust
fn first_word(s: &String) -> usize {
    s.len()
}
let mut s = String::from("hello world");
let word = first_word(&s);
s.clear(); // この時点でsから生成したwordは実質無効
}
```

## DisplayとDebugと{:?}{:#?}
println!マクロには、様々な整形があり、標準では、波括弧はDisplayとして知られる整形をする。
{:?}もしくは{:#?}とすることで、Debugで整形します。両者の違いは出力形式です。

```rust:構造体にはDisplayもDebugも実装が提供されないためエラーになります。
struct Rectangle {
    width: u32,
    height: u32,
}
let rect1 = Rectangle { width: 30, height: 50 };
println!("rect1 is {}", rect1); //エラー:構造体にはDisplay実装が提供されない
println!("rect1 is {:?}", rect1); //エラー:構造体にはDebug実装も提供されない
```

```rust:構造体にDebugマクロを追加
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
let rect1 = Rectangle { width: 30, height: 50 };
println!("rect1 is {:?}", rect1); //rect1 is Rectangle { width: 30, height: 50 }
println!("rect1 is {:#?}", rect1); 
//rect1 is Rectangle {
//    width: 30,
//    height: 50
//}
```

## 非同期
fnの代わりにasync fnと記述することで利用可能なasync機能は、コール時には単にFutrureを返す以外の何もしません。このFutureが実行を保留していて、.awaitすることで起動することができるのです。


```rust
async fn a_function() -> u32 { }

//`.await`は`async fn`か`async`ブロック内でしか呼び出せない
async fn another_function1() {
    let future = a_function();
    let result: u32 = future.await;
}
fn another_function2() {
    async {
        let r : u32 = a_function().await;
    }
}
```

理解が追いついていません。以下は後で読み返したいです。

https://tech-blog.optim.co.jp/entry/2019/11/08/163000
## テスト
Rustは、テストを単体テストと結合テストのカテゴリで捉えている。
単体テストは、テスト対象となるコードと共に、srcディレクトリの各ファイルに置きます。
結合テストは、プロジェクトディレクトリのトップ階層、srcの隣にtestsディレクトリを作成します。
参考：[Rustでテストコードをどこに書くべきか | teratail](https://teratail.com/questions/208572?sip=n0070000_019)

##### 単体テスト

```rust
#[cfg(test)]
mod tests {
    use super::*; // 外部モジュール内のテスト配下にあるコードを内部モジュールのスコープに持っていく

    #[test] // cargo testのみでビルドされるテスト関数
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7, };
        let smaller = Rectangle { width: 5, height: 1, };

        assert!(larger.can_hold(&smaller)); // trueならテストOK
        assert!(!larger.can_hold(&smaller)); // falseならテストOK
    }
    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2)); // ==ならテストOK
        assert_eq!(5, add_two(2)); // !=ならテストOK

        // 第二引数はカスタムエラ〜メッセージ
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }
    #[test]
    #[should_panic] // 状況がpanic!を引き起こすとテストする
    fn greater_than_100() {
        Guess::new(200);
    }
    // カスタムエラ〜メッセージ
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_m() {
        Guess::new(200);
    }
}
```

##### 結合テスト
ファイル：tests/integration_test.rs

```rust:
extern crate adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

結合テスト内のサブモジュール
ファイル名: tests/common/mod.rs

```rust
pub fn setup() {
    // ここにライブラリテスト固有のコードが来る
    // setup code specific to your library's tests would go here
}
```