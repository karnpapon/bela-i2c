// OSC server: Port number
pub const UDP_PORT: u16 = 9301;
// I2C bus available in Bela = 0,1,2
// initialized instant: I2c {
//    bus: 0,
//    funcs: Capabilities {
//        addr_10bit: false,
//        i2c_block_read: true,
//        i2c_block_write: true,
//        smbus_quick_command: false,
//        smbus_receive_byte: true,
//        smbus_send_byte: true,
//        smbus_read_byte: true,
//        smbus_write_byte: true,
//        smbus_read_word: true,
//        smbus_write_word: true,
//        smbus_process_call: true,
//        smbus_block_read: false,
//        smbus_block_write: true,
//        smbus_block_process_call: false,
//        smbus_pec: true,
//        smbus_host_notify: false,
//    },
//    i2cdev: File {
//        fd: 4,
//        path: "/dev/i2c-0",
//        read: true,
//        write: true,
//    },
//    addr_10bit: false,
//    address: 0,
//    not_sync: PhantomData,
//}
pub const I2CBUS: u8 = 0;

pub const LOCAL_IP: (u8, u8, u8, u8) = (127, 0, 0, 1);
pub const LOCAL_PORT: u16 = 7563;
pub const REMOTE_IP: (u8, u8, u8, u8) = (127, 0, 0, 1);
pub const REMOTE_PORT: u16 = 7562;
