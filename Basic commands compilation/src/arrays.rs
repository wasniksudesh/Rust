pub fn run(){
    let mut arr: [i8;5]=[1,2,3,4,5];
    println!("{:?}",arr);
    // GEt Slice of the array 
    let slice : &[i8]=&arr[0..2];
    println!("Here we come {:?}",slice);

}