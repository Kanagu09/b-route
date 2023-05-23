// serial communication with wi-sun module
use std::time::Duration;
use std::io::Error;
use serialport::SerialPort;

use super::config;

pub fn open() -> Result<Box<dyn SerialPort>, Error> {
    let port: Box<dyn SerialPort> = serialport
        ::new(config::SERIAL, 115200)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .timeout(Duration::from_millis(10000))
        .open()?;
    Ok(port)
}

pub fn write(port: &mut Box<dyn SerialPort>, msg: &Vec<u8>) -> Result<(), Error> {
    port.write(msg)?;
    Ok(())
}

pub fn read(port: &mut Box<dyn SerialPort>) -> Result<Vec<u8>, Error> {
    let mut value: Vec<u8> = vec![];
    // read header
    let mut header_value: [u8; 8] = [0; 8];
    port.read(&mut header_value)?;
    value.extend(header_value.to_vec());
    // calc length
    let length: i32 = ((header_value[6] as i32) << 8) + (header_value[7] as i32);
    // read data
    for _i in 0..length {
        let mut data_value: [u8; 1] = [0; 1];
        port.read(&mut data_value).unwrap();
        value.extend(data_value.to_vec());
    }
    Ok(value)
}