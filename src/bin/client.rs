use std::io::{self, prelude::*, BufReader};
use std::net::{TcpStream, ToSocketAddrs};
const SERVER_ADDRESS: &str = "127.0.0.1:7878";

fn main() -> io::Result<()> {
    let mut player = Player::new(SERVER_ADDRESS)?;
    // let mut buf = String::new();
    // io::stdin().read_line(&mut buf)?;
    // player.send_line(&buf)?;
    // buf.clear();
    loop {
        let buf = player.read_line()?;
        print!("{buf}");
    }
}

struct Player {
    connection: TcpStream,
}

impl Player {
    fn new(addr: impl ToSocketAddrs) -> io::Result<Player> {
        let stream = TcpStream::connect(addr)?;
        Ok(Player { connection: stream })
    }
    fn read_line(&mut self) -> io::Result<String> {
        let mut reader = BufReader::new(&mut self.connection);
        let mut buf = String::new();
        reader.read_line(&mut buf)?;
        Ok(buf)
    }
    fn send_line(&mut self, message: &str) -> io::Result<()> {
        let message = if !message.ends_with('\n') {
            message.to_owned() + "\n"
        } else {
            message.to_owned()
        };
        self.connection.write_all(message.as_bytes())
    }
}
