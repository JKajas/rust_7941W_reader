use crate::common::Common;
use core::time;
use rpi_embedded::uart::Uart;
use std::thread::sleep;

pub struct Driver125khz<'a> {
    buffer: [u8; 20],
    read_char: u8,
    pub command: Vec<u8>,
    api: &'a mut Uart,
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
            buffer: [0u8; 20],
            read_char: 0,
            command: Vec::new(),
            api: uart,
        }
    }
    pub fn prepare_read_request(&mut self) -> () {
        let mut base_command = [0xab, 0xba, 0x00, 0x15, 0x00, 0x00];
        let cs_byte = base_command.len() as u8 - 1;
        self.compute_checksum(&mut base_command, cs_byte);
        self.command = base_command.into()
    }
    pub fn prepare_write_request(&mut self) -> () {
        let mut base_buffer: [u8; 14] = [0u8; 14];
        base_buffer[..5].clone_from_slice(&[0xab, 0xba, 0x00, 0x16, self.read_char]);
        base_buffer[5..5 + self.read_char as usize]
            .clone_from_slice(&self.buffer[0..self.read_char as usize]);
        self.compute_checksum(&mut base_buffer, 5 + self.read_char);
        self.command = base_buffer.into()
    }
    pub fn get_125khz_tag_id(&mut self) -> () {
        self.read_char = self.buffer[4] as u8;
        let mut id_buffer = [0u8; 20];
        id_buffer[..self.read_char as usize]
            .clone_from_slice(&self.buffer[5..5 + self.read_char as usize]);
        self.buffer = id_buffer;
        self.command.clear();
        println!("Result {:?} in memory", self.buffer);
    }
}
