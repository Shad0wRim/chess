use std::io::{self, prelude::*};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
const SERVER_ADDRESS: &str = "127.0.0.1:7878";

fn main() -> io::Result<()> {
    let mut players = PlayerListener::new(SERVER_ADDRESS)?;
    players.accept()?;
    loop {
        players.check_connection()?;
        thread::sleep(Duration::from_secs(5));
    }
}

struct PlayerListener {
    listener: TcpListener,
    player1: Option<TcpStream>,
    player2: Option<TcpStream>,
}

impl PlayerListener {
    /// Initializes the server on the specified address
    ///
    /// # Errors
    ///
    /// returns any io error when binding to the address
    fn new(addr: &str) -> io::Result<PlayerListener> {
        let listener = TcpListener::bind(addr)?;
        // listener.set_nonblocking(true)?;
        Ok(PlayerListener {
            listener,
            player1: None,
            player2: None,
        })
    }
    /// Accepts two connections and stores them in the PlayerListener
    ///
    /// Blocks until both connections are established
    ///
    /// # Errors
    ///
    /// returns any io errors when accepting and doing an initial write to the client
    fn accept(&mut self) -> io::Result<()> {
        println!("Waiting for player 1");
        self.player1 = Some(self.listener.accept()?.0);
        self.player1
            .as_mut()
            .unwrap()
            .write_all(b"Waiting for second player...\n")?;
        println!("Waiting for player 2");
        self.player2 = Some(self.listener.accept()?.0);
        self.player2
            .as_mut()
            .unwrap()
            .write_all(b"Connected to game!\n")?;
        println!("Fully connected");
        Ok(())
    }
    /// This function checks the connection status of both players.
    ///
    /// # Errors
    ///
    /// If the players are unitialized (None) this function returns io::ErrorKind::Other,
    /// otherwise this function returns the the io error from failing to read from the client.
    ///
    /// If this function encounters an error, it will close the connection.
    fn check_connection(&mut self) -> io::Result<()> {
        if self.player1.is_none() || self.player2.is_none() {
            return Err(io::ErrorKind::Other.into());
        }

        match self.player1.as_mut().unwrap().write_all(b"HEARTBEAT\n") {
            Ok(_) => (),
            Err(e) => {
                self.player1.take();
                return Err(e);
            }
        }
        match self.player2.as_mut().unwrap().write_all(b"HEARTBEAT\n") {
            Ok(_) => (),
            Err(e) => {
                self.player2.take();
                return Err(e);
            }
        }

        Ok(())
    }
}
