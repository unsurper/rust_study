use std::io;
/*
fn main1() {
    //let x = 5; //不可变变量报错
    let mut x = 5;//可变变量
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    //Rust 对常量的命名约定是在单词之间使用全大写加下划线
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("const is {}",THREE_HOURS_IN_SECONDS);
    //隐藏(Shadowing)
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    //变量默认是不可变的,不可变的变量可以改变类型相当于重写,可变的变量类型是不可变的
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The spaces of spaces is: {spaces}");
    // let mut spaces = "   ";
    // spaces = spaces.len();

    let guess :u32 = "42".parse().expect("Not a Number");
    println!("guess is {}",guess);

    //元组类型
    let tup = (500, 6.4, 1);
    //解构（destructuring）,将一个元组拆成了三个部分
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    println!("five_hundred {},six_point_four {},one {}",five_hundred,six_point_four,one);
    //数组
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a first is {}",a[0]);
    //这些元素的值最初都将被设置为 3
    let a = [3; 5];

}

 */

/*
fn main2() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

 */

fn main() {

}