use std::{env, io, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}};

fn main() {
    let args: Vec<String> = env::args().collect();
    let address = (&args[1]).clone();
    let opt_type = (&args[2]).clone();
    let opt_listen = "listen".to_string();
    let opt_send = "send".to_string();
    if opt_type == opt_listen {
        listen(address);
    }else if opt_type == opt_send {
        let message = &args[3];
        println!("{:?}", send(address, message.clone()));
    }
}

fn listen(address: String){
    // 监听地址: 127.0.0.1:7878
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }


}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Message: {:#?}", request);
}

fn send(address: String, message: String) -> io::Result<()>{
    let mut stream = TcpStream::connect(address)?;
    stream.write(message.as_bytes()).expect("Fail to write!");
    Ok(())
}