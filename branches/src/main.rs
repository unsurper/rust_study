fn main() {
    let number = 3;
    if number<5{
        println!("condition was true");
    }else{
        println!("condition was false");
    }

    if number != 0{
        println!("number was three");
    }

    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is : {number}");

    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 10{
            break counter * 2;
        }
    };

    println!("The result is {result}");

    loopxh();
    whilexh();
    forxh();
    forxh1();
}

fn loopxh(){
    let mut count = 0 ;
    'counting_up:loop{
        println!("count = {count}");
        let mut remaining = 10 ;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count==2{
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn whilexh(){
    let mut number = 3;
    while number!=0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!");
}

fn forxh(){
    let a = [10,20,30,40,50];
    for element in a{println!("the value is: {element}")}
}

fn forxh1(){
    for number in (0..4).rev(){
        println!("{number}!");
    }
    println!("LIFTOFFF!!!");
}