#![no_std]

#[cfg(feature = "encryption")]
pub mod encryption;

#[cfg(feature = "std")]
extern crate std;

use core::fmt;
use tinyvec::SliceVec;

const BATTERY_OBJECT_ID: u8 = 0x01;
const TEMPERATURE_OBJECT_ID: u8 = 0x02;
const HUMIDITY_OBJECT_ID: u8 = 0x03;
const PRESSURE_OBJECT_ID: u8 = 0x04;
const ILLUMINANCE_OBJECT_ID: u8 = 0x05;
const MASS_KG_OBJECT_ID: u8 = 0x06;
const MASS_LB_OBJECT_ID: u8 = 0x06;
const POWER_OBJECT_ID: u8 = 0x0b;
// const VOLTAGE_OBJECT_ID: u8 = 0x0c;
const PM2_5_OBJECT_ID: u8 = 0x0d;
const PM10_OBJECT_ID: u8 = 0x0e;
const CO2_OBJECT_ID: u8 = 0x12;
const TVOC_OBJECT_ID: u8 = 0x13;
// const MOISTURE_OBJECT_ID: u8 = 0x14;
// const CURRENT_OBJECT_ID: u8 = 0x43;
// const GAS_OBJECT_ID: u8 = 0x4c;
// const ENERGY_OBJECT_ID: u8 = 0x4d;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BTHomeError {
    #[cfg(feature = "encryption")]
    Encrypt,
    BufferOverflow,
    ValueOverflow,
    ValueUnderflow,
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
    pub battery: Option<u8>,
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
    pub pressure: Option<f32>,
    pub illuminance: Option<f32>,
    pub mass_kg: Option<f32>,
    pub mass_lb: Option<f32>,
    pub power: Option<f32>,
    // pub voltage: Option<f32>,
    pub pm2_5: Option<u16>,
    pub pm10: Option<u16>,
    pub co2: Option<u16>,
    pub tvoc: Option<u16>,
    // pub moisture: Option<f32>,
    // pub current: Option<f32>,
    // pub gas: Option<f32>,
    // pub energy: Option<f32>,
}

impl BTHomeData {
    pub const fn new() -> BTHomeData {
        BTHomeData {
            battery: None,
            temperature: None,
            humidity: None,
            pressure: None,
            illuminance: None,
            mass_kg: None,
            mass_lb: None,
            power: None,
            // voltage: None,
            pm2_5: None,
            pm10: None,
            co2: None,
            tvoc: None,
            // moisture: None,
            // current: None,
            // gas: None,
            // energy: None,
        }
    }

    pub const fn battery(mut self, val: u8) -> Self {
        self.battery = Some(val);
        self
    }

    pub const fn temperature(mut self, val: f32) -> Self {
        self.temperature = Some(val);
        self
    }

    pub const fn humidity(mut self, val: f32) -> Self {
        self.humidity = Some(val);
        self
    }

    pub const fn pressure(mut self, val: f32) -> Self {
        self.pressure = Some(val);
        self
    }

    pub const fn illuminance(mut self, val: f32) -> Self {
        self.illuminance = Some(val);
        self
    }

    pub const fn mass_kg(mut self, val: f32) -> Self {
        self.mass_kg = Some(val);
        self
    }

    pub const fn mass_lb(mut self, val: f32) -> Self {
        self.mass_lb = Some(val);
        self
    }

    pub const fn pm2_5(mut self, val: u16) -> Self {
        self.pm2_5 = Some(val);
        self
    }

    pub const fn pm10(mut self, val: u16) -> Self {
        self.pm10 = Some(val);
        self
    }

    pub const fn co2(mut self, val: u16) -> Self {
        self.co2 = Some(val);
        self
    }

    pub const fn tvoc(mut self, val: u16) -> Self {
        self.tvoc = Some(val);
        self
    }
}

pub struct BTHomeUnencryptedSerializer {}

