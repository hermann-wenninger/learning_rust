use std::io::{self, BufRead};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(stream: TcpStream) {
    let mut reader = io::BufReader::new(&stream);

    loop {
        let mut input = String::new();
        let bytes_read = reader.read_line(&mut input).expect("Failed to read from client");

        if bytes_read == 0 {
            break;
        }

        println!("Received mouse coordinates: {}", input.trim());
        // Hier können Sie die erhaltenen Koordinaten weiterverarbeiten

        // Beispiel: Koordinaten an andere Funktion übergeben
        process_coordinates(input.trim());
    }
}

fn process_coordinates(coordinates: &str) {
    // Hier können Sie die Mauskoordinaten weiterverarbeiten, z.B. in Ihrem Spiel verwenden
    println!("Processing coordinates: {}", coordinates);
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8077")?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}