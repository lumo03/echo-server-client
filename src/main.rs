use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

fn main() {
    let ip = "127.0.0.1";
    let port = "8080";
    let stream = TcpStream::connect(format!("{}:{}", ip, port));
    match stream {
        Ok(mut stream) => {
            println!("Connected");
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if input.trim() == "quit" {
                    break;
                }
                stream.write_all(input.as_bytes()).unwrap();

                let mut read = [0; 1028];

                match stream.read(&mut read) {
                    Ok(n) => {
                        if n == 0 {
                            break;
                        }
                        let response = std::str::from_utf8(&read[0..n])
                            .unwrap()
                            .trim_end_matches('\n');
                        println!("{}", response);
                    }
                    Err(_) => panic!("Error"),
                }
            }
        }
        Err(_) => panic!("Couldn't connect to the server."),
    }
}
