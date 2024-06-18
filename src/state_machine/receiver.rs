use std::sync::Arc;
use crate::network::udp::UdpSocketWrapper;
use crate::state_machine::common::{Message, State};


pub struct Receiver<CurrentState: State> {
    socket: Arc<UdpSocketWrapper>,
    state: CurrentState,
}

// impl<CurrentState: State> Clone for Receiver<CurrentState> {
//     fn clone(&self) -> Self {
//         Receiver {
//             socket: Arc::clone(&self.socket),
//             state: self.state.clone(),
//         }
//     }
// }

pub struct Idle;
pub struct Receiving {
    pub message: Message,
}
pub struct SendingAck;
pub struct SendingNack;

impl State for Idle {}
impl State for Receiving {}
impl State for SendingAck {}
impl State for SendingNack {}

impl Receiver<Idle> {
    pub fn new(socket: Arc<UdpSocketWrapper>) -> Self {
        Receiver {
            socket,
            state: Idle,
        }
    }

    pub fn receive(self) -> Result<Receiver<Receiving>, std::io::Error> {
        let message = self.socket.receive_message()?;
        Ok(Receiver {
            socket: self.socket,
            state: Receiving { message },
        })
    }
}

impl Receiver<Receiving> {
    pub fn process_message(self) -> Result<Receiver<SendingAck>, Receiver<SendingNack>> {
        if self.state.message.is_valid() {
            Ok(Receiver {
                socket: self.socket,
                state: SendingAck,
            })
        } else {
            Err(Receiver {
                socket: self.socket,
                state: SendingNack,
            })
        }
    }

    pub fn get_message(&self) -> &Message {
        &self.state.message
    }
}

impl Receiver<SendingAck> {
    pub fn send_ack(self) -> Receiver<Idle> {
        self.socket.send_ack();
        Receiver {
            socket: self.socket,
            state: Idle,
        }
    }

}

impl Receiver<SendingNack> {
    pub fn send_nack(self) -> Receiver<Idle> {
        self.socket.send_nack();
        Receiver {
            socket: self.socket,
            state: Idle,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::UdpSocket;

    #[test]
    fn test_receiverstate_transitions() {
        let socket1 = UdpSocket::bind("127.0.0.1:0").unwrap();
        let socket2 = UdpSocket::bind("127.0.0.1:0").unwrap();
        let addr1 = socket1.local_addr().unwrap();

        let udp_wrapper1 = Arc::new(UdpSocketWrapper { socket: socket1 });
        let udp_wrapper2 = UdpSocketWrapper { socket: socket2 };

        udp_wrapper2.socket.connect(addr1).unwrap();
        udp_wrapper1.socket.connect(udp_wrapper2.socket.local_addr().unwrap()).unwrap();

        let receiver = Receiver::new(udp_wrapper1.clone());

        let message = Message::Data { sequence: 1, payload: vec![1, 2, 3] };
        udp_wrapper2.send_message(&message);  // Simulate the sender sending a DATA message

        let receiver = receiver.receive().expect("Failed to receive message");
        assert!(matches!(receiver.state, Receiving { .. }));

        let receiver = receiver.process_message();
        match receiver {
            Ok(receiver) => assert!(matches!(receiver.state, SendingAck)),
            Err(receiver) => assert!(matches!(receiver.state, SendingNack)),
        }
    }
}

