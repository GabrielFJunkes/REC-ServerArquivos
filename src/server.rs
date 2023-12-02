use std::{net::{TcpListener, TcpStream}, thread, io::{Write, Read, self, BufRead}, fs::{self, File}, str, path::Path};

use util::Commands;

fn list(stream: &mut TcpStream) {
    let dir = fs::read_dir("./arquivos");
    match dir {
        Ok(dir) => {
            let mut files = String::new();
            for file in dir {
                files.push_str(file.unwrap().file_name().to_str().unwrap());
                files.push_str("\n");
            }
            let _ = stream.write_all(files.as_bytes());
            let _ = stream.flush();
        },
        Err(_) => {
            let _ = stream.write_all("Servidor encontrou um erro ao ler o diretório de arquivos!".as_bytes());
        },
    }
}

fn download(stream: &mut TcpStream) {
    let mut buf = Vec::new();
    buf.resize(100,0);
    if let Ok(n) = stream.read(&mut buf){
        buf = (&buf[0..n]).to_vec();
    }
    if buf.len()>0 {
        let file_name = String::from(str::from_utf8(&buf).unwrap());
        let path = format!("./arquivos/{file_name}");
        let file = Path::new(&path);
        let file_exists = file.is_file();
        if file_exists {
            let mut file = File::open(&path).unwrap();
            loop {
                let mut buf_file = [0; 4096];
                let n = file.read(&mut buf_file).unwrap();
                match n {
                    0 => {break}
                    _ => {
                        let _ = stream.write_all(&buf_file[..n]);
                    }
                }                
            }
        }else{
            let _ = stream.write_all("O arquivo não existe.".as_bytes());
        }
    }else{
        let _ = stream.write_all("Erro ao ler nome de arquivo!".as_bytes());
    }
}

fn upload(mut stream: &mut TcpStream) {
    let mut string = String::new();
    let mut reader = io::BufReader::new(&mut stream);
    let _ = reader.read_line(&mut string);
    if string.len()>0 {
        // Pega primeira linha
        let name = string.lines().next().unwrap();
        let path = format!("./arquivos/{}", name);

        if Path::new(&path).is_file() {
            let _ = fs::remove_file(&path);
        }
        
        let mut file = File::create(&path).unwrap();
        if let Ok(mut buf) = reader.fill_buf() {
            let _ = file.write(&mut buf);
            let _ = stream.write_all("Arquivo lido com sucesso.".as_bytes());
        }else{
            let _ = stream.write_all("Erro ao ler arquivo!".as_bytes());
        }
        println!("Acabou")
    }else{
        let _ = stream.write_all("Erro ao ler nome de arquivo!".as_bytes());
    }
}

fn _upload(stream: &mut TcpStream) {
    let mut buf = Vec::new();
    buf.resize(100,0);
    if let Ok(n) = stream.read(&mut buf){
        buf = (&buf[0..n]).to_vec();
    }
    if buf.len()>0 {
        let mut name_vec = Vec::new();
        let iter = buf.clone().into_iter();
        for (i, char) in iter.enumerate() {
            if char==0{
                buf = (&buf[(i+1)..(buf.len())]).to_vec();
                break;
            }else{
                name_vec.push(char);
            }
        }
        let file_name = String::from(str::from_utf8(&name_vec).unwrap());
        let path = format!("./arquivos/{file_name}");
        let file = Path::new(&path);
        if file.is_file() {
            let _ = fs::remove_file(&path);
        }
        let mut file = File::create(&path).unwrap();
        if buf.len()>0 {
            let _ = file.write_all(&mut buf);
        }
        loop {
            let mut buf_file = [0; 4096];
            match stream.read(&mut buf_file) {
                Ok(0) => { 
                    let _ = stream.write_all("Arquivo recebido com sucesso".as_bytes());
                    break; 
                }
                Ok(n) => {
                    println!("{n}");
                    let mut buf_file = &buf_file[0..n];
                    let _ = file.write(&mut buf_file);
                }
                Err(_) => {
                    let _ = stream.write_all("Erro ao ler arquivo!".as_bytes());
                    break;
                }
            }
        }
        println!("Acabou")
    }else{
        let _ = stream.write_all("Erro ao ler nome de arquivo!".as_bytes());
    }
}

fn delete(stream: &mut TcpStream) {
    let mut buf = Vec::new();
    buf.resize(100,0);
    if let Ok(n) = stream.read(&mut buf){
        buf = (&buf[0..n]).to_vec();
    }
    if buf.len()>0 {
        let file_name = String::from(str::from_utf8(&buf).unwrap());
        let path = format!("./arquivos/{file_name}");
        let file_exists = Path::new(&path).is_file();
        if file_exists {
            let result = fs::remove_file(path);
            match result {
                Ok(_) => {
                    let _ = stream.write_all("Arquivo excluido com sucesso.".as_bytes());
                },
                Err(_) => {
                    let _ = stream.write_all("Não foi possivel excluir o arquivo.".as_bytes());
                },
            }
        }else{
            let _ = stream.write_all("O arquivo não existe.".as_bytes());
        }
    }else{
        let _ = stream.write_all("Erro ao ler nome de arquivo!".as_bytes());
    }
}

fn handle_connection(stream: &mut TcpStream) {
    loop {
        let mut command = [4; 1];
        match stream.read(&mut command) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                match command[0].into() {
                    Commands::List => {
                        list(stream);
                    },
                    Commands::Upload => {
                        upload(stream);
                    },
                    Commands::Delete => {
                        delete(stream);
                    },
                    Commands::Download => {
                        download(stream);
                    },
                    Commands::None => {
                        println!("Entrou no 5");
                        break;
                    }
                }
            },
            Err(err) => {
                println!("{err}");
                let _ = stream.write_all("Comando inválido!".as_bytes());
                break;
            },
        }
    }
}

pub fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8888")?;
    println!("Servidor rodando em {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut stream_clone = stream.try_clone().unwrap();
                let _ = thread::Builder::new().name(stream.peer_addr().unwrap().to_string()).spawn(move || {
                    handle_connection(&mut stream);
                }).map_err(|err| {
                    let _ = stream_clone.write_all("Servidor está na capacidade maxima!".as_bytes());
                    eprintln!("Erro: {err}")
                });
            },
            Err(err) => {
                println!("Log erro: {err}");
            },
        };
    }
    Ok(())
}