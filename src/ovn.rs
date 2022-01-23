use crate::jsonrpc::JsonRpcConnection;
use serde_json::json;

pub struct Ovn {
    connection: JsonRpcConnection,
}

impl Ovn {
    pub fn new(host: &str, port: u16) -> Self {
        Ovn {
            connection: JsonRpcConnection::new(host, port),
        }
    }

    pub fn echo(&mut self) {
        let echo = self.connection.request("echo", Some(json!([])));
        assert!(echo.error.is_null());
    }

    pub fn list_ls(&mut self) {
        let ls = self.connection.request(
            "monitor_cond_since",
            Some(json!([
                "OVN_Northbound",
                ["monid", "OVN_Northbound"], 
                {
                    "Logical_Switch": [{"columns": ["name"]}]
                },
                "00000000-0000-0000-0000-000000000000"
            ])),
        );
        println!("{ls:#?}");
    }
}
