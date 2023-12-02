use std::{net::TcpStream, io::{Read, Write}, str};

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