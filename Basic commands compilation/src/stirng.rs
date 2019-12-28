pub fn run(){
    println!("bithc are your ingngin");
    let mut hello= String::from("hhe;l;poo");
    hello.push('U');
    hello.push_str("This si new");
    println!("bitch {} and length {}",hello,hello.len());
    println!("Contains word {}",hello.contains("bitch"));
}