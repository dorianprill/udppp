use std::net::UdpSocket;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use udppp::state_machine::common::Message;
use udppp::state_machine::sender::{Sender, Idle as SenderIdle};
use udppp::state_machine::receiver::{Receiver, Idle as ReceiverIdle};
use udppp::network::udp::UdpSocketWrapper;

fn main() {
    let sender_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let receiver_socket = UdpSocket::bind("127.0.0.1:0").unwrap();

    let sender_addr = sender_socket.local_addr().unwrap();
    let receiver_addr = receiver_socket.local_addr().unwrap();

    sender_socket.connect(receiver_addr).unwrap();
    receiver_socket.connect(sender_addr).unwrap();

    let sender_thread = thread::spawn(move || {
        let udp_wrapper = UdpSocketWrapper { socket: sender_socket };
        let mut sender = Sender::<SenderIdle>::new(Arc::new(udp_wrapper));

        for i in 1..=100 {
            let message = if i % 15 == 0 {
                Message::Data { sequence: i, payload: b"FizzBuzz".to_vec() }
            } else if i % 3 == 0 {
                Message::Data { sequence: i, payload: b"Fizz".to_vec() }
            } else if i % 5 == 0 {
                Message::Data { sequence: i, payload: b"Buzz".to_vec() }
            } else {
                Message::Data { sequence: i, payload: i.to_string().as_bytes().to_vec() }
            };

            println!("Sender: Sending message {}", i);
            let mut attempt = sender.send(message.clone());
            loop {
                match attempt.await_ack(Duration::from_secs(1)) {
                    Ok(s) => {
                        sender = s.on_ack_received();
                        println!("Sender: Received ACK for message {}", i);
                        break;
                    }
                    Err(s) => {
                        println!("Sender: Resending message {}", i);
                        attempt = s.resend(message.clone());
                    }
                }
            }
        }
    });

    let receiver_thread = thread::spawn(move || {
        let udp_wrapper = UdpSocketWrapper { socket: receiver_socket };
        let mut receiver = Receiver::<ReceiverIdle>::new(Arc::new(udp_wrapper));

        for _ in 1..=100 {
            let received = receiver.receive().expect("Failed to receive message");
            if let Message::Data { sequence, payload } = received.get_message() {
                println!("Receiver: Received message {} with payload: {}", sequence, String::from_utf8_lossy(&payload));
            }
            receiver = match received.process_message() {
                Ok(r) => {
                    println!("Receiver: Sending ACK for message");
                    r.send_ack()
                }
                Err(r) => {
                    println!("Receiver: Sending NACK for message");
                    r.send_nack()
                }
            };
        }
    });

    sender_thread.join().unwrap();
    receiver_thread.join().unwrap();
}
