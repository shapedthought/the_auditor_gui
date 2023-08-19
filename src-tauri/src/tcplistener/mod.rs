use anyhow::Result;
use regex::Regex;
use std::io::{Read, Write};
use std::net::TcpListener;

#[allow(dead_code)]
pub async fn run_tcp_listener(address: String) -> Result<String> {
    let re = Regex::new(r":(\d+)/").unwrap();
    let port = if let Some(caps) = re.captures(&address) {
        format!(":{}", caps[1].to_string())
    } else {
        ":80".to_string()
    };
    let address = format!("127.0.0.1{}", port);
    let listener = TcpListener::bind(address)?;

    println!("Listening on {}", listener.local_addr()?);

    let mut data = String::new();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 2048];
                stream.read(&mut buffer)?;
                let request = String::from_utf8_lossy(&buffer[..]);

                if request.starts_with("GET") {
                    let re = Regex::new(r"GET /([^ ]+)").unwrap();
                    data = re.captures(&request).unwrap()[1].to_string();
                    let http_response =
                        "HTTP/1.1 200 OK\r\n\r\n You can close this window now.".to_string();
                    stream
                        .write_all(http_response.as_bytes())
                        .expect("Failed to write to stream");

                    break;
                }
            }
            Err(e) => {
                return Err(anyhow::anyhow!("Unable to connect: {}", e));
            }
        }
    }
    Ok(data)
}