impl BTHomeUnencryptedSerializer {
    pub fn new() -> BTHomeUnencryptedSerializer {
        BTHomeUnencryptedSerializer {}
    }

    pub fn serialize_to(&self, data: BTHomeData, buffer: &mut [u8]) -> Result<usize> {
        // BTHome Device Info (Unencrypted v2)
        buffer[0] = 0x40;

        let payload_size = add_payload(data, &mut buffer[1..])?;

        Ok(payload_size + 1)
    }

    #[cfg(feature = "std")]
    pub fn serialize(&self, data: BTHomeData) -> Result<std::vec::Vec<u8>> {
        let mut buffer = [0u8; 256];
        let size = self.serialize_to(data, &mut buffer)?;
        Ok(buffer[0..size].to_vec())
    }
}

fn add_payload(data: BTHomeData, payload: &mut [u8]) -> Result<usize> {
    let mut payload =
        SliceVec::try_from_slice_len(payload, 0).ok_or(BTHomeError::BufferOverflow)?;

    if let Some(val) = data.battery {
        add_u8(&mut payload, BATTERY_OBJECT_ID, val)?;
    }

    if let Some(val) = data.temperature {
        add_i16_from_f32(&mut payload, TEMPERATURE_OBJECT_ID, val, 0.01)?;
    }

    if let Some(val) = data.humidity {
        add_u16_from_f32(&mut payload, HUMIDITY_OBJECT_ID, val, 0.01)?;
    }

    if let Some(val) = data.pressure {
        add_u24_from_f32(&mut payload, PRESSURE_OBJECT_ID, val, 0.01)?;
    }

    if let Some(val) = data.illuminance {
        add_u24_from_f32(&mut payload, ILLUMINANCE_OBJECT_ID, val, 0.01)?;
    }

    if let Some(val) = data.mass_kg {
        add_u16_from_f32(&mut payload, MASS_KG_OBJECT_ID, val, 0.01)?;
    }

    if let Some(val) = data.mass_lb {
        add_u16_from_f32(&mut payload, MASS_LB_OBJECT_ID, val, 0.01)?;
    }

    if let Some(val) = data.power {
        add_u24_from_f32(&mut payload, POWER_OBJECT_ID, val, 0.01)?;
    }

    if let Some(val) = data.pm2_5 {
        add_u16(&mut payload, PM2_5_OBJECT_ID, val)?;
    }

    if let Some(val) = data.pm10 {
        add_u16(&mut payload, PM10_OBJECT_ID, val)?;
    }

    if let Some(val) = data.co2 {
        add_u16(&mut payload, CO2_OBJECT_ID, val)?;
    }

    if let Some(val) = data.tvoc {
        add_u16(&mut payload, TVOC_OBJECT_ID, val)?;
    }

    Ok(payload.len())
}

fn add_u8(buffer: &mut SliceVec<u8>, object_id: u8, val: u8) -> Result<()> {
    check_remaining_capacity(&buffer, 2)?;
    buffer.push(object_id);
    buffer.push(val);
    Ok(())
}

fn add_u16(buffer: &mut SliceVec<u8>, object_id: u8, val: u16) -> Result<()> {
    check_remaining_capacity(&buffer, 3)?;
    buffer.push(object_id);
    buffer.extend(val.to_le_bytes());
    Ok(())
}

fn add_u16_from_f32(buffer: &mut SliceVec<u8>, object_id: u8, val: f32, factor: f32) -> Result<()> {
    let scaled_val = val / factor;
    if scaled_val > u16::MAX as f32 {
        return Err(BTHomeError::ValueOverflow);
    }
    if scaled_val < 0.0 {
        return Err(BTHomeError::ValueUnderflow);
    }
    check_remaining_capacity(&buffer, 3)?;
    buffer.push(object_id);
    buffer.extend((scaled_val as u16).to_le_bytes());
    Ok(())
}

