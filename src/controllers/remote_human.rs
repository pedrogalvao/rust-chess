use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::controllers::controller::{Command, Controller};
use crate::model::game_state::GameState;
use crate::model::piece::Color;

/// Controller for receiving commands from another device through TCP connection.
pub struct RemoteHuman {
    stream: TcpStream
}

const GET_COLOR : &str = "GET_COLOR";
const GET_STATE : &str = "GET_STATE";

impl RemoteHuman {

    /// Create controller and wait for connection
    pub fn new_listener() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
        let port = listener.local_addr().unwrap().port();
        println!("Listening on 127.0.0.1:{}", port);

        for stream in listener.incoming() {
            let rh = RemoteHuman {
                stream: stream.unwrap(),
            };
            println!("Connection established!");
            return rh;
        }
        panic!()
    }

    /// Create controller and connect to an existing host
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

    /// Ask what is the current game state
    pub fn get_game_state(&mut self) -> GameState {
        let _ = self.stream.write(GET_STATE.as_bytes());
        let response = self.receive_message();
        dbg!(&response);
        let Ok(game_state) = serde_json::from_str(response.as_str()) else {
            let response = self.receive_message();
            dbg!(&response);
            let Ok(game_state) = serde_json::from_str(response.as_str()) else {
                let response = self.receive_message();
                dbg!(&response);
                todo!()
            };
            return game_state;
        };
        return game_state;
    }

    /// Ask host what is the player's color
    pub fn get_color(&mut self) -> Color {
        let _ = self.stream.write(GET_COLOR.as_bytes());
        let response = self.receive_message();
        return serde_json::from_str(response.as_str()).unwrap();
    }

    /// Handle messages that don't contain movements
    pub fn handle_message(
        &mut self,
        received_message: String,
        game_state: &GameState,
        color: &Color,
    ) {
        if received_message == GET_STATE {
            let send_msg = serde_json::to_string(game_state).unwrap();
            let _ = self.stream.write(send_msg.as_bytes());
        } else if received_message == GET_COLOR {
            let send_msg = serde_json::to_string(color).unwrap();
            let _ = self.stream.write(send_msg.as_bytes());
        } else {
            println!("Invalid move");
        }
    }

    /// Handle initial requests for color and game state
    pub fn reply_to_initial_messages(&mut self, game_state: &GameState, color: &Color) {
        for _ in 0..2 {
            let received_message = self.receive_message();
            self.handle_message(received_message, game_state, &color);
        }
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
            self.handle_message(received_message, game_state, &game_state.player_to_move);
            return self.choose_command(game_state);
        };
        return cmd;
    }
}
