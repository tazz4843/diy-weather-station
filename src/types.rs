#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub enum WebSocketInbound {
    IncomingData(SensorData),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct SensorData {
    pub sensors: Vec<Sensor>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Sensor {
    Temperature(Temperature),
    Humidity(Humidity),
    Pressure(Pressure),
    WindSpeed(WindSpeed),
    WindDirection(WindDirection),
    Rainfall(Rainfall),
    Uv(Uv),
    Light(Light),
    AirQuality(AirQuality),
    ParticulateMatter(ParticulateMatter),
    Lightning(Lightning),
    Magnetometer(Magnetometer),
    Noise(Noise),
    Wetness(Wetness),
    Radiation(Radiation),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Temperature {
    /// Temperature in degrees Celsius
    pub temperature: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Humidity {
    /// Humidity in %RH
    pub humidity: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pressure {
    /// Pressure in hPa
    pub pressure: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct WindSpeed {
    /// Wind speed in m/s
    pub wind_speed: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct WindDirection {
    /// Wind direction in degrees
    pub wind_direction: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rainfall {
    /// Rainfall in mm
    pub rainfall: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Uv {
    /// Light intensity in mW/cm²
    pub uv: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Light {
    /// Light intensity in lux
    pub light: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AirQuality {
    /// VOC index - 0 to 500
    ///
    /// * 0 - 50: Excellent.
    ///   Pure air, best for well-being.
    ///   No measures required
    /// * 51 - 100: Good.
    ///   No irritation or impact on well-being.
    ///   No measures required
    /// * 101 - 150: Lightly polluted.
    ///   Reduction of well-being possible.
    ///   Caution suggested
    /// * 151 - 200: Moderately polluted.
    ///   More significant irritation possible.
    ///   Strong caution suggested, reduce exposure
    /// * 201 - 250: Heavily polluted.
    ///   Exposure may lead to effects like headache depending on type of VOCs.
    ///   No exposure recommended in high risk groups, reduce exposure in others
    /// * 251 - 350: Severely polluted.
    ///   More severe effects possible, depending on type of VOCs.
    ///   No exposure in medium or high risk groups, reduce exposure in healthy people
    /// * 351 - 500: Extremely polluted.
    ///   Headaches, other neurotoxic effects possible.
    ///   No exposure to anyone.
    pub air_quality: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ParticulateMatter {
    /// PM2.5 in µg/m³
    pub pm_2_5: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Lightning {
    /// Number of strikes in the last 10 minutes
    pub strikes: u32,
    /// Distance to the closest strike in km, if any (+/- 1km)
    pub closest_distance: Option<f32>,
    /// Average strike distance in km, if any (+/- 1km)
    pub average_distance: Option<f32>,
    /// Farthest strike distance in km, if any (+/- 1km)
    pub farthest_distance: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Magnetometer {
    /// Magnetic field strength in µT along the x-axis
    pub x: f32,
    /// Magnetic field strength in µT along the y-axis
    pub y: f32,
    /// Magnetic field strength in µT along the z-axis
    pub z: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Noise {
    /// Noise level in dB
    pub noise: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Wetness {
    /// Is the rain sensor wet?
    pub is_wet: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Radiation {
    /// Radiation level in µSv/h
    pub radiation_sv: f32,
    /// Radiation level in cpm
    pub radiation_cpm: f32,
    /// Radiation level in cps
    pub radiation_cps: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn msgpack_enum_test() {
        let test_enum = WebSocketInbound::IncomingData(SensorData {
            sensors: vec![Sensor::Temperature(Temperature { temperature: 1.0 })],
        });

        // attempt to serialize the enum
        let serialized = rmp_serde::to_vec(&test_enum).unwrap();
        // then deserialize it
        let deserialized: WebSocketInbound = rmp_serde::from_slice(&serialized).unwrap();
        // check that the deserialized enum is the same as the original
        assert_eq!(test_enum, deserialized);
    }
}
