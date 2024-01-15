use log::{debug, error, info, trace, warn, LevelFilter};
use serde::{Deserialize, Serialize};
use shared::models::{
    fragment::{
        fragment_request::{self, FragmentRequest},
        fragment_result::{self, FragmentResult},
        fragment_task::{self, FragmentTask},
    },
    pixel::pixel_data::PixelData,
    point::Point,
    range::Range,
    resolution::Resolution,
    u8_data::U8Data,
};
use std::{
    io::{Read, Write},
    net::{Shutdown, SocketAddr, TcpStream},
    thread,
};

fn main() -> std::io::Result<()> {
    let server = "localhost";
    let port = 8787;
    loop {
        thread::sleep(std::time::Duration::from_secs(1));
        let mut stream = connect_to_server(server, port)?;

        let fragment_request = FragmentRequest {
            worker_name: "paschyz".to_string(),
            maximal_work_load: 1000,
        };
        let _ = send_fragment_request(fragment_request, &mut stream);

        let mut total_response_message_size = [0u8; 4]; // u8*4 = 32
        let mut json_response_message_size = [0u8; 4];
        let mut data_response_message_size = [0u8; 4];

        let fragment_task: Result<FragmentTask, std::io::Error> = receive_task(
            &mut total_response_message_size,
            &mut json_response_message_size,
            &mut data_response_message_size,
            &mut stream,
        );
        let _ = stream.shutdown(Shutdown::Both);
        loop {
            thread::sleep(std::time::Duration::from_secs(1));
            let mut stream = match connect_to_server(server, port) {
                Ok(stream) => stream,
                Err(e) => {
                    println!("Failed to connect to server : {}", e);
                    continue;
                }
            };

            let mut fragment_result = FragmentResult {
                id: {
                    U8Data {
                        offset: 4,
                        count: 4,
                    }
                },
                resolution: { Resolution { nx: 800, ny: 600 } },
                range: {
                    Range {
                        min: Point { x: 0.5, y: 0.2 },
                        max: Point { x: 0.8, y: 0.9 },
                    }
                },
                pixels: {
                    PixelData {
                        offset: 100,
                        count: 5000,
                    }
                },
            };

            match fragment_task {
                Ok(ref task) => {
                    fragment_result.id = task.id.clone();
                    fragment_result.resolution = task.resolution.clone();
                    fragment_result.range = task.range.clone();
                }
                Err(ref e) => {
                    println!("error fragment_task{}", e);
                    continue;
                }
            }

            let _ = send_fragment_result(fragment_result, &mut stream);
            let mut total_response_message_size = [0u8; 4]; // u8*4 = 32
            let mut json_response_message_size = [0u8; 4];
            let mut data_response_message_size = [0u8; 4];

            let fragment_task: Result<FragmentTask, std::io::Error> = match receive_task(
                &mut total_response_message_size,
                &mut json_response_message_size,
                &mut data_response_message_size,
                &mut stream,
            ) {
                Ok(fragment_task) => Ok(fragment_task),
                Err(e) => {
                    println!("Failed to receive_task : {}", e);
                    continue;
                }
            };
        }
    }
}

pub fn connect_to_server(server: &str, port: u16) -> Result<TcpStream, std::io::Error> {
    let socket_addr = format!("{}:{}", server, port);
    println!("Connecting to server: {}", &socket_addr);

    let stream = TcpStream::connect(&socket_addr)?;

    println!("Connected to server: {}", &socket_addr);
    Ok(stream)
}

pub fn send_fragment_request(
    fragment_request: FragmentRequest,
    stream: &mut TcpStream,
) -> std::io::Result<()> {
    // on transforme fragment_request en json
    let fragment_request_json = serde_json::json!({"FragmentRequest": fragment_request});
    // on transforme fragment_request_json en string
    let request_str = serde_json::to_string(&fragment_request_json)?;
    // on prélève la size de request_str
    let total_size = request_str.len() as u32;
    stream.write(&total_size.to_be_bytes())?; // Total message size
    stream.write(&total_size.to_be_bytes())?; // JSON message size
    stream.write(request_str.as_bytes())?; // JSON message
    Ok(())
}
pub fn send_fragment_result(
    fragment_result: FragmentResult,
    stream: &mut TcpStream,
) -> std::io::Result<()> {
    let fragment_result_json = serde_json::json!({"FragmentResult":fragment_result});
    let result_str = serde_json::to_string(&fragment_result_json)?;
    let offset_size = fragment_result.pixels.offset.to_be_bytes();
    let count_size = fragment_result.pixels.count.to_be_bytes();
    let total_size = (result_str.len() + offset_size.len() + count_size.len()) as u32;
    let json_size = result_str.len() as u32;

    println!("total_size: {:?}", total_size);
    println!("result_str: {:?}", result_str);
    println!("result_str.len(): {:?}", result_str.len());
    println!("&total_size.to_be_bytes(): {:?}", &total_size.to_be_bytes());
    stream.write(&total_size.to_be_bytes())?; // Total message size
    stream.write(&json_size.to_be_bytes())?; // JSON message size
    stream.write(result_str.as_bytes())?; // JSON message
    stream.write(&offset_size)?; // Data
    stream.write(&count_size)?; // Data

    println!("{:?}", fragment_result.pixels);
    Ok(())
}
pub fn receive_task(
    total_response_message_size: &mut [u8; 4],
    json_response_message_size: &mut [u8; 4],
    data_response_message_size: &mut [u8; 4],
    stream: &mut TcpStream,
) -> Result<FragmentTask, std::io::Error> {
    println!("receive_task");
    stream.read_exact(total_response_message_size)?; // on lit le u32 total_size
    stream.read_exact(json_response_message_size)?; // on relit le u32 qui correspond au json message size
    let json_resonse_size = u32::from_be_bytes(*json_response_message_size);
    println!("{:?}", json_resonse_size);
    let mut json_byte = vec![0u8; json_resonse_size as usize];
    stream.read_exact(&mut json_byte)?;
    let string_fragment_task = String::from_utf8_lossy(&json_byte); // string
    let fragment_value: serde_json::Value = serde_json::from_str(&string_fragment_task)?; // string to json
    println!("{:?}", fragment_value);
    let fragment_task: FragmentTask =
        serde_json::from_value(fragment_value["FragmentTask"].clone())?; // retrieve fragmenTask from json dictionnary
    println!("{:?}", fragment_task);
    stream.read_exact(data_response_message_size);
    println!("{:?}", data_response_message_size);

    Ok(fragment_task)
}
