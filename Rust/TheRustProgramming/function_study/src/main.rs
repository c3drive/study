fn main() {
    println!("Hello, world!");

    let some_number = Some(5);
    let _some_string = Some("a string");

    let _absent_number: Option<i32> = None;

    let _x = 5;
    let _y: Option<i8> = Some(5);

    // let sum = x + some_number;
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let six = plus_one(Some(5));
    let none = plus_one(None);

    println!("{:?}", six);
    println!("{:?}", none);

    match some_number {
        Some(some_number) if some_number % 2 == 0 => println!("偶数です"),
        Some(some_number) => println!("奇数です"),
        None => println!("値がありません"),
    }
    match some_number {
        Some(some_number) if some_number % 2 == 0 => println!("偶数です"),
        _ => println!("奇数もしくは、値がありません"),
    }
}
