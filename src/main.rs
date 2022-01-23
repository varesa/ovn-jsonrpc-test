use serde::{Deserialize, Serialize};
use serde_json::de::Deserializer;
use serde_json::json;
use serde_json::Value;
use std::{
    io::{Write},
    net::TcpStream,
};

struct JsonRpcConnection {
    stream: TcpStream,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub id: Value,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Response {
    pub result: Value,
    pub error: Value,
    pub id: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    /// An RPC request.
    Request(Request),
    /// A response to a Request.
    Response(Response),
}

impl JsonRpcConnection {
    fn new(host: &str, port: u16) -> JsonRpcConnection {
        let stream = TcpStream::connect((host, port)).expect("Failed to connect");
        JsonRpcConnection { stream }
    }

    fn request(&mut self, method: &str, params: Option<Value>) {
        let payload = Request {
            id: 1.into(),
            method: method.into(),
            params,
        };
        //let mut payload = String::from(r#"{"id": 1, "method": "get_schema", "params": ["_Server"]}"#);
        //payload.push('\n');
        self.stream
            .write(&serde_json::to_vec(&payload).unwrap())
            .unwrap();
        self.stream.write(b"\n").unwrap();
        let deserializer = Deserializer::from_reader(self.stream.try_clone().unwrap());
        let decoded: Message = deserializer.into_iter().next().unwrap().unwrap();
        println!("{decoded:?}");
    }
}

fn main() {
    let mut connection = JsonRpcConnection::new("127.0.0.1", 6641);
    let params = vec!["_Server"];
    connection.request("get_schema", Some(json!(params)));
}
