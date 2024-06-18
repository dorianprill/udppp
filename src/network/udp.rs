use std::net::UdpSocket;
use std::time::Duration;
use bincode;

use crate::state_machine::common::Message;

pub struct UdpSocketWrapper {
    pub socket: UdpSocket,
}

impl UdpSocketWrapper {
    pub fn send_message(&self, message: &Message) {
        let data = bincode::serialize(message).expect("Failed to serialize message");
        match self.socket.send(&data) {
            Ok(_) => println!("Sent message: {:?}", message),
            Err(e) => println!("Failed to send message: {:?}", e),
        }
    }

    pub fn receive_message(&self) -> Result<Message, std::io::Error> {
        let mut buf = [0; 1024];
        match self.socket.recv_from(&mut buf) {
            Ok((amt, _)) => {
                let message: Message = bincode::deserialize(&buf[..amt]).expect("Failed to deserialize message");
                println!("Received message: {:?}", message);
                Ok(message)
            }
            Err(e) => Err(e),
        }
    }

    pub fn send_ack(&self) {
        let ack = Message::Ack;
        self.send_message(&ack);
    }

    pub fn send_nack(&self) {
        let nack = Message::Nack;
        self.send_message(&nack);
    }

    pub fn receive_ack(&self, timeout: Duration) -> bool {
        self.set_read_timeout(Some(timeout));
        match self.receive_message() {
            Ok(Message::Ack) => true,
            _ => false,
        }
    }

    pub fn set_read_timeout(&self, duration: Option<Duration>) {
        self.socket.set_read_timeout(duration).expect("Failed to set read timeout");
    }
}

