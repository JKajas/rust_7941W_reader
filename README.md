# rust_7941W_reader

## About

Repo contains driver for 7941W reader/writer working with raspberry pi written in Rust.

### Driver 125khz

Package was designed as wrapper for rpi_embedded crate that allows communication by UART, supporting 7941W protocol with checksum byte.\
Features:

- read tag
- write tag
- print read value storing in memory

## Quick start

By default program communicates with reader by UART, mapped to "serial0" on raspbian. Firstly, we need to correctly connect reader's pins to rpi's pins and enable serial UART on rpi.\
To connect devices:
| RPI's pin | Corresponding 7941W pin |
| :--------:| :----------------------:|
| 5V (Pin 2 or 4)| 5V |
|GND (Pin 6)| GND|
|TX (Pin 8)| RX|
|RX (Pin 10)| TX|

To enable UART edit `/boot/config.txt` and add `enable_uart=1` under `[all] section`\
Next, we need to download repo with:\
\
`git clone https://github.com/JKajas/rust_7941W_reader.git`\
\
Program should be compiled for specific platform.\
If we run program on the same platform where repository is we need to run:\
\
`cargo run`\
\
Repo supports cross compilation for Raspberry Pi 4. To do that we can run:\
\
`make build`\
\
which produces "driver" exec file.

## TODO

- [x] Driver for 125khz tags
- [ ] Driver for 13.56 MHz tags
