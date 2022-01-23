use std::{io::{BufReader, Write, BufRead}, net::TcpStream, thread};

struct JsonRpcConnection {
    stream: TcpStream,
    reader: BufReader<TcpStream>,
}

impl JsonRpcConnection {
    fn new(host: &str, port: u16) -> JsonRpcConnection {
        let stream = TcpStream::connect((host, port)).expect("Failed to connect");
        let reader = BufReader::new(stream.try_clone().unwrap());
        JsonRpcConnection { stream, reader }
    }

    fn request(&mut self, method: &str, params: &str) {
        let mut payload = String::from(r#"{"id": 1, "method": "get_schema", "params": ["_Server"]}"#);
        payload.push('\n');
        self.stream.write(payload.as_bytes()).unwrap();
        loop {
            let mut buf = String::new();
            if self.reader.read_line(&mut buf).unwrap() == 0 {
                break;
            }

            println!("{buf}");
        }
    }

    fn close(self) {
        self.stream.shutdown(std::net::Shutdown::Both).unwrap();
    }
}

fn main() {
    println!("Hello, world!");

    let mut connection = JsonRpcConnection::new("127.0.0.1", 6641);
    connection.request("get_schema", r#"["_Server"]"#);
    connection.close();

    
    // get_schema", ["_Server"]
}
