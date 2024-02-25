use crate::common::{Common, DriverProperty};
use core::time;
use rpi_embedded::uart::Uart;
use std::thread::sleep;

pub const MAX_BUFFER_SIZE: usize = 14;

pub struct Driver125khz<'a> {
    buffer: [u8; MAX_BUFFER_SIZE],
    read_char: u8,
    command: Vec<u8>,
    api: &'a mut Uart,
}
impl<'a> DriverProperty for Driver125khz<'a> {
    fn set_command(&mut self, vector: Vec<u8>) {
        self.command = vector;
    }
    fn get_command(&mut self) -> &mut Vec<u8> {
        &mut self.command
    }
    fn set_read_char(&mut self, read_char: u8) {
        self.read_char = read_char
    }
    fn get_read_char(&self) -> u8 {
        self.read_char
    }
    fn set_buffer(&mut self, buffer: [u8; MAX_BUFFER_SIZE]) {
        self.buffer = buffer
    }
    fn get_buffer(&self) -> &[u8; MAX_BUFFER_SIZE] {
        &self.buffer
    }
    fn get_t5577_id(&self) -> &[u8] {
        &self.buffer[..self.read_char as usize]
    }
}
impl<'a> Common for Driver125khz<'a> {
    fn write_command(&mut self) -> () {
        println!("Writing..");
        self.api.write_bytes(&self.command).unwrap();
        sleep(time::Duration::from_millis(200)); // Required interval to FIFO buffer on board
                                                 // propertely filled
        self.api.read_bytes(&mut self.buffer).unwrap();
    }
}

impl<'a> Driver125khz<'a> {
    pub fn new(uart: &'a mut Uart) -> Self {
        Self {
            buffer: [0u8; MAX_BUFFER_SIZE],
            read_char: 0,
            command: Vec::new(),
            api: uart,
        }
    }
}

impl<'a> DriverFunc for Driver125khz<'a> {}

pub trait DriverFunc: Common + DriverProperty {
    fn prepare_read_request(&mut self) -> () {
        let mut base_command = [0xab, 0xba, 0x00, 0x15, 0x00, 0x00];
        let cs_byte = base_command.len() as u8 - 1;
        self.compute_checksum(&mut base_command, cs_byte);
        self.set_command(base_command.into());
    }
    fn prepare_write_request(&mut self) -> () {
        let mut base_buffer: [u8; 14] = [0u8; 14];
        base_buffer[..5].clone_from_slice(&[0xab, 0xba, 0x00, 0x16, self.get_read_char()]);
        base_buffer[5..5 + self.get_read_char() as usize]
            .clone_from_slice(&self.get_buffer()[0..self.get_read_char() as usize]);
        self.compute_checksum(&mut base_buffer, 5 + self.get_read_char());
        self.set_command(base_buffer.into())
    }
    fn get_125khz_tag_id(&mut self) -> () {
        let buffer = self.get_buffer();
        if buffer[3] == 0x80 {
            println!("Reading operation failed");
            self.set_buffer([0u8; MAX_BUFFER_SIZE]);
            return;
        } else if buffer[4] + 5 >= 14 {
            self.set_read_char(8);
            println!("Data was to long to write to T5577, program will store only 8 byte of data");
        } else {
            self.set_read_char(buffer[4] as u8);
        }
        let mut id_buffer = [0u8; MAX_BUFFER_SIZE];
        id_buffer[..self.get_read_char() as usize]
            .clone_from_slice(&self.get_buffer()[5..5 + self.get_read_char() as usize]);
        self.set_buffer(id_buffer);
        self.get_command().clear();
        println!("Result {:?} in memory", self.get_buffer());
    }
    fn print_t5577_tag(&self) -> () {
        println!("{:x?}", self.get_t5577_id());
    }
}
