use std::{
    io::{self, Read, Write},
    net::TcpStream,
};

fn main() {
    let ip = "127.0.0.1";
    let port = "8080";
    // connect to the server
    let mut stream =
        TcpStream::connect(format!("{}:{}", ip, port)).expect("Couldn't connect to the server");
    println!("Successfully connected to '{}:{}'!", ip, port);
    loop {
        // get input from user
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        // exit the loop if the user types "quit"
        if input.trim() == "quit" {
            break;
        }
        // write the input to the server
        stream
            .write_all(input.as_bytes())
            .expect("Error writing to stream");
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                // get the response from the server
                let response = std::str::from_utf8(&read[0..n])
                    .expect("Error reading response")
                    .trim_end_matches('\n');
                println!("{}", response);
            }
            Err(_) => panic!("Error"),
        }
    }
}
