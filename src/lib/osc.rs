use rosc::{OscMessage, OscPacket, OscType};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, ToSocketAddrs, UdpSocket};

use super::error;
use super::settings::{REMOTE_IP, REMOTE_PORT};

pub fn handle_packet(packet: OscPacket, socket: &UdpSocket) -> Result<(), error::OscErrors> {
  println!("handle packet = {:?}", packet);
  match packet {
    OscPacket::Message(msg) => {
      let path: Vec<&str> = (msg.addr).trim_matches('/').split_terminator("/").collect();
      match path[0] {
        "osc-setup" => send_handshake_reply(socket).unwrap(),
        "osc-acknowledge" => match msg.args[0] {
          OscType::Int(_arg) => {
            println!("osc-acknowledge INT: {}", _arg)
          }
          _ => {
            // ("Unexpected type for argument 0: " );
            return Err(error::OscErrors::NoSetAction);
          }
        },
        _ => {
          println!("addr {:?}", msg.addr);
          // handle Blob here.
          return Err(error::OscErrors::NoSetAction);
        }
      }
    }
    _ => {}
  };

  Ok(())
}

fn send_handshake_reply(socket: &UdpSocket) -> Result<(), error::OscErrors> {
  let packet = OscPacket::Message(OscMessage {
    addr: "/osc-setup-reply".to_string(),
    args: vec![],
  });

  let _remote_ip = SocketAddr::from((
    [REMOTE_IP.0, REMOTE_IP.1, REMOTE_IP.2, REMOTE_IP.3],
    REMOTE_PORT,
  ));
  let mut __remote_ip = _remote_ip.to_socket_addrs().unwrap();

  match rosc::encoder::encode(&packet) {
    Ok(p) => match socket.send_to(&p, &__remote_ip.next().unwrap()) {
      // Ok(_p) => send_osc(&socket).unwrap(),
      Ok(_p) => {}
      Err(_e) => return Err(error::OscErrors::NoSetAction),
    },
    Err(e) => {
      return Err(error::OscErrors::UnrecognizedAddress(
        __remote_ip.next().unwrap().to_string(),
      ))
    }
  };

  Ok(())
}

fn send_osc(socket: &UdpSocket) -> Result<(), error::OscErrors> {
  let packet = OscPacket::Message(OscMessage {
    addr: "/osc-test".to_string(),
    args: vec![OscType::Int(112), OscType::Float(3.14)],
  });

  match rosc::encoder::encode(&packet) {
    // Ok(p) => socket.send_to(&p, "192.168.7.2:7562").unwrap(),
    Ok(p) => socket.send_to(&p, "127.0.0.1:7562").unwrap(),
    Err(e) => return Err(error::OscErrors::NoSetAction),
  };

  Ok(())
}
