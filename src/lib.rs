use std::fs;
use std::io::{prelude::*, BufReader, Error, ErrorKind, Result};
use std::net::{Ipv4Addr, TcpStream};

pub fn get_ip_from_stream(mut stream: TcpStream) -> Result<Ipv4Addr> {
    let bufreader = BufReader::new(&mut stream);
    let http_request = bufreader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>();

    // Respond with a 404 page
    let contents = fs::read_to_string("404.html")?;
    let response = format!(
        "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes())?;

    // Parse X-Forwarded-For
    for i in http_request {
        let x: Vec<&str> = i.split(':').collect();

        if x[0] == "X-Forwarded-For" {
            return x[1]
                .trim()
                .parse::<Ipv4Addr>()
                .map_err(|_| Error::new(ErrorKind::Other, "Couldn't parse to Ipv4Addr."));
        }
    }

    Err(Error::new(
        ErrorKind::Other,
        "Couldn't get IP from request.",
    ))
}
