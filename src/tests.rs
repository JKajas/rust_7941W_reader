#[cfg(test)]
mod driver125khz {
    use crate::common::{Common, DriverProperty};
    use crate::driver_125khz::{DriverFunc, MAX_BUFFER_SIZE};
    struct TestDriver {
        buffer: [u8; MAX_BUFFER_SIZE],
        read_char: u8,
        command: Vec<u8>,
    }
    impl TestDriver {
        fn new() -> Self {
            Self {
                buffer: [0u8; MAX_BUFFER_SIZE],
                read_char: 0,
                command: Vec::new(),
            }
        }
    }
    impl DriverProperty for TestDriver {
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
        fn get_tag_id(&self) -> &[u8] {
            &self.buffer[..self.read_char as usize]
        }
    }
    impl DriverFunc for TestDriver {}

    impl Common for TestDriver {
        fn write_command(&mut self) -> () {}
    }
    #[test]
    fn test_compute_checksum() -> () {
        let driver = TestDriver::new();
        let mut command = [0xab, 0xba, 0x00, 0x17, 0x01, 0x12, 0x00];
        let cs_byte = command.len() - 1;
        let expected_command = [0xab, 0xba, 0x00, 0x17, 0x01, 0x12, 0x04];
        driver.compute_checksum(&mut command, cs_byte as u8);
        assert_eq!(command, expected_command)
    }

    #[test]
    fn test_preparing_read_request() -> () {
        let mut driver = TestDriver::new();
        let expected_command = [0xab, 0xba, 0x00, 0x15, 0x00, 0x15];
        driver.prepare_read_request();
        assert_eq!(driver.command, expected_command)
    }
    #[test]
    fn test_prepare_write_request_correct() -> () {
        let mut driver = TestDriver::new();
        driver.read_char = 4;
        let mut mock_data_buffer = [0u8; MAX_BUFFER_SIZE];
        mock_data_buffer[0..4].clone_from_slice(&[0x12, 0x14, 0xa1, 0x33]);
        driver.set_buffer(mock_data_buffer);

        driver.prepare_write_request();

        let expected_command = [
            0xab, 0xba, 0x00, 0x16, 0x04, 0x12, 0x14, 0xa1, 0x33, 0x86, 0, 0, 0, 0,
        ]; // Write command represents size of max combination of incoming data from device
        assert_eq!(driver.command, expected_command)
    }
    #[test]
    #[should_panic]
    fn test_prepare_write_request_data_too_long() -> () {
        let mut driver = TestDriver::new();
        driver.read_char = 9;
        let mut mock_data_buffer = [0u8; MAX_BUFFER_SIZE];
        mock_data_buffer[0..9]
            .clone_from_slice(&[0x12, 0x14, 0xa1, 0x33, 0x15, 0xb1, 0x17, 0x11, 0x11]);
        driver.set_buffer(mock_data_buffer);

        driver.prepare_write_request();
    }
    #[test]
    fn test_get_125khz_tag_id_correct() -> () {
        let mut driver = TestDriver::new();
        let mut mock_returned_buffer = [0u8; MAX_BUFFER_SIZE];
        let mut mock_tag_id = [0u8; MAX_BUFFER_SIZE];
        mock_returned_buffer[0..10]
            .clone_from_slice(&[0xcd, 0xdc, 0x00, 0x81, 0x04, 0x12, 0x14, 0xa1, 0x33, 0x00]);
        driver.set_buffer(mock_returned_buffer);
        driver.get_125khz_tag_id();
        mock_tag_id[0..4].copy_from_slice(&[0x12, 0x14, 0xa1, 0x33]);

        assert_eq!(driver.get_buffer(), &mock_tag_id);
    }
    #[test]
    fn test_get_125khz_tag_id_bad() -> () {
        let mut driver = TestDriver::new();
        let mut mock_returned_buffer = [0u8; MAX_BUFFER_SIZE];
        mock_returned_buffer[0..10]
            .clone_from_slice(&[0xcd, 0xdc, 0x00, 0x80, 0x04, 0x12, 0x14, 0xa1, 0x33, 0x00]);
        driver.set_buffer(mock_returned_buffer);
        driver.get_125khz_tag_id();
        assert_eq!(driver.get_buffer(), &[0u8; MAX_BUFFER_SIZE]);
    }
    #[test]
    fn test_get_125khz_tag_id_data_too_long() -> () {
        let mut driver = TestDriver::new();
        let mock_incoming_data = [
            0xcd, 0xdc, 0x00, 0x81, 0x0b, 0x12, 0x14, 0xa1, 0x33, 0x01, 0xa4, 0xd2, 0x42, 0x11,
            0x23, 0x17,
        ];
        //Mock writing
        for (dst, src) in driver.buffer.iter_mut().zip(mock_incoming_data.iter()) {
            *dst = *src
        }

        assert_eq!(
            driver.get_buffer(),
            &[0xcd, 0xdc, 0x00, 0x81, 0x0b, 0x12, 0x14, 0xa1, 0x33, 0x01, 0xa4, 0xd2, 0x42, 0x11]
        );
        driver.get_125khz_tag_id();
        assert_eq!(
            driver.get_buffer(),
            &[0x12, 0x14, 0xa1, 0x33, 0x01, 0xa4, 0xd2, 0x42, 0, 0, 0, 0, 0, 0]
        );
    }
    #[test]
    fn test_print_t5577_tag_id() -> () {
        let mut driver = TestDriver::new();
        let mut mock_returned_buffer = [0u8; MAX_BUFFER_SIZE];
        mock_returned_buffer[0..10]
            .clone_from_slice(&[0xcd, 0xdc, 0x00, 0x81, 0x04, 0x12, 0x14, 0xa1, 0x33, 0x00]);
        driver.set_buffer(mock_returned_buffer);
        driver.get_125khz_tag_id();
        assert_eq!(
            println!(
                "{:x?}",
                &driver.get_buffer()[..driver.get_read_char() as usize]
            ),
            println!("{:x?}", [0x12, 0x14, 0xa1, 0x33])
        )
    }
}
#[cfg(test)]
mod driver_nfc {

