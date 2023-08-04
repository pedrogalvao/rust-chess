use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::controllers::controller::{Command, Controller};
use crate::model::game_state::GameState;

pub struct RemoteHuman {
    stream: TcpStream,
    // listener : TcpStream
}

impl RemoteHuman {
    pub fn new_listener() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
        let port = listener.local_addr().unwrap().port();
        println!("Listening on port {}", port);

        for stream in listener.incoming() {
            let rh = RemoteHuman {
                stream: stream.unwrap(),
            };
            println!("Connection established!");
            return rh;
        }
        panic!()
    }

    pub fn new_client(address: &str) -> Self {
        println!("try to connect to {}", address);
        if let Ok(stream) = TcpStream::connect(address) {
            let rh = RemoteHuman { stream };
            println!("Connection established!");
            return rh;
        }
        panic!()
    }

    pub fn receive_message(&mut self) -> String {
        let mut buffer = [0u8; 1024];
        let n = self
            .stream
            .read(&mut buffer)
            .expect("Failed to read message.");
        if n == 0 {
            println!("Peer disconnected.");
            return String::new();
        }
        return String::from_utf8_lossy(&buffer[..n]).to_string();
    }
}

impl Controller for RemoteHuman {
    fn choose_command(&mut self, game_state: &mut GameState) -> super::controller::Command {
        if let Some(last_move) = game_state.last_move.clone() {
            let send_msg = serde_json::to_string(&Command::Move(last_move)).unwrap();
            let _ = self.stream.write(send_msg.as_bytes());
        };
        let received_message = self.receive_message();

        let Ok(cmd) = serde_json::from_str(&received_message.as_str()) else {
            println!("Invalid move");
            return self.choose_command(game_state);
        };
        return cmd;
    }
}
