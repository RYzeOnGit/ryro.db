fn main() {
    let x = 5;
    let mut y = 10;


    println!("{}", x);
    println!("y before = {}", y);
    y = 20;
    println!("y after = {}", y);

    let a = "hello";                // &str
    let b = String::from("hello");  // String

    println!("a = {}", a);
    println!("b = {}", b);

    let key = "name";
    let key2 = key.to_string();
    println!("{}", key2);

}