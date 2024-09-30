use std::{env, fs, io, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread, time::Duration};
use tokio::sync::mpsc;
use std::error::Error;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let address = (&args[1]).clone();
    let opt_type = (&args[2]).clone();

    let mut message = &"None".to_string();
    let mut num = 1;
    let mut time = 500;
    let temp;

    match &opt_type[..] {
        "listen" => listen(address.clone()),
        "stall" => {
            let (tx, mut rx) = mpsc::channel(32);
            for i in 0..10 {
                let address = address.clone();
                let tx = tx.clone();
                tokio::spawn(async move {
                    // let address = address.clone();
                    loop {
                        {
                            let _ = stall(address.clone()).await;
                        }
                        let _ = tx.send(i).await;
                        // println!("{:?}", res);
                    }
                });
            }
            while let Some(i) = rx.recv().await {
                println!("{i} stall error.");
            }
        },
        "flood" => loop{
            let res = flood(address.clone());
            println!("{:?}", res);
        },
        "send" => {
            message = &args[3];
        },
        "sendf" => {
            let path = &args[3];
            temp = fs::read_to_string(path).unwrap();
            message = &temp;
        },
        "sendn" => {
            message = &args[3];
            num = args[4].clone().parse().unwrap();

        },
        "sends" => {
            message = &args[3];
            num = args[4].clone().parse().unwrap();
            time = args[5].clone().parse().unwrap();
        },
        _ => ()
    }

    if num == -1 {
        loop {
            println!("{:?}", send(address.clone(), message.clone()));
            thread::sleep(Duration::from_millis(time));
        }
    }
    for _ in 0..num {
        println!("{:?}", send(address.clone(), message.clone()));
        thread::sleep(Duration::from_millis(time));
    }
}

fn listen(address: String) -> !{
    // 监听地址: 127.0.0.1:7878
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    panic!("Listening end unexpected!");
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

async fn stall(address: String) -> Result<(), Box<dyn Error>> {
    use tokio::net::{TcpStream};
    use tokio::io::{AsyncWriteExt};
    loop {
        let mut stream = TcpStream::connect(&address).await?;
        // stream.set_nodelay(true);
        stream.write_all("hello".as_bytes()).await?;
        // stream.flush()?;
        thread::sleep(Duration::from_millis(10000));
        stream.write_all("hello".as_bytes()).await?;
        // stream.flush()?;
    }
    // panic!("Stall end Unexpected!");
}

fn flood(address: String) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect(address.clone())?;
    loop {
        stream.write("message".as_bytes())?;
    }
    // panic!("Flood end Unexpected!");
}
