use tokio::net::{TcpListener,TcpStream};
use tokio::io::AsyncReadExt;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>> {
    let binding = ":::10113";
    let mut listener = TcpListener::bind(binding).await?;

    loop {
        let (socket,_)= listener.accept().await?;
        // let _p=handle_client(socket).await;
        tokio::spawn(async move {
            let _ = handle_client(socket).await;
        }); 
    }
}

async fn handle_client (mut socket: TcpStream) -> Result<(),Box<dyn Error>>{
    let remote_ip = socket.peer_addr()?.ip();
    println!("This si the recieved addr from client {}", remote_ip);
    let mut i=0;
    loop{
        let mut buf = [0;100];
        let n =socket.read(&mut buf).await?;
        i+=1;
        if n==0 {
            break;
        }
        let received= String::from_utf8(buf[0..n].to_vec())?;
        println!("Message number {} , Message : {} with ip {}",i,received,socket.peer_addr()?.ip());
    }
    Ok(())
}