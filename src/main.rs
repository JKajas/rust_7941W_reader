mod common;
mod driver_125khz;
mod tests;

use common::Common;
use driver_125khz::*;
use rpi_embedded::uart::{Parity, Uart};
use std::io::{stdin, stdout, Write};

#[repr(u8)]
enum Commands {
    ReadId = 53,
    WriteId = 54,
}

fn main() {
    let mut uart: Uart = Uart::with_path("/dev/serial0", 115200, Parity::None, 8, 1).unwrap();
    loop {
        let chosen_command = display_menu_and_return_pressed_key().as_bytes()[0];
        if [Commands::ReadId as u8, Commands::WriteId as u8].contains(&chosen_command) {
            let mut driver = Driver125khz::new(&mut uart);
            match chosen_command {
                c if c == Commands::ReadId as u8 => driver.prepare_read_request(),
                c if c == Commands::WriteId as u8 => driver.prepare_write_request(),
                _ => continue,
            };
            driver.write_command();
            match chosen_command {
                c if c == Commands::ReadId as u8 => driver.get_125khz_tag_id(),
                _ => continue,
            };
        }
    }
}

fn display_menu_and_return_pressed_key() -> String {
    let mut stdout = stdout();
    let menu = " 
    5) Read ID card number
    6) Write T5577 card number\n";
    stdout.write(menu.as_bytes()).unwrap();
    let mut line = String::new();
    let _ = stdin().read_line(&mut line);
    line.trim().to_string()
}
