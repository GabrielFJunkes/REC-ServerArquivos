use std::{net::TcpStream, io::{Write, self, stdout}, fs};
use util::{read_server_string, list, upload, download, delete};


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
    println!("\tLs: \t\tlista os arquivos locais");
    println!("\tUpload: \tenviar um arquivo para o servidor");
    println!("\tDownload: \tbaixa um arquivo do servidor");
    println!("\tDelete: \tdelete um arquivo no servidor");
    println!("\tHelp: \t\tExibe este painel");
    println!("\tExit: \t\tfinaliza a sessão");
}

fn main() {
    std::process::Command::new("clear").status().unwrap();
    print!("Ip: ");
    let stdin = io::stdin();
    let mut ip = String::new();
    let _ = stdout().flush();
    stdin.read_line(&mut ip).unwrap();
    let ip = ip.trim();
    
    match TcpStream::connect(&ip) {
        Ok(mut stream) =>  {
            std::process::Command::new("clear").status().unwrap();
            read_server_string(&mut stream, true);
            help();
            loop {
                let mut cmd = String::new();
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
                                list(&mut stream);
                            },
                            "upload" => {
                                let file_name = cmd_list.next();
                                if let Some(file_name) = file_name {
                                    upload(&mut stream, file_name, true);
                                }else{
                                    eprintln!("Comando upload requer nome do arquivo");
                                    eprintln!("Exemplo: upload file.txt")
                                }
                            },
                            "download" => {
                                let file_name = cmd_list.next();
                                if let Some(file_name) = file_name {
                                    download(&mut stream, file_name, true);
                                }else{
                                    eprintln!("Comando download requer nome do arquivo");
                                    eprintln!("Exemplo: download file.txt")
                                }
                            },
                            "delete" => {
                                let file_name = cmd_list.next();
                                if let Some(file_name) = file_name {
                                    delete(&mut stream, file_name, true);
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
        },
        Err(err) => {
            eprintln!("Falha ao conectar ao servidor {ip}");
            eprintln!("Erro: {err}");
        }
    }
}