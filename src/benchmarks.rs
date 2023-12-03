use std::{time::{Instant, Duration}, net::TcpStream, fs::File, io::Write, thread::{self, JoinHandle}, sync::{Arc, Mutex, }, path::Path};
use util::{upload, download, read_server_string, delete};
use rand::distributions::{Alphanumeric, DistString};

fn setup(ip: &str) -> TcpStream {
    let stream = TcpStream::connect(&ip).expect("Failed to connect to the server");
    stream
}

fn run_benchmark_for_2_params(f: fn(&mut TcpStream, &str, bool), param: &str, data: Arc<Mutex<Vec<u128>>>) {
    let now = Instant::now();

    let mut stream = setup("0.0.0.0:8888");
    read_server_string(&mut stream, false);
    f(&mut stream, param, false);
    
    let elapsed = now.elapsed();
    data.lock().unwrap().push(elapsed.as_micros());
}

fn upload_benchmark(id: u64, data: Arc<Mutex<Vec<u128>>>) {
    let path = format!("arq{}.txt", id);
    if !Path::new(&path).is_file() {
        let mut file = File::create(&path).unwrap();
        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 64*64*64);
        let _ = file.write_all(&string.as_bytes());
    }
    run_benchmark_for_2_params(upload, &path, data);
}

fn download_benchmark(id: u64, data: Arc<Mutex<Vec<u128>>>) {
    let name = format!("arq{}.txt", id);
    run_benchmark_for_2_params(download, &name, data);
}

fn delete_benchmark(id: u64, data: Arc<Mutex<Vec<u128>>>) {
    let name = format!("arq{}.txt", id);
    run_benchmark_for_2_params(delete, &name, data);
}

fn run_benchmark(count: u64, f: fn(u64, Arc<Mutex<Vec<u128>>>), data: Arc<Mutex<Vec<u128>>>, nome: &str) {
    let mut threads = Vec::<Result<JoinHandle<()>, ()>>::new();
    for i in 0..count {
        let data1 = data.clone();
        threads.push(thread::Builder::new().name(format!("{nome} - {count}")).spawn(move || {
            f(i, data1);
        }).map_err(|err| {
            eprintln!("Erro: {err}")
        }));
    }
    for thread in threads {
        if let Ok(thread) = thread{
            let _ = thread.join();
        }
    }
}

fn run_benchmark_print_average(f: fn(u64, Arc<Mutex<Vec<u128>>>), nome: &str, count: u64) {
    let data = Arc::new(Mutex::new(Vec::new()));
    let mut loop_count = 1;
    loop {
        run_benchmark(count, f, Arc::clone(&data), nome);
        std::process::Command::new("clear").status().unwrap();
        let lock = data.lock().unwrap();
        let iter = lock.iter();
        let sum: u128 = iter.clone().sum();
        let average = sum as f64 / (count * loop_count) as f64;
        println!("Média de {count} {nome}: {:.2} μs - max: {} μs - min: {} μs", average, iter.clone().max().unwrap(), iter.min().unwrap());
        loop_count += 1;
    }
}

fn main() {
    let count = 300;
    //run_benchmark_print_average(upload_benchmark, "upload", count);
    run_benchmark_print_average(download_benchmark, "download", count);
    //run_benchmark_print_average(delete_benchmark, "delete", count);

}