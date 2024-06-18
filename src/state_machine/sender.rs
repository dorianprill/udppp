use std::time::Duration;
use std::sync::Arc;
use crate::network::udp::UdpSocketWrapper;
use crate::state_machine::common::{Message, State};

/// Sender State Machine
/// Works with an Arc<UdpSocketWrapper> so that Sender and Receiver 
/// can share the same socket, even between threads
pub struct Sender<CurrentState: State> {
    socket: Arc<UdpSocketWrapper>,
    _state: CurrentState,
}

pub struct Idle;
pub struct Sending;
pub struct AwaitingAck;
pub struct Resend;

impl State for Idle {}
impl State for Sending {}
impl State for AwaitingAck {}
impl State for Resend {}


impl Sender<Idle> {
    pub fn new(socket: Arc<UdpSocketWrapper>) -> Self {
        Sender {
            socket,
            _state: Idle,
        }
    }

    pub fn send(self, message: Message) -> Sender<Sending> {
        self.socket.send_message(&message);
        Sender {
            socket: self.socket,
            _state: Sending,
        }
    }
}

impl Sender<Sending> {
    pub fn await_ack(self, timeout: Duration) -> Result<Sender<AwaitingAck>, Sender<Resend>> {
        if self.socket.receive_ack(timeout) {
            Ok(Sender {
                socket: self.socket,
                _state: AwaitingAck,
            })
        } else {
            Err(Sender {
                socket: self.socket,
                _state: Resend,
            })
        }
    }
}

impl Sender<AwaitingAck> {
    pub fn on_ack_received(self) -> Sender<Idle> {
        Sender {
            socket: self.socket,
            _state: Idle,
        }
    }
}

impl Sender<Resend> {
    pub fn resend(self, message: Message) -> Sender<Sending> {
        self.socket.send_message(&message);
        Sender {
            socket: self.socket,
            _state: Sending,
        }
    }
}

impl<CurrentState: State> Sender<CurrentState> {
    pub fn get_socket(&self) -> Arc<UdpSocketWrapper> {
        Arc::clone(&self.socket)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::net::UdpSocket;

    #[test]
    fn test_sender_state_transitions() {
        let socket1 = UdpSocket::bind("127.0.0.1:0").unwrap();
        let socket2 = UdpSocket::bind("127.0.0.1:0").unwrap();
        let addr2 = socket2.local_addr().unwrap();

        let udp_wrapper1 = UdpSocketWrapper { socket: socket1 };
        let udp_wrapper2 = UdpSocketWrapper { socket: socket2 };

        udp_wrapper1.socket.connect(addr2).unwrap();
        udp_wrapper2.socket.connect(udp_wrapper1.socket.local_addr().unwrap()).unwrap();

        let sender = Sender::new(Arc::new(udp_wrapper1));
        let message = Message::Data { sequence: 1, payload: vec![1, 2, 3] };

        let sender = sender.send(message);
        assert!(matches!(sender._state, Sending));

        udp_wrapper2.send_ack();  // Simulate the receiver sending an ACK

        let sender = sender.await_ack(Duration::from_secs(1));
        match sender {
            Ok(sender) => assert!(matches!(sender._state, AwaitingAck)),
            Err(sender) => assert!(matches!(sender._state, Resend)),
        }
    }
}
