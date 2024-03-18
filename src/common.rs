use crate::driver_125khz::MAX_BUFFER_SIZE;
pub trait Common {
    fn _compute_checksum(&self, data: &mut [u8]) -> u8 {
        let mut cs = 0;

        for i in 3..data.len() {
            // Device wants checksum all bytes starts from address byte
            cs ^= data[i]
        }
        cs
    }

    fn compute_checksum(&self, data: &mut [u8], cs_byte: u8) -> () {
        let cs = self._compute_checksum(data);
        data[cs_byte as usize] = cs;
    }
    fn write_command(&mut self);
}

pub trait DriverProperty {
    fn set_command(&mut self, vector: Vec<u8>);
    fn get_command(&mut self) -> &mut Vec<u8>;
    fn set_read_char(&mut self, read_char: u8);
    fn get_read_char(&self) -> u8;
    fn set_buffer(&mut self, buffer: [u8; MAX_BUFFER_SIZE]);
    fn get_buffer(&self) -> &[u8; MAX_BUFFER_SIZE];
    fn get_tag_id(&self) -> &[u8];
}

pub trait DriverNFCProperty {
    fn set_command(&mut self, vector: Vec<u8>);
    fn get_command(&mut self) -> &mut Vec<u8>;
    fn set_read_char(&mut self, read_char: u8);
    fn get_read_char(&self) -> u8;
    fn set_buffer(&mut self, buffer: [u8; 31]);
    fn get_buffer(&self) -> &[u8; 31];
    fn get_tag_id(&self) -> &[u8];
    fn get_a_pwd(&self) -> ([u8; 6], u8);
    fn get_b_pwd(&self) -> ([u8; 6], u8);
}
