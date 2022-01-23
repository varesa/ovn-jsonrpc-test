mod jsonrpc;
mod ovn;

use crate::ovn::Ovn;

fn main() {
    let host = "127.0.0.1";
    let port = 6641;

    let mut ovn = Ovn::new(host, port);
    //ovn.print_schema();

    ovn.echo();
    println!("Echo OK!");
    ovn.list_ls();
}
