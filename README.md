# DIY Weather Station

A weather station packed to the brim with features.
Tracks temperature, humidity, pressure, wind speed, wind direction, rain,
UV, light, and (in the future) air quality, Earth's magnetic field,
and lightning strikes, and collects it into a Prometheus database,
where it can be viewed in Grafana.

## Setup
I strongly recommend you view the [wiki](https://github.com/tazz4843/diy-weather-station/wiki) for
more details on how to set up the weather station. The following is a
quick overview of the setup, but it is not comprehensive.

## Hardware
Lots of sensors, a RPi Pico, wires, and some commodity parts.
See the [wiki](https://github.com/tazz4843/diy-weather-station/wiki/Hardware) for more details.

## Software
This repo contains both the firmware for the RPi Pico and the Rust backend
that handles collecting and storing the data. 

### Firmware
This project uses PlatformIO to build and deploy the firmware.

#### Building
You will need to edit the `platformio.ini` file to include your WiFi
credentials and the IP address of the backend server.

```bash
pio run
```

### Backend
The Rust app can run on basically any hardware that can run
Prometheus and Grafana, with the ability to communicate with the
Pico over WiFi.

#### Building
```bash
cargo build --release
```

#### Running
Check out the [wiki](https://github.com/tazz4843/diy-weather-station/wiki/Backend) for more details,
including how to run the backend as a systemd service.

## Contributing
Contributions are welcome! Please open an issue or PR if you have any
questions or suggestions.

## License
EUPL-1.2
