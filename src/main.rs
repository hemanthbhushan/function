use std::io;

fn main() {
    let mut num = String::new();
    let mut name: String = String::new();
    println!("enter the number");

    io::stdin()
        .read_line(&mut num)
        .expect("enter proper number");
    io::stdin()
        .read_line(&mut name)
        .expect("enter proper number");

    let num: i32 = num.trim().parse().expect("error");
    println!("Hello, world!");
    check(num, &name);
    println!("{}",print(num));
}

fn check(_num: i32, _name: &str)  {
    println!("the number is {_num} and the name is {_name} : check Function");
}
fn print(_num : i32) -> i32{
    _num+1
}
