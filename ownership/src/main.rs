fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    //if value borrowed here after move ->  print{s} err
    //let s1 = s;
    println!("{}",s);

    let t = s.clone();
    println!("s = {s}, t = {t}");

    drop(s);

    let s = String::from("hello");//s 进入作用域
    takes_ownership(s);//s 的值移动到了函数里...
    //... 所以到这里不再有效 = drop(s)

    let x = 5;//x 进入作用域

    makes_copy(x);//x 应该移动到函数里,
    //但 i32是Copyd ,
    //所以在后面可继续使用 x

    let s1 = gives_ownership(); //gives_ownership 将返回值转移给s1

    let s2 = String::from("hello");//s2进入作用域

    let s3 = takes_and_gives_back(s2);//s2 被移动到takes_and_gives_back中, 它也将返回值移交给s3

    println!("s1 is {s1},s3 is {s3}");

    //***
    //重点来了
    //***

    let s6 = String::from("hello");

    let (s8,len) = calculate_length(s6);

    println!("The length of `{}` is {}",s8,len)

}//这里,x 先移出了作用域,然后是s,但是因为 s 的值已被移走了
//没有特殊之处

fn takes_ownership(some_string:String){//some_string 进入作用域
    println!("{}",some_string);
}//这里, some_string 移出作用域并调用'drop'方法.
//占用的内存被释放

fn makes_copy(some_interger: i32){//some_integer 进入作用域
    println!("{}",some_interger);
}//这里, some_integer 移出作用域,没有特殊之处

fn gives_ownership()-> String{
    //gives_ownership 会将返回值移动给调用它的函数
    let some_string = String::from("yours");// some_ship进入作用域.

    some_string//返回 some_string
    //并移出给调用的函数
}

fn takes_and_gives_back(a_string:String)->String{
    a_string
}

fn calculate_length(s:String)->(String,usize){
    let length = s.len();

    (s,length)
}