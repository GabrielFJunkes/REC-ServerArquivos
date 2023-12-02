use std::{net::TcpStream, str, io::{Write, Read, self}, fs::{File, self}};
use util::Commands;


fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8888") {
        println!("Connected to the server!");
        println!("Teste de listar!");
        let _ = stream.write_all(&[Commands::List as u8]);
        let _ = stream.flush();
        let mut buf = Vec::new();
        buf.resize(100,0);
        if let Ok(n) = stream.read(&mut buf){
            let buf = &buf[0..n];
            let string = str::from_utf8(&buf).unwrap();
            println!("{string}");
        }
        println!("Teste de excluir!");
        let _ = stream.write_all(&[Commands::Delete as u8]);
        let _ = stream.flush();
        let string = String::from("teste");
        let _ = stream.write_all(string.as_bytes());
        let mut buf = Vec::new();
        buf.resize(100,0);
        if let Ok(n) = stream.read(&mut buf){
            let buf = &buf[0..n];
            let string = str::from_utf8(&buf).unwrap();
            println!("{string}");
        }
        println!("Teste de download!");
        let _ = stream.write_all(&[Commands::Download as u8]);
        let _ = stream.flush();
        let string = String::from("test.txt");
        let _ = stream.write_all(string.as_bytes());
        let mut buf_file = [0; 4096];
        let mut file = File::create(&string).unwrap();
        if let Ok(n) = stream.read(&mut buf_file){
            let mut buf_file = &buf_file[0..n];
            let _ = file.write(&mut buf_file);
        }

        println!("Teste de upload!");
        let _ = stream.write_all(&[Commands::Upload as u8]);
        let _ = stream.flush();
        let string = String::from("New_Infinity.apk\n");
        let _ = stream.write_all(string.as_bytes());
        let _ = stream.flush();
        
        let file = &fs::read(&string.trim()).unwrap();
        let _ = stream.write_all(&file);
        let _ = stream.flush().unwrap();
        let mut buf = Vec::new();
        buf.resize(100,0);
        if let Ok(n) = stream.read(&mut buf){
            let buf = &buf[0..n];
            let string = str::from_utf8(&buf).unwrap();
            println!("{string}");
        }
    } else {
        println!("Couldn't connect to server...");
    }
}
