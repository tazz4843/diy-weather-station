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
    pub temperature: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Humidity {
    /// Humidity in %RH
    pub humidity: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pressure {
    /// Pressure in hPa
    pub pressure: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct WindSpeed {
    /// Wind speed in m/s
    pub wind_speed: f64,
    /// Gust wind speed in m/s
    pub wind_gust: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct WindDirection {
    /// Wind direction in degrees
    pub wind_direction: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Rainfall {
    /// Rainfall in mm
    pub rainfall: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Uv {
    /// Light intensity in mW/cm²
    pub uv: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Light {
    /// Light intensity in lux
    pub light: f64,

    /// Raw light sensor reading for both visible and IR light
    #[serde(rename = "full_spectrum")]
    pub raw_light: i64,
    /// Raw light sensor reading for visible spectrum
    #[serde(rename = "visible")]
    pub raw_visible: i64,
    /// Raw light sensor reading for IR spectrum
    #[serde(rename = "infrared")]
    pub raw_ir: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AirQuality {
    /// Air quality index
    ///
    /// 1 - 2: Excellent - indefinite exposure
    /// 3: Good - 12 months exposure
    /// 4: Poor - long-term exposure may have adverse effects - max 1 month
    /// 5: Extremely poor - highly neurotoxic - max exposure of hours
    pub aqi: i64,

    /// TVOC in ppb
    pub tvoc: f64,

    /// eCO2 in ppm
    pub eco2: f64,

    /// Raw gas sensor reading
    pub resistances: [i64; 4],
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ParticulateMatter {
    /// PM2.5 in µg/m³
    pub pm_2_5: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Lightning {
    /// Number of strikes in the last 10 minutes
    pub strikes: i64,
    /// Distance to the closest strike in km, if any (+/- 1km)
    pub closest_distance: Option<i64>,
    /// Average strike distance in km, if any (+/- 1km)
    pub average_distance: Option<i64>,
    /// Farthest strike distance in km, if any (+/- 1km)
    pub farthest_distance: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Magnetometer {
    /// Magnetic field strength in µT along the x-axis
    pub x: f64,
    /// Magnetic field strength in µT along the y-axis
    pub y: f64,
    /// Magnetic field strength in µT along the z-axis
    pub z: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Noise {
    /// Noise level in dB
    pub noise: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Wetness {
    /// Is the rain sensor wet?
    pub is_wet: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Radiation {
    /// Radiation level in µSv/h
    pub radiation_sv: f64,
}
