use std::{net::TcpStream, io::{Read, Write}, str, process, fs::{self, File}};

#[repr(u8)]
pub enum Commands {
    List,
    Upload,
    Delete,
    Download,
    None
}
impl From<Commands> for u8 {
    fn from(m: Commands) -> u8 {
        m as u8
    }
}

impl Into<Commands> for u8 {
    fn into(self) -> Commands {
        match self {
            0 => Commands::List,
            1 => Commands::Upload,
            2 => Commands::Delete,
            3 => Commands::Download,
            _ => Commands::None,
        }
    }
}

pub fn read_size(stream: &mut TcpStream) -> usize {
    let mut buf_size: [u8; 8] = [0; 8];
    let _ = stream.read_exact(&mut buf_size);
    usize::from_le_bytes(buf_size)   
}

pub fn read_string(stream: &mut TcpStream) -> Option<String> {
    let size = read_size(stream);
    if size==0 {
        return None
    }
    let mut buf_name: Vec<u8> = Vec::new();
    buf_name.resize(size, 0);
    let _ = stream.read_exact(&mut buf_name);
    let string = str::from_utf8(&buf_name).unwrap();
    Some(String::from(string))
}

pub fn send_string(stream: &mut TcpStream, string: &str) {
    let size = string.len().to_le_bytes();
    let _ = stream.write_all(&size);
    let _ = stream.write_all(string.as_bytes());
}

pub fn read_server_string(stream: &mut TcpStream, print: bool) {
    if let Some(string) = read_string(stream){
        if print{
            println!("Servidor: {string}");
        }
    }else{
        panic!("Servidor fechou a conexão!");
    }
}

pub fn list(stream: &mut TcpStream) {
    let _ = stream.write_all(&[Commands::List as u8]);
    let _ = stream.flush();
    if let Some(string) = read_string(stream){
        println!("Arquivos no server:");
        for line in string.lines() {
            println!("\t{line}");
        }
    }else{
        eprintln!("Servidor fechou a conexão!");
        process::exit(0x0100);
    }
    
}

pub fn upload(stream: &mut TcpStream, file_name: &str, print: bool) {
    let _ = stream.write_all(&[Commands::Upload as u8]);
    let _ = stream.flush();
    send_string(stream, &file_name);
    let _ = stream.flush();
    
    let file = &fs::read(&file_name.trim()).unwrap();
    let size = file.len().to_le_bytes();
    let _ = stream.write_all(&size);
    let _ = stream.write_all(&file);
    let _ = stream.flush();
    read_server_string(stream, print);
}

pub fn delete(stream: &mut TcpStream, file_name: &str, print: bool) {
    let _ = stream.write_all(&[Commands::Delete as u8]);
    let _ = stream.flush();
    send_string(stream, &file_name);
    read_server_string(stream, print);
}

pub fn download(stream: &mut TcpStream, file_name: &str, print: bool) {
    let _ = stream.write_all(&[Commands::Download as u8]);
    let _ = stream.flush();
    send_string(stream, &file_name);
    
    let mut buf = [0;1];
    println!("1 {buf:?}");
    let _ = stream.read_exact(&mut buf);

    println!("1 {:?}", buf[0]);

    if buf[0] == 1 {

        let mut file = File::create(&file_name).unwrap();
        let mut size = read_size(stream);
        loop {
            if size==0 {
                println!("Arquivo recebido com sucesso.");
                break;
            }
            let mut buf_file = [0; 4096];
            match stream.read(&mut buf_file) {
                Ok(0) => {
                    break;
                },
                Ok(n) => {
                    let _ = file.write_all(&buf_file[..n]);
                    size -= n;
                },
                Err(_) => {
                    eprintln!("Falha ao receber arquivo!");
                    break;
                }
            }
            
        }  
    }else{
        read_server_string(stream, print);
    }
}