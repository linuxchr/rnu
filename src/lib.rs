use std::io::{Error, ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
pub mod client;

pub fn listener(ip: String, port: u16) -> Result<TcpStream, Error> {
    let server = TcpListener::bind(format!("{ip}:{port}"));
    loop {
        for stream in server.as_ref().expect("Failed!").incoming() {
            match stream {
                Ok(stream) => {
                    println!("New Connection: {}", stream.peer_addr()?);
                    return Ok(stream);
                }
                Err(e) => {
                    Err(e)?;
                }
            }
        }
    }
}

type Rsljh = std::thread::JoinHandle<Result<(), Error>>;

fn pipe_thread<R, W>(mut r: R, mut w: W) -> Rsljh
where
    R: std::io::Read + Send + 'static,
    W: std::io::Write + Send + 'static,
{
    std::thread::spawn(move || {
        let mut buffer = [0; 1024];
        loop {
            match r.read(&mut buffer) {
                Ok(len) => {
                    if len == 0 {
                        break;
                    }
                    if let Err(e) = w.write_all(&buffer[..len]) {
                        return Err(e);
                    }
                    if let Err(e) = w.flush() {
                        return Err(e);
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(())
    })
}

pub fn rsl(port: u16) -> Result<(), Error> {
    let stream: TcpStream = listener("0.0.0.0".to_string(), port)?;
    let t1 = pipe_thread(std::io::stdin(), stream.try_clone()?);
    let t2 = pipe_thread(stream, std::io::stdout());
    match t1.join() {
        Ok(_) => {}
        Err(_) => return Err(Error::new(ErrorKind::Other, "Failed!")),
    }
    match t2.join() {
        Ok(_) => {}
        Err(_) => return Err(Error::new(ErrorKind::Other, "Failed!")),
    }
    Ok(())
}

pub fn sender(mut stream: &TcpStream, msg: &String) -> Result<(), Error> {
    let buf: &[u8] = msg.as_bytes();
    stream.write(buf)?;
    Ok(())
}

pub fn sender_bytes(mut stream: &TcpStream, buf: &[u8]) -> Result<(), Error> {
    stream.write(buf)?;
    Ok(())
}
