use std::{net::{TcpListener, TcpStream}, thread, io::{Write, Read}, fs::{self, File}, path::Path};

use util::{Commands, read_string, read_size, send_string};
use std::io::ErrorKind::ConnectionReset;

fn list(stream: &mut TcpStream) {
    let dir = fs::read_dir("./arquivos_server");
    match dir {
        Ok(dir) => {
            let mut files = String::new();
            for file in dir {
                if let Ok(file) = file {
                    if let Some(string) = file.file_name().to_str() {
                        files.push_str(string);
                        files.push_str("\n");
                    }
                }
            }
            send_string(stream, &files);
        },
        Err(_) => {
            send_string(stream, "Servidor encontrou um erro ao ler o diretório de arquivos!");
        },
    }
}

fn download(stream: &mut TcpStream) {
    
    if let Some(string) = read_string(stream) {
        if string.len()>0 {
            let path = format!("./arquivos_server/{string}");
            let file = Path::new(&path);
            let file_exists = file.is_file();
            if file_exists {
                let buf = [1];
                let _ = stream.write_all(&buf);

                let file = &fs::read(&path).unwrap();
                let size = file.len().to_le_bytes();
                let _ = stream.write_all(&size);
                let _ = stream.write_all(&file);
            }else{
                let _ = stream.write_all(&[0;1]);
                send_string(stream, "O arquivo não existe!");
            }
        }else{
            let _ = stream.write_all(&[0;1]);
            send_string(stream, "Erro ao ler nome de arquivo!");
        }
    }
    
}


fn upload(stream: &mut TcpStream) {
    
    if let Some(string) = read_string(stream) {
        if string.len()>0 {
            let path = format!("./arquivos_server/{}", string);
    
            if Path::new(&path).is_file() {
                let _ = fs::remove_file(&path);
            }
            
            let mut file = File::create(&path).unwrap();
            let mut size = read_size(stream);
    
            loop {
                let mut buf_file = [0; 4096];
                match stream.read(&mut buf_file) {
                    Ok(0) => {
                        if size==0 {
                            send_string(stream, "Arquivo recebido com sucesso.");
                        }else{
                            send_string(stream, "Falha ao receber arquivo!");
                        }
                        break;
                    },
                    Ok(n) => {
                        let _ = file.write_all(&buf_file[..n]);
                        size -= n;
                    },
                    Err(_) => {
                        send_string(stream, "Falha ao receber arquivo!");
                        break;
                    }
                }
                if size==0 {
                    send_string(stream, "Arquivo recebido com sucesso.");
                    break;
                }
            }
        }else{
            send_string(stream, "Erro ao ler nome de arquivo!");
        }
    }
}

fn delete(stream: &mut TcpStream) {

    if let Some(string) = read_string(stream) {
        if string.len()>0 {
            let path = format!("./arquivos_server/{string}");
            let file_exists = Path::new(&path).is_file();
            if file_exists {
                let result = fs::remove_file(path);
                match result {
                    Ok(_) => {
                        send_string(stream, "Arquivo excluido com sucesso.");
                    },
                    Err(_) => {
                        send_string(stream, "Não foi possivel excluir o arquivo!");
                    },
                }
            }else{
                send_string(stream, "O arquivo não existe!");
            }
        }else{
            send_string(stream, "Erro ao ler nome de arquivo!");
        }
    }    
}

fn handle_connection(stream: &mut TcpStream) {
    send_string(stream, "Conectado com sucesso!");
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
                        break;
                    }
                }
            },
            Err(err) => {
                if err.kind() != ConnectionReset {
                    eprintln!("Erro no server: {err}");
                }
                break;
            },
        }
    }
}

pub fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8888")?;
    println!("Servidor rodando em {}", listener.local_addr().unwrap());
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut stream_clone = stream.try_clone().unwrap();
                let _ = thread::Builder::new().name(stream.peer_addr().unwrap().to_string()).spawn(move || {
                    handle_connection(&mut stream);
                }).map_err(|err| {
                    send_string(&mut stream_clone, "Servidor está na capacidade maxima!");
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