# UDPPP (UDP++)

## Overview

**UDPPP** (UDP++) is a simple, educational protocol layer over UDP designed to demonstrate how to use the typestate pattern to represent state machines in Rust. This project aims to teach students about protocol design, state machines, and the benefits of compile-time checks using Rust's type system.

## Architecture

### Protocol Design

The protocol facilitates reliable, bidirectional message transfer using UDP, implementing acknowledgment and retransmission mechanisms. The key components are:

- **DATA**: Contains the actual message payload.
- **ACK**: Acknowledgment for the received DATA message.
- **NACK** (Optional): Negative acknowledgment indicating a problem with the received DATA message.

### State Machines

The protocol uses state machines for both the sender and receiver, ensuring correct state transitions and handling using the typestate pattern.

#### Sender States

1. **Idle**: Waiting to send a message.
2. **Sending**: Sending the DATA message.
3. **AwaitingAck**: Waiting for an acknowledgment (ACK) from the receiver.
4. **Resend**: Resending the DATA message after a timeout or upon receiving NACK.

#### Receiver States

1. **Idle**: Waiting to receive a message.
2. **Receiving**: Receiving a DATA message.
3. **SendingAck**: Sending an ACK message back to the sender.
4. **SendingNack**: (Optional) Sending a NACK message if there was an issue with the DATA message.

### Implementation

The implementation is divided into modules for state machines, network operations, and utilities. Key modules include:

- **state_machine**: Contains the sender and receiver state machines.
- **network**: Handles UDP operations.
- **utils**: Provides utility functions like checksums for data integrity.

## Educational Goal

The primary educational goal is to demonstrate the typestate pattern in Rust, showcasing how compile-time checks can enforce correct state transitions and prevent common errors in protocol implementation.

## Example: FizzBuzz

An example that showcases both sender and receiver on different threads is provided in the `examples` directory.

To run the example, use:

```sh
cargo run --example fizzbuzz