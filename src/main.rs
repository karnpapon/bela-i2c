// instructions
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

// modules
use crate::lib::settings::{LOCAL_IP, LOCAL_PORT, REMOTE_IP, REMOTE_PORT};
use rosc::OscPacket;
use rosc::OscType;
use std::error::Error;
use std::fmt;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::{thread, time};

mod lib;
use lib::{error, osc};

fn main() {
  greetings();

  // udp
  // let addr = SocketAddrV4::new(
  //   Ipv4Addr::new(LOCAL_IP.0, LOCAL_IP.1, LOCAL_IP.2, LOCAL_IP.3),
  //   LOCAL_PORT,
  // );
  let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), LOCAL_PORT);
  let socket = UdpSocket::bind(addr).expect("couldn't bind to address");
  println!("Listening on port {} / full addr: {}", addr.port(), addr);

  // midi : midi.rs
  // let midi_in = midi::create_midi_in().unwrap();
  // let mut midi_out = midi::create_midi_out().unwrap();

  // osc : osc.rs
  let mut buf = [0u8; rosc::decoder::MTU];
  loop {
    match socket.recv_from(&mut buf) {
      Ok((size, addr)) => {
        println!("Received packet with size {} from: {}", size, addr);
        let packet = rosc::decoder::decode(&buf[..size]).unwrap();
        osc::handle_packet(packet, &socket).unwrap();
      }
      Err(e) => {
        println!("Error receiving from socket: {}", e);
        break;
      }
    }
  }
}

fn greetings() {
  println!("hellooooooo");
}
