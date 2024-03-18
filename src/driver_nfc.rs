use core::time;
use std::thread::sleep;

use rpi_embedded::uart::Uart;

use crate::common::{Common, DriverNFCProperty};

pub const MAX_BUFFER_SIZE: usize = 31;

pub enum GroupPwd {
    APassword,
    BPassword,
}
struct DriverNFC<'a> {
    api: &'a mut Uart,
    command: Vec<u8>,
    buffer: [u8; MAX_BUFFER_SIZE],
    read_char: u8,
    uid: [u8; 4],
    sector_data: [u8; 16],
    a_pwd: [u8; 6],
    b_pwd: [u8; 6],
}
impl<'a> DriverNFC<'a> {
    fn new(api: &'a mut Uart, a_pwd: [u8; 6], b_pwd: [u8; 6]) -> Self {
        Self {
            api,
            a_pwd,
            b_pwd,
            command: Vec::new(),
            read_char: 0,
            buffer: [0u8; MAX_BUFFER_SIZE],
            sector_data: [0u8; 16],
            uid: [0u8; 4],
        }
    }
}

impl<'a> DriverNFCProperty for DriverNFC<'a> {
    fn set_command(&mut self, vector: Vec<u8>) {
        self.command = vector;
    }
    fn set_read_char(&mut self, read_char: u8) {
        self.read_char = read_char;
    }
    fn set_buffer(&mut self, buffer: [u8; MAX_BUFFER_SIZE]) {
        self.buffer = buffer;
    }
    fn get_read_char(&self) -> u8 {
        self.read_char
    }
    fn get_command(&mut self) -> &mut Vec<u8> {
        &mut self.command
    }
    fn get_tag_id(&self) -> &[u8] {
        &self.uid
    }
    fn get_buffer(&self) -> &[u8; MAX_BUFFER_SIZE] {
        &self.buffer
    }
    fn get_a_pwd(&self) -> ([u8; 6], u8) {
        (self.a_pwd, 0x0a)
    }
    fn get_b_pwd(&self) -> ([u8; 6], u8) {
        (self.b_pwd, 0x0b)
    }
}

impl<'a> Common for DriverNFC<'a> {
    fn write_command(&mut self) {
        println!("Writing...");
        self.api.write_bytes(&self.command).unwrap();
        sleep(time::Duration::from_millis(200));
        self.api.read_bytes(&mut self.buffer).unwrap();
    }
}
pub trait DriverFunc: Common + DriverNFCProperty {
    fn prepare_read_uid_request(&mut self) -> () {
        let mut base_command = [0xab, 0xba, 0x00, 0x10, 0x00, 0x00];
        let cs_byte = base_command.len() as u8 - 1;
        self.compute_checksum(&mut base_command, cs_byte);
        self.set_command(base_command.into());
    }
    fn prepare_write_uid_request(&mut self, new_uid: [u8; 4]) -> () {
        let mut base_buffer: [u8; 10] = [0u8; 10];
        base_buffer[..5].clone_from_slice(&[0xab, 0xba, 0x00, 0x11, 0x04]);
        base_buffer[5..9].clone_from_slice(&new_uid);
        self.compute_checksum(&mut base_buffer, 9);
        self.set_command(base_buffer.into());
    }
    fn prepare_read_specified_sector(
        &mut self,
        sector_number: u8,
        block_number: u8,
        group_pwd: GroupPwd,
    ) -> () {
        let mut base_buffer = self._create_base_buffer::<15>(
            sector_number,
            Some(block_number),
            group_pwd,
            0x12,
            0x09,
        );
        self.compute_checksum(&mut base_buffer, 14);
        self.set_command(base_buffer.into());
    }
    fn prepare_write_specified_sector(
        &mut self,
        sector_number: u8,
        block_number: u8,
        group_pwd: GroupPwd,
        data: [u8; 16],
    ) -> () {
        let mut base_buffer = self._create_base_buffer::<31>(
            sector_number,
            Some(block_number),
            group_pwd,
            0x13,
            0x19,
        );
        base_buffer[14..30].clone_from_slice(&data);
        self.compute_checksum(&mut base_buffer, 30);
        self.set_command(base_buffer.into());
    }
    fn prepare_modify_password(
        &mut self,
        sector_number: u8,
        group_pwd: GroupPwd,
        new_pwd: [u8; 6],
    ) -> () {
        let mut base_buffer =
            self._create_base_buffer::<20>(sector_number, None, group_pwd, 0x14, 0x0d);
        base_buffer[13..19].clone_from_slice(&new_pwd);
        self.compute_checksum(&mut base_buffer, 19);
        self.set_command(base_buffer.into());
    }
    fn _create_base_buffer<const BUFFER_SIZE: usize>(
        &mut self,
        sector_number: u8,
        block_number: Option<u8>,
        group_pwd: GroupPwd,
        command: u8,
        data_size: u8,
    ) -> [u8; BUFFER_SIZE] {
        let mut base_buffer: [u8; BUFFER_SIZE] = [0u8; BUFFER_SIZE];
        let (pwd, pwd_byte) = match group_pwd {
            GroupPwd::APassword => self.get_a_pwd(),
            GroupPwd::BPassword => self.get_b_pwd(),
        };

        if block_number.is_some() {
            base_buffer[..8].clone_from_slice(&[
                0xab,
                0xba,
                0x00,
                command,
                data_size,
                sector_number,
                block_number.unwrap(),
                pwd_byte,
            ]);
            base_buffer[8..8 + 6].clone_from_slice(&pwd);
            return base_buffer;
        } else {
            base_buffer[..7].clone_from_slice(&[
                0xab,
                0xba,
                0x00,
                command,
                data_size,
                sector_number,
                pwd_byte,
            ]);
            base_buffer[7..7 + 6].clone_from_slice(&pwd);
            return base_buffer;
        }
    }
}