fn add_i16_from_f32(buffer: &mut SliceVec<u8>, object_id: u8, val: f32, factor: f32) -> Result<()> {
    let scaled_val = val / factor;
    if scaled_val > i16::MAX as f32 {
        return Err(BTHomeError::ValueOverflow);
    }
    if scaled_val < i16::MIN as f32 {
        return Err(BTHomeError::ValueUnderflow);
    }
    check_remaining_capacity(&buffer, 3)?;
    buffer.push(object_id);
    buffer.extend((scaled_val as i16).to_le_bytes());
    Ok(())
}

const U24_MAX: f32 = (256 * 256 * 256) as f32;

fn add_u24_from_f32(buffer: &mut SliceVec<u8>, object_id: u8, val: f32, factor: f32) -> Result<()> {
    let scaled_val = val / factor;
    if scaled_val > U24_MAX {
        return Err(BTHomeError::ValueOverflow);
    }
    if scaled_val < 0.0 {
        return Err(BTHomeError::ValueUnderflow);
    }
    check_remaining_capacity(&buffer, 4)?;
    buffer.push(object_id);
    buffer.extend_from_slice(&(((val / factor) as u32).to_le_bytes())[0..3]);
    Ok(())
}

fn check_remaining_capacity(slice: &SliceVec<u8>, needed_bytes: usize) -> Result<()> {
    if slice.capacity() - slice.len() < needed_bytes {
        return Err(BTHomeError::BufferOverflow);
    }
    Ok(())
}

pub const SERVICE_UUID: u16 = 0xFCD2;

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_DATA: crate::BTHomeData = crate::BTHomeData::new()
        .temperature(18.6)
        .humidity(20.5)
        .illuminance(0.02)
        .co2(428)
        .pm2_5(49);
    const TEST_BYTES: [u8; 17] = [64, 2, 68, 7, 3, 2, 8, 5, 2, 0, 0, 13, 49, 0, 18, 172, 1];

    #[test]
    fn serialize() {
        let serializer = BTHomeUnencryptedSerializer::new();
        let mut buffer = [0u8; 256];
        let size = serializer.serialize_to(TEST_DATA, &mut buffer).unwrap();
        assert_eq!(buffer[0..size], TEST_BYTES);
    }

    #[test]
    #[cfg(feature = "std")]
    fn serialize_std() {
        let serializer = super::BTHomeUnencryptedSerializer::new();
        let bytes = serializer.serialize(TEST_DATA).unwrap();
        assert_eq!(bytes, TEST_BYTES);
    }

    #[test]
    fn buffer_overflow() {
        let serializer = BTHomeUnencryptedSerializer::new();
        let mut buffer = [0u8; 2];
        let res = serializer.serialize_to(TEST_DATA, &mut buffer);
        assert_eq!(res, Err(crate::BTHomeError::BufferOverflow));
    }

    #[test]
    fn u16_overflow() {
        let serializer = BTHomeUnencryptedSerializer::new();
        let mut buffer = [0u8; 256];
        let res = serializer.serialize_to(BTHomeData::new().humidity(1000.0), &mut buffer);
        assert_eq!(res, Err(crate::BTHomeError::ValueOverflow));
    }

    #[test]
    fn u16_underflow() {
        let serializer = BTHomeUnencryptedSerializer::new();
        let mut buffer = [0u8; 256];
        let res = serializer.serialize_to(BTHomeData::new().humidity(-1.0), &mut buffer);
        assert_eq!(res, Err(crate::BTHomeError::ValueUnderflow));
    }

    #[test]
    fn i16_underflow() {
        let serializer = BTHomeUnencryptedSerializer::new();
        let mut buffer = [0u8; 256];
        let res = serializer.serialize_to(BTHomeData::new().temperature(-1000.0), &mut buffer);
        assert_eq!(res, Err(crate::BTHomeError::ValueUnderflow));
    }
}
