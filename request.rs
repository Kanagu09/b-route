pub fn make(command: &Vec<u8>, data: &Vec<u8>) -> Vec<u8> {
    let unique_code: Vec<u8> = vec![0xd0, 0xea, 0x83, 0xfc];
    let length: u16 = 4 + (data.len() as u16);
    let header_checksum: u16 = sum(&unique_code) + sum(command) + (length >> 8) + (length % 0x100);
    let data_checksum: u16 = sum(data);

    let mut value: Vec<u8> = vec![];
    value.extend(&unique_code);
    value.extend(command);
    value.extend([(length >> 8) as u8, length as u8].to_vec());
    value.extend([(header_checksum >> 8) as u8, header_checksum as u8].to_vec());
    value.extend([(data_checksum >> 8) as u8, data_checksum as u8].to_vec());
    value.extend(data);

    value
}

pub fn sum(data: &Vec<u8>) -> u16 {
    let mut sum: u16 = 0;
    for value in data {
        sum += *value as u16;
    }
    sum
}