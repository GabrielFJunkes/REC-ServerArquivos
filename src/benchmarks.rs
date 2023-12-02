use std::{time::Instant, net::TcpStream, fs::File, io::Write, thread};
use util::{read_server_string, list, upload, download, delete};
use rand::distributions::{Alphanumeric, DistString};

fn setup(ip: &str) -> TcpStream {
    match TcpStream::connect(&ip) {
        Ok(stream) =>  {stream}
        Err(err) => {
            panic!("Erro: {err}")
        }
    }
}

fn run_benchmark_for_2_params(f: fn(&mut TcpStream, &str, bool), param2: &str) {
    let now = Instant::now();

    let mut stream = setup("0.0.0.0:8888");
    f(&mut stream, param2, false);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn run_benchmark_for_1_params(f: fn(&mut TcpStream), param1: &mut TcpStream) {
    let now = Instant::now();
    f(param1);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn upload_benchmark(id: u64) {
    let path = format!("arq{}.txt", id);
    let mut file = File::create(&path).unwrap();
    let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 64);
    let _ = file.write_all(&string.as_bytes());

    run_benchmark_for_2_params(upload, &path);
}

fn download_benchmark() {
    let name = "testeUpload.txt";
    run_benchmark_for_2_params(download, &name);
}

fn main() {
    download_benchmark()
    //for i in 0..100 {
    //    let _ = thread::Builder::new().name(i.to_string()).spawn(move || {
    //        upload_benchmark(i);
    //    }).map_err(|err| {
    //        eprintln!("Erro: {err}")
    //    });
    //}
}