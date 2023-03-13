use std::io::{BufRead, BufReader, Error, ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;

pub fn listener(ip: String, port: i16) -> Result<TcpStream, Error> {
    let server = TcpListener::bind(format!("{}:{}", ip, port));
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

/*
pub fn reverse_shell_listener(){
    let mut counter: i8 = 0;
    let inc: TcpStream = listener("0.0.0.0".to_string(), 23234);
    let inc_clone = match inc.try_clone() {
        Ok(c) => c,
        Err(_) => return
    };
    let t1 = if let Some(t) = pipe_thread(std::io::stdin(), inc_clone) {
        counter += 1;
        t
    } else {
        return
    };
    let t2 = match pipe_thread(inc, std::io::stdout()){
        Some(t) => {
            counter += 1;
            t
        },
        None => return
    };
    if counter == 2{
        match t1.join() {
            Ok(_) => {},
            Err(_) => return
        }
        match t2.join() {
            Ok(_) => {},
            Err(_) => return
        }
    }
}*/
pub fn rsl() -> Result<(), Error>{
    let stream: TcpStream = listener("0.0.0.0".to_string(), 23234)?;
    let t1 = pipe_thread(std::io::stdin(), stream.try_clone()?);
    let t2 = pipe_thread(stream, std::io::stdout());
    match t1.join() {
        Ok(_) => {},
        Err(_) => return Err(Error::new(ErrorKind::Other, "Failed!"))
    }
    match t2.join() {
        Ok(_) => {},
        Err(_) => return Err(Error::new(ErrorKind::Other, "Failed!"))
    }
    Ok(())
}

/* 
fn run_rsl_threads(t1: Rsljh, t2: Rsljh) -> Result<(), Error>{
    t1.join()?;
    t2.join()?;
    Ok(())
}
*/
pub fn reader(stream: &TcpStream) -> Result<String, Error> {
    let mut msg: BufReader<&TcpStream> = BufReader::new(&stream);
    let mut buffer: Vec<u8> = Vec::new();
    msg.read_until(b'\n', &mut buffer)?;
    let output: &str = match from_utf8(&buffer) {
        Ok(it) => it,
        Err(err) => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Buffer invalid: {}", err),
            ))
        }
    };
    Ok(output.to_string())
}

pub fn sender(mut stream: &TcpStream, msg: &String) -> Result<(), Error> {
    let buf: &[u8] = msg.as_bytes();
    stream.write(buf)?;
    Ok(())
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}*/
