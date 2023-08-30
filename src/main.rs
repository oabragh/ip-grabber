use std::io::Result;
use std::net::TcpListener;

use ip_grabber::get_ip_from_stream;

fn main() -> Result<()> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:6969")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let ip_addr = get_ip_from_stream(stream)?;

        println!("Got ip: {}", ip_addr);

        //send_to_channel(
        //    get_ip_from_stream(stream)?,
        //    "CHANNEL ID",
        //    "TOKEN",
        //)?;
    }

    Ok(())
}

//fn send_to_channel(ip_addr: Ipv4Addr, channel_id: &str, discord_token: &str) -> Result<()> {
//    let json_payload = format!("{{\"content\": \"Got ip address: {}\"}}", ip_addr);
//    let mut request = String::new();
//
//    request.push_str(&format!(
//        "POST /api/v9/channels/{}/messages/ HTTP/1.1\r\n",
//        channel_id
//    ));
//    request.push_str("Host: discord.com\r\n");
//    request.push_str(&format!("Authorization: {}\r\n", discord_token));
//    request.push_str("Content-Type: application/json\r\n");
//    request.push_str(&format!("Content-Length: {}\r\n\r\n", json_payload.len()));
//    request.push_str(&json_payload);
//
//    println!("{}", request);
//
//    let mut discord_stream = TcpStream::connect("discord.com:443")?;
//    discord_stream.write_all(request.as_bytes())?;
//
//    let mut response = String::new();
//    discord_stream.read_to_string(&mut response)?;
//
//    println!("Response:\n{}", response);
//
//    Ok(())
//}