    use crate::common::{Common, DriverNFCProperty};
    use crate::driver_nfc::{DriverFunc, GroupPwd, MAX_BUFFER_SIZE};
    struct TestDriver {
        buffer: [u8; MAX_BUFFER_SIZE],
        read_char: u8,
        command: Vec<u8>,
        uid: [u8; 4],
        sector_data: [u8; 16],
        a_pwd: [u8; 6],
        b_pwd: [u8; 6],
    }
    impl TestDriver {
        fn new(a_pwd: [u8; 6], b_pwd: [u8; 6]) -> Self {
            Self {
                a_pwd,
                b_pwd,
                buffer: [0u8; MAX_BUFFER_SIZE],
                read_char: 0,
                command: Vec::new(),
                sector_data: [0u8; 16],
                uid: [0u8; 4],
            }
        }
    }
    impl DriverNFCProperty for TestDriver {
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
    impl Common for TestDriver {
        fn write_command(&mut self) {}
    }
    impl DriverFunc for TestDriver {}
    const A_PWD: [u8; 6] = [0u8; 6];
    const B_PWD: [u8; 6] = [0u8; 6];

    #[test]
    fn test_prepare_read_uid_request() -> () {
        let mut driver = TestDriver::new(A_PWD, B_PWD);
        driver.prepare_read_uid_request();
        assert_eq!(driver.get_command(), &[0xab, 0xba, 0x00, 0x10, 0x00, 0x10]);
    }
    #[test]
    fn test_prepare_write_uid_request() -> () {
        let mut driver = TestDriver::new(A_PWD, B_PWD);
        let mut testing_data = [0xab, 0xba, 0x00, 0x11, 0x04, 0x01, 0x02, 0x03, 0x04, 0x00];
        let len = testing_data.len() - 1;
        driver.prepare_write_uid_request([0x01, 0x02, 0x03, 0x04]);
        driver.compute_checksum(&mut testing_data, len as u8);
        assert_eq!(driver.get_command(), &testing_data);
    }
    #[test]
    fn test_prepare_read_specified_sector() -> () {
        let mut driver = TestDriver::new(A_PWD, B_PWD);
        let sector_number = 0;
        let block_number = 1;
        let mut testing_data = [
            0xab,
            0xba,
            0x00,
            0x12,
            0x09,
            sector_number,
            block_number,
            0x0a,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ];
        let len = testing_data.len() - 1;
        driver.compute_checksum(&mut testing_data, len as u8);
        driver.prepare_read_specified_sector(sector_number, block_number, GroupPwd::APassword);
        assert_eq!(driver.get_command(), &testing_data);
    }
    #[test]
    fn test_prepare_write_specified_sector() -> () {
        let mut driver = TestDriver::new(A_PWD, B_PWD);
        let sector_number = 0;
        let block_number = 1;
        let mut testing_data = [
            0xab,
            0xba,
            0x00,
            0x13,
            0x19,
            sector_number,
            block_number,
            0x0a,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x01,
            0x02,
            0x03,
            0x04,
            0x05,
            0x06,
            0x07,
            0x08,
            0x09,
            0x0a,
            0x0b,
            0x0c,
            0x0d,
            0x0e,
            0x10,
            0x00,
        ];
        let len = testing_data.len() - 1;
        driver.compute_checksum(&mut testing_data, len as u8);
        driver.prepare_write_specified_sector(
            sector_number,
            block_number,
            GroupPwd::APassword,
            [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
                0x0e, 0x10,
            ],
        );
        assert_eq!(driver.get_command(), &testing_data);
    }
    #[test]
    fn test_prepare_modify_password() -> () {
        let mut driver = TestDriver::new(A_PWD, B_PWD);
        let sector_number = 0;
        let mut testing_data = [
            0xab,
            0xba,
            0x00,
            0x14,
            0x0d,
            sector_number,
            0x0a,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x01,
            0x02,
            0x03,
            0x04,
            0x05,
            0x06,
            0x00,
        ];
        let len = testing_data.len() - 1;
        driver.compute_checksum(&mut testing_data, len as u8);
        driver.prepare_modify_password(
            sector_number,
            GroupPwd::APassword,
            [0x01, 0x02, 0x03, 0x04, 0x05, 0x06],
        );
        assert_eq!(driver.get_command(), &testing_data);
    }
}
