pub fn run(){
    let arr:[i8;1]=[10];
    println!("inside vectors {:?}",arr);
    let mut vecc:Vec<i32> = vec![2,3,4,5];
    println!("This si noew {:?}",vecc);
    vecc.push(23);

    println!("thisis new {:?}",vecc);
    // Using a loop here
    for x in vecc.iter_mut(){
        *x *= 2;
        println!("this si in loop {}",x);
    }
}