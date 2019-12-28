struct Person (String,String);

impl Person{
    fn single(one:&str,two:&str)->Person{
        Person(
            one.to_string(),two.to_string())
    }
    fn fullname(&self)->String{
        format!("{},{}",self.0,self.1)
    }
}

pub fn run(){
    let var1 = Person::single("Yeah","NO");
    println!("This si new and good {},{}",var1.0,var1.1);
    println!("This is second new {}",var1.fullname());
}