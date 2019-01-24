/*
Реализовать на языке Rust tcp клиент/сервер (общее приложение, режим определяется параметрами),
способное установить сокет соединение и последовательно обмениваться сообщениями. На каждой
стороне реализуется алгоритм переменных ключей (смотри реализацию на Python - protector.py). На
каждом шаге обмена вычисляется следующий ключ и сравнивается с полученным от второй стороны.

Шаг 1. Установление соединения. Клиент подключается к серверу и передает стартовую строку и первый
ключ

Шаг 2. Сервер на основе строки и ключа генерирует новый ключ и отдает его клиенту

Шаг 3. Клиент сравнивает полученный ключ со следующим ключом, и, если все успешно, создает новый
ключ и отправляет следующее сообщение на сервер.

Шаг 4..10 - аналогично

На каждом шагу приложение должно выводить в консоли текущий статус, текущий ключ и
отправленное/полученное сообщение.

По желанию можно дополнить код функцией чата и вводить попутное сообщение/ответ из консоли.

 При запуске программа должна принимать два параметра командной строки:

 1) порт - режим сервера или ip:port - режим клиента
 2) -n 100 - кол-во одновременных подключений

protector.py protector.py
*/

extern crate clap;

use clap::{Arg, App};

use thread_pool::ThreadPool;
use protector::SessionProtector;

use std::io::prelude::*;
use std::io::{self, BufRead, BufReader, Read, Write, Error};
use std::net::{TcpStream, TcpListener, IpAddr};
use std::fs;
use std::thread;
use std::process;
use std::time::Duration;
use std::str;
use std::str::FromStr;
use std::result::Result;

fn main() {
    test_session_protector();
    /*
     * Parse command line arguments
     */
    //
    // let matches = App::new("CSD-ProgLang-2018-4")
    //     .about("Protected client/server application on rust")
    //     .arg(Arg::with_name("[ip]:port")
    //              .takes_value(true)
    //              .required(true)
    //              .help("port - server mode, ip:port - client mode"))
    //     .arg(Arg::with_name("connection_number")
    //              .short("n")
    //              .takes_value(true)
    //              .help("simultaneous connections number"))
    //     .get_matches();
    //
    // let ip_port_str = matches.value_of("[ip]:port").unwrap();
    // let ip_port: Vec<&str> = ip_port_str.split(":").collect();
    // if ip_port.len() == 1 { // server mode
    //     let parsed_port = ip_port[0].parse::<i32>();
    //     match parsed_port {
    //         Ok(n) => println!("The port parsed is: {}", n),
    //         Err(e) =>  { println!("Error parsing port: {:?}", e); process::exit(0); }
    //     }
    //
    //     let num_str = matches.value_of("connection_number").unwrap_or("100");
    //     let parsed_connections_number = num_str.parse::<usize>();
    //     let mut connections_number: usize = 1;
    //     match parsed_connections_number {
    //         Ok(n) => { println!("simultaneous connections number {}.", n); connections_number = n; }
    //         Err(e) => println!("That's not a number! {}", e),
    //     }
    //
    //     // for arguments use str variables, asserted by ip & i32 parsers
    //     start_server(ip_port_str, connections_number);
    //
    // } else if ip_port.len() == 2 { // client mode
    //     let parsed_ip = IpAddr::from_str(ip_port[0]);
    //     match parsed_ip {
    //         Ok(p) => println!("The ip parsed is: {}", p),
    //         Err(e) => { println!("Error parsing ip: {:?}", e); process::exit(0); }
    //     }
    //     let port = ip_port[1].parse::<i32>();
    //     match port {
    //         Ok(n) => println!("The port parsed is: {}", n),
    //         Err(e) =>  { println!("Error parsing port: {:?}", e); process::exit(0); }
    //     }
    //
    //     connect_to_server(ip_port_str);
    // } else {
    //     println!("Incorrect command-line arguments input!")
    // }
}

fn test_session_protector() {
    let session_key = protector::get_session_key();
    println!("Session Key: {}", session_key);

    let hash_str = protector::get_hash_str();
    println!("Hash Str: {}", hash_str);
}

fn start_server(port: &str, connections_number: usize) {
    let socket_addr = "127.0.0.1:".to_owned() + port;
    let listener = TcpListener::bind(socket_addr).unwrap();
    let pool = ThreadPool::new(connections_number);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            // handle_connection(stream);
            handle_client(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    let message = stream.read(&mut buffer).unwrap();

    println!("Buffer: {:?}", str::from_utf8(&buffer));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}


// NEW VERSION
//
fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    loop {
        let mut buf: Vec<u8> = Vec::new();
        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\n', &mut buf);
        // let bytes_read = stream.read(b'\n', &mut buf)?;
        println!("Client: {:?}", str::from_utf8(&buf).unwrap());
        // if bytes_read == 0 { return Ok(()) }
        // stream.write(&buf[..bytes_read])?;
        stream.write(&buf).unwrap();
        stream.flush().unwrap();
    }
}

fn connect_to_server(ip_port: &str) {
    let mut stream = TcpStream::connect(ip_port).unwrap();
    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        io::stdin().read_line(&mut input);
        stream.write(input.as_bytes());
        stream.flush().unwrap();

        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n', &mut buffer);
        print!("Server: {:?}", str::from_utf8(&buffer).unwrap());
    }
}

// END NEW VERSION
