#[cfg(feature = "encryption")]
pub mod encryption;

const TEMPERATURE_OBJECT_ID: u8 = 0x02;
const HUMIDITY_OBJECT_ID: u8 = 0x03;
const PM2_5_OBJECT_ID: u8 = 0x0d;
const PM10_OBJECT_ID: u8 = 0x0e;
const CO2_OBJECT_ID: u8 = 0x12;

#[derive(Debug, Clone, Copy)]
pub struct BTHomeData {
    pub temperature: Option<f32>,
    pub humidity: Option<f32>,
    pub co2: Option<u16>,
    pub pm2_5: Option<u16>,
    pub pm10: Option<u16>,
}

impl BTHomeData {
    pub fn new() -> BTHomeData {
        BTHomeData {
            temperature: None,
            humidity: None,
            co2: None,
            pm2_5: None,
            pm10: None,
        }
    }

    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }

    pub fn humidity(mut self, rh: f32) -> Self {
        self.humidity = Some(rh);
        self
    }

    pub fn co2(mut self, co2: u16) -> Self {
        self.co2 = Some(co2);
        self
    }

    pub fn pm2_5(mut self, pm2_5: u16) -> Self {
        self.pm2_5 = Some(pm2_5);
        self
    }

    pub fn pm10(mut self, pm10: u16) -> Self {
        self.pm10 = Some(pm10);
        self
    }
}

pub struct BTHomeUnencryptedSerializer {
}

impl BTHomeUnencryptedSerializer {
    pub fn new() -> BTHomeUnencryptedSerializer {
        BTHomeUnencryptedSerializer {}
    }

    pub fn serialize(self, data: BTHomeData) -> Result<Vec<u8>, TooManyValuesError> {
        let mut packet = Vec::new();

        // BTHome Device Info (Unencrypted v2)
        packet.push(0x40);

        packet.extend(Self::get_payload(data));
        // TODO find proper limit
        if packet.len() > 200 {
            return Err(TooManyValuesError)
        }

        Ok(packet)
    }

    fn get_payload(data: BTHomeData) -> Vec<u8> {
        let mut payload = Vec::new();

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

        payload
    }
}


pub const SERVICE_UUID: u16 = 0xFCD2;


#[derive(Debug, Clone)]
pub struct TooManyValuesError;
