use std::{thread::sleep, time::Duration};

use serialport::SerialPort;

struct Dysv17f {
    port: Box<dyn SerialPort>,
    buffer: [u8;8],
}
const PLAY: [u8;3] = [0xAA,0x02,0x00];
const STOP: [u8;3] = [0xAA,0x04,0x00];
const SET_VOLUME: [u8;4] = [0xAA,0x13,0x01,0x00];

impl Dysv17f {
    fn new(port: Box<dyn SerialPort>)->Self {
        Dysv17f {
            port,
            buffer: [0_u8;8],
        }
    }

    fn play(&mut self) {
        self.send_command(&PLAY, Box::new(|_| {}));
    }

    fn stop(&mut self) {
        self.send_command(&STOP, Box::new(|_| {}));
    }

    fn set_volume(&mut self, volume: u8) {
        self.send_command(&SET_VOLUME, Box::new(move |buffer| {
            buffer[3] = volume;
        }))
    }

    fn send_command(&mut self, command: &[u8], mut params: Box<dyn FnMut(&mut [u8])>) {
        self.buffer[0..command.len()].copy_from_slice(&command);
        params(&mut self.buffer);
        self.add_crc(command.len());
        println!("{:x?}",&self.buffer[0..=command.len()]);
        self.port.write(&self.buffer[0..=command.len()]).expect("Write failed!");  
    }

    fn add_crc(&mut self, cmd_size: usize) {
        let mut crc = 0_u8;
        for i in 0..cmd_size {
            crc = crc.wrapping_add(self.buffer[i]);
        }
        self.buffer[cmd_size] = crc;
    }
}

fn main() {

    let mut port = serialport::new("/dev/cu.usbserial-0001", 9600)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    let mut driver = Dysv17f::new(port);

    driver.play();
    loop {
        for i in 0..0x20 {
            driver.set_volume(i);
            sleep(Duration::from_millis(500))
        }
    }
        // PLAY AA 02 00 AC

        // STOP AA 04 00 AE

        // CMD：AA 13 01 VOL SM

    // let mut output = [0_u8;4];
    // output[0] = 0xAA;
    // output[1] = 0x02;
    // output[2] = 0x00;
    // output[3] = 0xAC;

    // port.write(&output).expect("Write failed!");  
}
