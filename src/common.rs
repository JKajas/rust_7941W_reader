pub trait Common {
    fn compute_checksum(&self, data: &mut [u8], cs_byte: u8) -> () {
        let mut cs = 0;
        for i in 0..data.len() {
            if i < 3 {
                continue;
            } else {
                cs ^= data[i]
            }
        }
        data[cs_byte as usize] = cs;
    }
    fn write_command(&mut self) -> () {}
}
