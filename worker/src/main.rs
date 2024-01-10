use log::{debug, error, info, trace, warn, LevelFilter};
use serde::{Deserialize, Serialize};
use shared::models::fragment::{
    fragment_request::{self, FragmentRequest},
    fragment_task::{self, FragmentTask},
};
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

fn main() -> std::io::Result<()> {
    let server = "localhost";
    let port = 8787;
    let mut stream = connect_to_server(server, port)?;

    let fragment_request = FragmentRequest {
        worker_name: "paschyz".to_string(),
        maximal_work_load: 1000,
    };
    let fragment_request_wrapped = serde_json::json!({"FragmentRequest": fragment_request});
    let request_str = serde_json::to_string(&fragment_request_wrapped)?;
    let total_size = request_str.len() as u32;
    stream.write(&total_size.to_be_bytes())?;
    stream.write(&total_size.to_be_bytes())?;
    stream.write(request_str.as_bytes())?;
    let mut length_byte = [0u8; 4];
    let total_size_response = stream.read_exact(&mut length_byte)?;
    println!("{:?}", total_size_response);
    stream.read_exact(&mut length_byte)?;
    println!("{:?}", length_byte);
    let json_size = u32::from_be_bytes(length_byte);

    let mut json_byte = vec![0u8; json_size as usize];
    stream.read_exact(&mut json_byte)?;
    let string_fragment_task = String::from_utf8_lossy(&json_byte);
    let fragment_value: serde_json::Value = serde_json::from_str(&string_fragment_task)?;
    println!("{:?}", fragment_value);
    let fragment_task: FragmentTask =
        serde_json::from_value(fragment_value["FragmentTask"].clone())?;
    println!("{:?}", fragment_task);

    Ok(())
}

pub fn connect_to_server(server: &str, port: u16) -> Result<TcpStream, std::io::Error> {
    let socket_addr = format!("{}:{}", server, port);
    println!("Connecting to server: {}", &socket_addr);

    let stream = TcpStream::connect(&socket_addr)?;

    println!("Connected to server: {}", &socket_addr);
    Ok(stream)
}
