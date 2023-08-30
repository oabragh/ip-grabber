use std::io::Result;
use std::net::TcpListener;

use ip_grabber::get_ip_from_stream;

fn main() -> Result<()> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:6969")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let ip_addr = get_ip_from_stream(stream)?;

        println!("Got ip: {}", ip_addr);
    }

    Ok(())
}
