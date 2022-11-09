fn main() {
    println!("Hello, world!");

    another_function1();
    another_function2(5);

    print_labeled_measurement(6,'h');

    let y = {
        let x = 3;
        //x+1没有;作为表达式返回值返回
        x+1
    };
    println!("The value of y is {y}");

    let x = five();
    println!("The value of x is {x}");

    let z = plus_one(5);
    println!("The value of z is {} and {}",z.0,z.1);
}


fn another_function1(){
    println!("Another function.");
}

fn another_function2(x:i32){
    println!("The value of x is :{x}")
}
fn print_labeled_measurement(value:i32,unit_label:char){
    println!("The measurement is: {value}{unit_label}");
}
//具有返回值的函数
fn five()->i32{
    5
}
fn plus_one(x:i32)->(i32,i32){
    //x+1作为返回值返回
    (x+1,x+2)
}