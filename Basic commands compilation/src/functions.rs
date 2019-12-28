pub fn run(){
    println!("Inside functions");
    let add_num = |n1:i32, n2:i32| n1+n2;
    println!("C.Sum {}",add_num(1,2));
    println!("This si brand {}",five(34,21));
    strin("yeah","no");
}

fn strin(one:&str,two:&str){
    println!("{},{}",one,two);
}
fn five(n1:i32,n2:i32)-> i32{
    println!("this is new ");
    n1+n2
}