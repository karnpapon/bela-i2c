use rosc::{OscMessage, OscPacket, OscType};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, ToSocketAddrs, UdpSocket};

use super::error;
use crate::lib::eurorack::*;
use super::eurorack;
use super::settings::{REMOTE_IP, REMOTE_PORT};

pub fn handle_packet(packet: OscPacket, socket: &UdpSocket) -> Result<(), error::OscErrors> {
    if let OscPacket::Message(msg) = packet {
        let path: Vec<&str> = (msg.addr).trim_matches('/').split_terminator('/').collect();
        println!("path ---> {:?}", &path);
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
            "er301" | "Er301" => route_eurorack_module(&msg, path),
            _ => {
                println!("addr {:?}", msg.addr);
                // handle Blob here.
                return Err(error::OscErrors::NoSetAction);
            }
        }
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
        args: vec![OscType::Int(112), OscType::Float(3.4)],
    });

    let _remote_ip = SocketAddr::from((
        [REMOTE_IP.0, REMOTE_IP.1, REMOTE_IP.2, REMOTE_IP.3],
        REMOTE_PORT,
    ));
    let mut __remote_ip = _remote_ip.to_socket_addrs().unwrap();

    match rosc::encoder::encode(&packet) {
        Ok(p) => socket.send_to(&p, __remote_ip.next().unwrap()).unwrap(),
        Err(e) => return Err(error::OscErrors::NoSetAction),
    };

    Ok(())
}

//fn handle_osc_ii(msg: &str) -> Result<(), error::OscErrors>{
//    println!("handle osc ii = {}", msg);
//    Ok(())
//}

fn route_eurorack_module(msg: &rosc::OscMessage, path: Vec<&str>) {
    println!("msg = {:?}", msg); //eg. OscMessage { addr: "/er301", args: [Float(0.0), Float(0.0)] }
    //println!("path = {:?}", &path); //eg. ["er301"]
    //let mut data: Vec<u16> = Vec::new();
    let m = msg.args.to_vec();
    let _cmd_args: Option<eurorack::Command> = None;

    
    let module_number = m[0].clone().int().unwrap() as usize;
    let port_number = m[2].clone().int().unwrap() as u8;
    let cmd = &m[1];
    let cmd_param = get_command_param(&m).unwrap();

    let module_name = match path[0] {
        "er301" | "Er301" => Some(eurorack::EuroModules::Er301),
        _ => None,
    };


    if let Some(module_name) = module_name {
        let command = get_module_command(&module_name, &cmd.clone().string().unwrap());
        println!("command = {:?}", command);
        match ii::send_i2c(module_name, module_number, port_number, command, cmd_param) {
         Ok(_) => {},
         Err(_) => println!("Unreachable module"),
        }
    } else {
        println!("Osc format error")
    }
}

fn get_module_command(
    module_name: &eurorack::EuroModules,
    command: &str,
) -> Option<eurorack::Command> {
    let cmd;
    match module_name {
        eurorack::EuroModules::Er301 => cmd = eurorack::er301::cmd_from_string(command),
    }
    cmd
}

fn get_command_param(osc_msg: &[OscType]) -> Option<Vec<u16>>{
    let mut _param = Vec::new();
    if let Some(param) = osc_msg.iter().nth(3) {
        if let OscType::Int(value) = param { 
            _param.push(*value as u16)
        }
    }

    if _param.len() > 0 {
        Some(_param)
    } else {
        None
    }
}

//fn route_custom_action(_msg: &rosc::OscMessage, _path: Vec<&str>) {
//    println!("Custom Action!");
//}

//fn get_module_number(module: i32) -> Option<usize> {
//    match module.parse::<usize>() {
//        Ok(value) => Some(value),
//        Err(_) => None,
//    }
//}


// fn get_port_number(path: &Vec<&str>) -> Option<u8> {
//     let port: Option<u8>;
//     if path.len() == 4 {
//         match path[3].parse::<u8>() {
//             Ok(value) => port = Some(value),
//             Err(_) => port = None,
//         }
//     } else {
//         port = None
//     }
//     port
// }
