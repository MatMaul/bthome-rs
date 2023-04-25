#![no_std]

#[cfg(feature = "encryption")]
pub mod encryption;

#[cfg(feature = "std")]
extern crate std;

use core::fmt;
use tinyvec::SliceVec;

const TEMPERATURE_OBJECT_ID: u8 = 0x02;
const HUMIDITY_OBJECT_ID: u8 = 0x03;
const PM2_5_OBJECT_ID: u8 = 0x0d;
const PM10_OBJECT_ID: u8 = 0x0e;
const CO2_OBJECT_ID: u8 = 0x12;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BTHomeError {
    #[cfg(feature = "encryption")]
    Encrypt,
}

/// Result type alias with [`Error`].
pub type Result<T> = core::result::Result<T, BTHomeError>;

impl fmt::Display for BTHomeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("bthome::Error")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for BTHomeError {}

#[derive(Debug, Clone, Copy)]
pub struct BTHomeData {
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
    pub co2: Option<u16>,
    pub pm2_5: Option<u16>,
    pub pm10: Option<u16>,
}

impl BTHomeData {
    pub const fn new() -> BTHomeData {
        BTHomeData {
            temperature: None,
            humidity: None,
            co2: None,
            pm2_5: None,
            pm10: None,
        }
    }

    pub const fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub const fn humidity(mut self, rh: f32) -> Self {
        self.humidity = Some(rh);
        self
    }

    pub const fn co2(mut self, co2: u16) -> Self {
        self.co2 = Some(co2);
        self
    }

    pub const fn pm2_5(mut self, pm2_5: u16) -> Self {
        self.pm2_5 = Some(pm2_5);
        self
    }

    pub const fn pm10(mut self, pm10: u16) -> Self {
        self.pm10 = Some(pm10);
        self
    }
}

pub struct BTHomeUnencryptedSerializer {}

impl BTHomeUnencryptedSerializer {
    pub fn new() -> BTHomeUnencryptedSerializer {
        BTHomeUnencryptedSerializer {}
    }

    pub fn serialize_to(&self, data: BTHomeData, buffer: &mut [u8]) -> Result<usize> {
        let mut buffer = SliceVec::from(buffer);
        buffer.set_len(0);

        // BTHome Device Info (Unencrypted v2)
        buffer.push(0x40);

        Self::add_payload(data, &mut buffer);

        Ok(buffer.len())
    }

    #[cfg(feature = "std")]
    pub fn serialize(&self, data: BTHomeData) -> Result<std::vec::Vec<u8>> {
        let mut buffer = [0u8; 256];

        let size = self.serialize_to(data, &mut buffer)?;

        Ok(buffer[0..size].to_vec())
    }

    fn add_payload(data: BTHomeData, payload: &mut SliceVec<u8>) {
        if let Some(temperature) = data.temperature {
            // Temperature i16 factor 0.01
            payload.push(TEMPERATURE_OBJECT_ID);
            payload.extend(((temperature / 0.01) as i16).to_le_bytes());
        }

        if let Some(humidity) = data.humidity {
            // Humidity u16 factor 0.01
            payload.push(HUMIDITY_OBJECT_ID);
            payload.extend(((humidity / 0.01) as u16).to_le_bytes());
        }

        if let Some(pm2_5) = data.pm2_5 {
            // PM2.5 u16 factor 1
            payload.push(PM2_5_OBJECT_ID);
            payload.extend((pm2_5 as u16).to_le_bytes());
        }

        if let Some(pm10) = data.pm10 {
            // PM10 u16 factor 1
            payload.push(PM10_OBJECT_ID);
            payload.extend((pm10 as u16).to_le_bytes());
        }

        if let Some(co2) = data.co2 {
            // CO2 u16 factor 1
            payload.push(CO2_OBJECT_ID);
            payload.extend((co2 as u16).to_le_bytes());
        }
    }
}

pub const SERVICE_UUID: u16 = 0xFCD2;
