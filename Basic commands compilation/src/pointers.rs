pub fn run(){
    println!("insides poiters");
    let vec1= vec![1,2,34,4];
    let vec2=&vec1;
    println!("THsi si the value  {:?} {:?}",vec1,vec2);
}