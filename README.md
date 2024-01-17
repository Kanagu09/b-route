# B-route

This is a sample program for operating a Wi-SUN module `BP35C2-J11-T01` and communicating with a smart meter.
In the sample, you can get instantaneous power consumption data from the smart meter.

## Requirements

- a Wi-SUN module `BP35C2-J11-T01`
- rust 1.69.0

## Usage

1. Fill in the required information in `config.rs`.

- `SERIAL` : serial port name of the Wi-SUN module
  - example : `/dev/ttyUSB0`
- `BROUTE_ID` : authentication ID of the B-route issued by the power company
  - 32-digits decimal number
  - example : `00000000000000000000000000000000`
- `BROUTE_PASS` : password of the B-route issued by the power company
  - 12-character alphabet
  - example : `XXXXXXXXXXXX`

2. Add permission to serial port.

```sh
chmod a+rw /dev/ttyUSB0 # example
```

3. Execute the program.

```sh
cargo run
```
