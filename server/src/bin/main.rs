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

use server::ThreadPool;

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

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
