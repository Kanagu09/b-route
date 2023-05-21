use std::io::Error;
use serialport::SerialPort;

mod config;
mod serial;
mod request;

fn main() -> Result<(), Error> {
    // connect serial port
    let mut port: Box<dyn SerialPort> = serial::open()?;

    let udp_port: Vec<u8> = vec![(3610 >> 8) as u8, (3610 % 256) as u8];

    // initial setting sequence
    println!("hardware reset");
    let req: Vec<u8> = request::make(&mut vec![0x00, 0xd9], &mut vec![]);
    serial::write(&mut port, &req)?;
    print("tx", &req);
    let res: Vec<u8> = serial::read(&mut port)?;
    print("rx", &res);
    println!();

    println!("initial setting");
    let req: Vec<u8> = request::make(&mut vec![0x00, 0x5f], &mut vec![0x05, 0x00, 0x04, 0x00]);
    serial::write(&mut port, &req)?;
    print("tx", &req);
    let res: Vec<u8> = serial::read(&mut port)?;
    print("rx", &res);
    println!();

    println!("B-route PANA setting");
    let mut data: Vec<u8> = vec![];
    data.extend(config::BROUTE_ID.as_bytes().to_vec());
    data.extend(config::BROUTE_PASS.as_bytes().to_vec());
    let req: Vec<u8> = request::make(&mut vec![0x00, 0x54], &mut data);
    serial::write(&mut port, &req)?;
    print("tx", &req);
    let res: Vec<u8> = serial::read(&mut port)?;
    print("rx", &res);
    println!();

    // active scan sequence
    println!("active scan");
    let mut channel: u8 = u8::MAX;
    let mut data: Vec<u8> = vec![0x07, 0x00, 0x03, 0xff, 0xf0, 0x01];
    data.extend(config::BROUTE_ID.as_bytes()[24..].to_vec());
    let req: Vec<u8> = request::make(&mut vec![0x00, 0x51], &mut data);
    serial::write(&mut port, &req)?;
    print("tx", &req);
    for i in 0..14 {
        let res: Vec<u8> = serial::read(&mut port)?;
        print("rx", &res);
        if res[12] == 0x00 {
            channel = i + 4;
        }
    }
    let res: Vec<u8> = serial::read(&mut port)?;
    print("rx", &res);
    println!();
    // let mut channel: u8 = 0x11;

    if channel == u8::MAX {
        panic!("ERROR : cannot setting communication channel.");
    }

    // B-route connection sequence
    println!("initial setting");
    let req: Vec<u8> = request::make(&mut vec![0x00, 0x5f], &mut vec![0x05, 0x00, channel, 0x00]);
    serial::write(&mut port, &req)?;
    print("tx", &req);
    let res: Vec<u8> = serial::read(&mut port)?;
    print("rx", &res);
    println!();

    println!("start B-route");
    let req: Vec<u8> = request::make(&mut vec![0x00, 0x53], &mut vec![]);
    serial::write(&mut port, &req)?;
    print("tx", &req);
    let res: Vec<u8> = serial::read(&mut port)?;
    print("rx", &res);
    println!();

    println!("open udp port");
    let req: Vec<u8> = request::make(&mut vec![0x00, 0x05], &udp_port);
    serial::write(&mut port, &req)?;
    print("tx", &req);
    let res: Vec<u8> = serial::read(&mut port)?;
    print("rx", &res);
    println!();

    println!("start B-route PANA");
    let req: Vec<u8> = request::make(&mut vec![0x00, 0x56], &mut vec![]);
    serial::write(&mut port, &req)?;
    print("tx", &req);
    let res: Vec<u8> = serial::read(&mut port)?;
    print("rx", &res);
    println!();

    println!("result of PANA authentication");
    let res: Vec<u8> = serial::read(&mut port)?;
    print("rx", &res);
    println!();

    println!("notification of data receive");
    let res: Vec<u8> = serial::read(&mut port)?;
    print("rx", &res);
    println!();

    let addr: Vec<u8> = res[12..28].to_vec();

    // data request sequence
    println!("send data");
    let mut data: Vec<u8> = vec![];
    data.extend(addr);
    data.extend(&udp_port);
    data.extend(&udp_port);
    data.extend([0x00, 0x0e].to_vec());
    data.extend(
        [
            0x10, 0x81, 0x00, 0x00, 0x05, 0xff, 0x01, 0x02, 0x88, 0x01, 0x62, 0x01, 0xe7, 0x00,
        ].to_vec()
    );
    let req: Vec<u8> = request::make(&mut vec![0x00, 0x08], &mut data);
    serial::write(&mut port, &req)?;
    print("tx", &req);
    let res: Vec<u8> = serial::read(&mut port)?;
    print("rx", &res);
    let res: Vec<u8> = serial::read(&mut port)?;
    print("rx", &res);
    println!();

    let data: Vec<u8> = res[39..].to_vec();
    print("echonet lite data", &data);

    let data: Vec<u8> = data[14..].to_vec();
    print("value data", &data);

    let kwh: i32 =
        ((data[0] as i32) << 24) +
        ((data[1] as i32) << 16) +
        ((data[2] as i32) << 8) +
        (data[3] as i32);
    println!("{:?} kwh", kwh);

    Ok(())
}

fn print(tag: &str, data: &Vec<u8>) {
    print!("{} : ", tag);
    for value in data {
        print!("{:02x} ", value);
    }
    println!();
}