use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    if let Ok(mut stream) = TcpStream::connect("localhost:8787") {
        println!("Connected to the server!");

        // Additional code to interact with the stream goes here
    } else {
        println!("Failed to connect to the server");
    }

    Ok(())
}
