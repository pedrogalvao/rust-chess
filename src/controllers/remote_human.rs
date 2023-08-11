use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::controllers::controller::{Command, Controller};
use crate::menu::accept_undo_menu;
use crate::model::game_state::GameState;
use crate::model::piece::Color;

/// Controller for receiving commands from another device through TCP connection.
pub struct RemoteHuman {
    stream: TcpStream,
    pub color: Color,
    dont_send_last_move: bool,
    undo_accepted: bool
}

const GET_COLOR: &str = "GET_COLOR";
const GET_STATE: &str = "GET_STATE";
const UNDO_MSG: &str = "UNDO_MSG";
const ACCEPT_UNDO: &str = "ACCEPT_UNDO";

impl RemoteHuman {
    /// Create controller and wait for connection
    pub fn new_listener(color: Color) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
        let port = listener.local_addr().unwrap().port();
        println!("Listening on 127.0.0.1:{}", port);

        for stream in listener.incoming() {
            println!("Listening on 127.0.0.1:{}", port);
            let rh = RemoteHuman {
                color,
                stream: stream.unwrap(),
                dont_send_last_move: true,
                undo_accepted: false
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
            let mut rh = RemoteHuman { color: Color::White, stream ,
                dont_send_last_move: true,
                undo_accepted: false};
            println!("Connection established!");
            rh.color = rh.get_color().get_opponent_color();
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
        let Ok(game_state) = serde_json::from_str::<GameState>(response.as_str()) else {
            todo!()
        };
        if game_state.player_to_move == self.color {
            self.dont_send_last_move = true;
        } else {
            self.dont_send_last_move = false;
        }
        return game_state;
    }

    /// Ask host what is the player's color
    fn get_color(&mut self) -> Color {
        let _ = self.stream.write(GET_COLOR.as_bytes());
        let response = self.receive_message();
        let opponent_color: Color = serde_json::from_str(response.as_str()).unwrap();
        return opponent_color;
    }

    /// Handle messages that don't contain movements
    pub fn handle_message(
        &mut self,
        received_message: String,
        game_state: &GameState,
    ) {
        if received_message == GET_STATE {
            if game_state.player_to_move == self.color {
                self.dont_send_last_move = true;
            } else {
                self.dont_send_last_move = false;
            }
            let send_msg = serde_json::to_string(game_state).unwrap();
            let _ = self.stream.write(send_msg.as_bytes());
        } else if received_message == GET_COLOR {
            let send_msg = serde_json::to_string(&self.color).unwrap();
            let _ = self.stream.write(send_msg.as_bytes());
        } else if received_message == UNDO_MSG {
            if accept_undo_menu() {
                self.undo_accepted = true;
                let _ = self.stream.write(ACCEPT_UNDO.as_bytes());
            } else {
                self.dont_send_last_move = true; // the opponent already has the previous move, dont send it again
                let _ = self.stream.write("no".as_bytes());
            }
        } else {
            println!("Invalid move");
        }
    }

    /// Handle initial requests for color and game state
    pub fn reply_to_initial_messages(&mut self, game_state: &GameState) {
        for _ in 0..2 {
            let received_message = self.receive_message();
            self.handle_message(received_message, game_state);
        }
    }
}

impl Controller for RemoteHuman {
    
    fn accept_undo(&mut self) -> bool {
        let _ = self.stream.write(UNDO_MSG.as_bytes());
        let reply = self.receive_message();
        if reply == ACCEPT_UNDO {
            return true;
        } else {
            return false;
        }
    }

    fn choose_command(&mut self, game_state: &mut GameState) -> super::controller::Command {
        if self.undo_accepted {
            self.undo_accepted = false;
            self.dont_send_last_move = true; // the opponent already has the previous move, dont send it again
            return Command::AcceptUndo;
        } else if self.dont_send_last_move {
            self.dont_send_last_move = false;
        } else if let Some(last_move) = game_state.last_move.clone() {
            if game_state.player_to_move == self.color { 
                let send_msg = serde_json::to_string(&Command::Move(last_move)).unwrap();
                let _ = self.stream.write(send_msg.as_bytes());
            }
        };
        let received_message = self.receive_message();

        let Ok(cmd) = serde_json::from_str(&received_message.as_str()) else {
            self.handle_message(received_message, game_state);
            return self.choose_command(game_state);
        };
        return cmd;
    }
}
