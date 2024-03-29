use super::*;
use super::*;
use crate::lib::eurorack::*;
use crate::lib::settings;
use rppal::i2c::I2c;
use std::error::Error;
use std::num::Wrapping;

#[allow(unused_assignments)]

pub fn send_i2c(
   module_name: EuroModules,
   module_number: usize,
   port_number: u8,
   command: Option<Command>,
   data: Vec<u16>,
 ) -> Result<(), Box<dyn Error>> {
   let mut i2c = I2c::with_bus(settings::I2CBUS).unwrap();
   let cmd = command;
   let module_address: Option<usize> = get_module_address(&module_name, module_number);
   let mut match_args = false;
   let mut args: &[Arg] = &[];
   let mut cmd_addr: u8 = 0;
   let port = offset_port_number(&module_name, port_number).unwrap();
   if let Some(cmd) = cmd {
     if data.len() == cmd.required {
       match_args = true;
       //args = cmd.args;
       cmd_addr = cmd.command_number;
     }
   }

   // if we have an address and a valid command -> carry on and send the buffer
   if let (Some(module_address), true) = (module_address, match_args) {
     let buffer = format_the_buffer(data, port);
     let final_buffer: &[u8] = &buffer;
   //  i2c.set_slave_address(module_address as u16).unwrap();
    // i2c.block_write(cmd_addr, final_buffer).unwrap();

     println!("module_address = {:?},cmd_addr = {:?},buffer = {:?}", module_address, cmd_addr, buffer)
   }
   Ok(())
 }

 fn get_module_address(module_name: &EuroModules, module_number: usize) -> Option<usize> {
   let addr;
   match module_name {
     EuroModules::Er301 => addr = Some(er301::ADDRESSES[((module_number + 3 - 1) % 3)]), // wrapped overflowed value
   }
   addr
 }

 fn offset_port_number(module_name: &EuroModules, port: u8) -> Option<u8> {
   let port_number;
   match module_name {
     EuroModules::Er301 => port_number = Some((port + 101 - 1) % 101), 
   }
   port_number
 }

 fn format_the_buffer(data: Vec<u16>, port: u8) -> Vec<u8> {
   // create a buffer -array of bytes- with the port number and the data
   // data type is u16 = 2^16 = max value = 65,536;
   // since bytes = 2^8 so 2^16, the buffer data will be represented by [port_value,2^8,2^8] (big-endian bytes order);
   // overflow value ( > 65,536 will be wrapped)
   let mut buffer: Vec<u8> = Vec::new();
   buffer.push(port);
   for i in data {
     let value = i.to_be_bytes();
     for s in 0..value.len() {
       buffer.push(value[s]);
     }
   }
   buffer
 }
