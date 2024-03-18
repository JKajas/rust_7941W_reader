mod common;
mod driver_125khz;
mod driver_nfc;
mod tests;

use common::Common;
use driver_125khz::*;
use driver_nfc::*;
use rpi_embedded::uart::{Parity, Uart};
use std::io::{stdin, stdout, Write};

#[repr(u8)]
enum Commands {
    ReadNFCId = 48,
    ReadId = 53,
    WriteId = 54,
    PrintTag = 55,
}

fn main() {
    let mut uart: Uart = Uart::new(115200, Parity::None, 8, 1).unwrap();
    'main_loop: loop {
        let chosen_command = display_menu_and_return_pressed_key().as_bytes()[0];
        if [
            Commands::ReadId as u8,
            Commands::WriteId as u8,
            Commands::PrintTag as u8,
        ]
        .contains(&chosen_command)
        {
            let mut driver = Driver125khz::new(&mut uart);
            match chosen_command {
                c if c == Commands::ReadId as u8 => driver.prepare_read_request(),
                c if c == Commands::WriteId as u8 => driver.prepare_write_request(),
                c if c == Commands::PrintTag as u8 => {
                    driver.print_t5577_tag();
                    continue 'main_loop;
                }
                _ => continue 'main_loop,
            };
            driver.write_command();
            match chosen_command {
                c if c == Commands::ReadId as u8 => driver.get_125khz_tag_id(),
                _ => continue,
            };
        } else {
        }
    }
}

fn display_menu_and_return_pressed_key() -> String {
    let mut stdout = stdout();
    let menu = " 
    5) Read ID card number
    6) Write T5577 card number
    7) Print T5577 tag from memory\n";

    stdout.write(menu.as_bytes()).unwrap();
    let mut line = String::new();
    let _ = stdin().read_line(&mut line);
    line.trim().to_string()
}
