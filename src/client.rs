use std::{net::TcpStream, str, io::{Write, Read, self, stdout}, fs::{File, self}};
use util::{Commands, read_size, send_string, read_string};

fn read_server_string(stream: &mut TcpStream) {
    let string = read_string(stream);
    println!("Servidor: {string}");
}

fn list(stream: &mut TcpStream) {
    let _ = stream.write_all(&[Commands::List as u8]);
    let _ = stream.flush();
    let string = read_string(stream);
    for line in string.lines() {
        println!("\t{line}");
    }
}

fn upload(stream: &mut TcpStream, file_name: &str) {
    let _ = stream.write_all(&[Commands::Upload as u8]);
    let _ = stream.flush();
    send_string(stream, &file_name);
    let _ = stream.flush();
    
    let file = &fs::read(&file_name.trim()).unwrap();
    let size = file.len().to_le_bytes();
    let _ = stream.write_all(&size);
    let _ = stream.write_all(&file);
    let _ = stream.flush();
    read_server_string(stream);
}

fn delete(stream: &mut TcpStream, file_name: &str) {
    let _ = stream.write_all(&[Commands::Delete as u8]);
    let _ = stream.flush();
    send_string(stream, &file_name);
    read_server_string(stream);
}

fn download(stream: &mut TcpStream, file_name: &str) {
    let _ = stream.write_all(&[Commands::Download as u8]);
    let _ = stream.flush();
    send_string(stream, &file_name);

    
    let mut buf = [0;1];
    let _ = stream.read_exact(&mut buf);

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
        read_server_string(stream);
    }
}

fn list_local() {
    let dir = fs::read_dir(".");
    match dir {
        Ok(dir) => {
            for file in dir {
                if let Ok(file) = file {
                    if file.file_type().unwrap().is_file() {
                        if let Some(string) = file.file_name().to_str() {
                            println!("\t{}", string);
                        }
                    }
                }
            }
        },
        Err(_) => {
            eprintln!("Erro ao ler arquivos locais")
        },
    }
}

fn help() {
    println!("Comandos disponiveis:");
    println!("\tList: \t\tlista os arquivos do servidor");
    println!("\tLs: \tlista os arquivos locais");
    println!("\tUpload: \tenviar um arquivo para o servidor");
    println!("\tDownload: \tbaixa um arquivo do servidor");
    println!("\tDelete: \tdelete um arquivo no servidor");
    println!("\tHelp: \t\tExibe este painel");
    println!("\tExit: \t\tfinaliza a sessão");
}

fn main() {
    std::process::Command::new("clear").status().unwrap();
    //TODO: adicionar ips diferentes
    //TODO: close quando server fecha
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8888") {
        read_server_string(&mut stream);
        help();
        loop {
            let mut cmd = String::new();
            let stdin = io::stdin();
            print!("Comando: ");
            let _ = stdout().flush();
            stdin.read_line(&mut cmd).unwrap();
            let mut cmd_list = cmd.split_whitespace();
            match cmd_list.next() {
                Some(cmd) => {
                    std::process::Command::new("clear").status().unwrap();
                    match cmd.to_lowercase().as_str() {
                        "ls" => {
                            println!("Arquivos locais:");
                            list_local();
                        },
                        "list" => {
                            println!("Arquivos no server:");
                            list(&mut stream);
                        },
                        "upload" => {
                            let file_name = cmd_list.next();
                            if let Some(file_name) = file_name {
                                upload(&mut stream, file_name);
                            }else{
                                eprintln!("Comando upload requer nome do arquivo");
                                eprintln!("Exemplo: upload file.txt")
                            }
                        },
                        "download" => {
                            let file_name = cmd_list.next();
                            if let Some(file_name) = file_name {
                                download(&mut stream, file_name);
                            }else{
                                eprintln!("Comando download requer nome do arquivo");
                                eprintln!("Exemplo: download file.txt")
                            }
                        },
                        "delete" => {
                            let file_name = cmd_list.next();
                            if let Some(file_name) = file_name {
                                delete(&mut stream, file_name);
                            }else{
                                eprintln!("Comando delete requer nome do arquivo");
                                eprintln!("Exemplo: delete file.txt")
                            }
                        },
                        "help" => {
                            help();
                        },
                        "exit" => {
                            break;
                        },
                        _ => {
                            eprintln!("Digite um comando válido")
                        }
                    }
                },
                _ => {
                    eprintln!("Digite um comando válido")
                },
            }
        }
    }
}

fn _main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8888") {
        println!("Connected to the server!");
        println!("Teste de listar!");
        let _ = stream.write_all(&[Commands::List as u8]);
        let _ = stream.flush();
        let string = read_string(&mut stream);
        println!("{string}");
        println!("Teste de excluir!");
        let _ = stream.write_all(&[Commands::Delete as u8]);
        let _ = stream.flush();
        let string = String::from("teste");
        send_string(&mut stream, &string);
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
        send_string(&mut stream, &string);

        let mut file = File::create(&string).unwrap();
        let mut size = read_size(&mut stream);
        println!("{size}");
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
                    println!("Falha ao receber arquivo!");
                    break;
                }
            }
            
        }

        println!("Teste de upload!");
        let _ = stream.write_all(&[Commands::Upload as u8]);
        let _ = stream.flush();
        let string = String::from("New_Infinity.apk");
        send_string(&mut stream, &string);
        let _ = stream.flush();
        
        let file = &fs::read(&string.trim()).unwrap();
        let size = file.len().to_le_bytes();
        let _ = stream.write_all(&size);
        let _ = stream.write_all(&file);
        let _ = stream.flush();
        let mut buf = Vec::new();
        if let Ok(n) = stream.read(&mut buf){
            let buf = &buf[0..n];
            let string = str::from_utf8(&buf).unwrap();
            println!("{string}");
        }
    } else {
        println!("Couldn't connect to server...");
    }
}
