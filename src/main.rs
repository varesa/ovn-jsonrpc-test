use serde::{Deserialize, Serialize};
use serde_json::de::Deserializer;
use serde_json::json;
use serde_json::Value;
use std::{io::Write, net::TcpStream};

struct JsonRpcConnection {
    stream: TcpStream,
    id: u64,
}

impl JsonRpcConnection {
    fn new(host: &str, port: u16) -> JsonRpcConnection {
        let stream = TcpStream::connect((host, port)).expect("Failed to connect");
        JsonRpcConnection { stream, id: 0 }
    }

    fn request(&mut self, method: &str, params: Option<Value>) -> Response {
        let request = Request {
            id: self.next_id().into(),
            method: method.into(),
            params,
        };
        self.stream
            .write(&serde_json::to_vec(&request).unwrap())
            .unwrap();
        let deserializer = Deserializer::from_reader(self.stream.try_clone().unwrap());
        let response: Response = deserializer.into_iter().next().unwrap().unwrap();
        assert!(response.id == request.id);
        return response;
    }

    fn next_id(&mut self) -> u64 {
        let current_id = self.id;
        self.id += 1;
        return current_id;
    }
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

fn main() {
    let mut connection = JsonRpcConnection::new("127.0.0.1", 6641);
    let params = vec!["_Server"];
    let schema = connection.request("get_schema", Some(json!(params))).result;
    println!("{schema:?}");
    let echo = connection.request("echo", Some(json!([])));
    assert!(echo.error.is_null());
    println!("Echo OK!");
}
