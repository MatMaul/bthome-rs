use aes::{cipher::generic_array::GenericArray, Aes128};
use ccm::{consts::*, AeadInPlace, Ccm, KeyInit};
use tinyvec::{ArrayVec, SliceVec};

use crate::*;

type Aes128Ccm = Ccm<Aes128, U4, U13>;

pub struct BTHomeEncryptedSerializer {
    cipher: Aes128Ccm,
    mac_address: [u8; 6],
    counter: u32,
}

impl BTHomeEncryptedSerializer {
    pub fn new(
        encryption_key: [u8; 16],
        mac_address: [u8; 6],
        counter_seed: u32,
    ) -> BTHomeEncryptedSerializer {
        BTHomeEncryptedSerializer {
            cipher: Aes128Ccm::new(GenericArray::from_slice(&encryption_key)),
            mac_address: mac_address,
            counter: counter_seed,
        }
    }

    pub fn serialize_to(&mut self, data: BTHomeData, buffer: &mut [u8]) -> Result<usize> {
        // BTHome Device Info (Encrypted v2)
        buffer[0] = 0x41;

        let payload_size = add_payload(data, &mut buffer[1..])?;

        let mut nonce = ArrayVec::<[u8; 13]>::new();
        nonce.extend(self.mac_address);
        nonce.extend([0xD2, 0xFC]);
        nonce.push(0x41);
        nonce.extend(self.counter.to_le_bytes());

        let mic = self
            .cipher
            .encrypt_in_place_detached(
                nonce.as_slice().into(),
                &[],
                &mut buffer[1..payload_size + 1],
            )
            .map_err(|_| BTHomeError::Encrypt)?;

        let mut buffer = SliceVec::from(buffer);
        buffer.set_len(payload_size + 1);

        check_remaining_capacity(&buffer, 8)?;

        buffer.extend(self.counter.to_le_bytes());
        buffer.extend(mic);

        self.counter = self.counter.checked_add(1).unwrap_or(0);

        Ok(buffer.len())
    }

    #[cfg(feature = "std")]
    pub fn serialize(&mut self, data: BTHomeData) -> Result<std::vec::Vec<u8>> {
        let mut buffer = [0u8; 256];
        let size = self.serialize_to(data, &mut buffer)?;
        Ok(buffer[0..size].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: BTHomeData = BTHomeData::new()
        .temperature(18.6)
        .humidity(20.5)
        .co2(428)
        .pm2_5(49);
    const TEST_BYTES: [u8; 21] = [
        65, 75, 42, 32, 212, 185, 208, 237, 26, 140, 64, 158, 112, 100, 0, 0, 0, 168, 3, 96, 60,
    ];

    #[test]
    fn serialize() {
        let mut serializer = BTHomeEncryptedSerializer::new([1u8; 16], [2u8; 6], 100);
        let mut buffer = [0u8; 256];
        let size = serializer.serialize_to(TEST_DATA, &mut buffer).unwrap();
        assert_eq!(buffer[0..size], TEST_BYTES);
    }

    #[test]
    #[cfg(feature = "std")]
    fn serialize_std() {
        let mut serializer = BTHomeEncryptedSerializer::new([1u8; 16], [2u8; 6], 100);
        let bytes = serializer.serialize(TEST_DATA).unwrap();
        assert_eq!(bytes, TEST_BYTES);
    }

    #[test]
    fn encryption_overhead_buffer_overflow() {
        let mut serializer = BTHomeEncryptedSerializer::new([1u8; 16], [2u8; 6], 100);
        let mut buffer = [0u8; 14];
        let res = serializer.serialize_to(TEST_DATA, &mut buffer);
        assert_eq!(res, Err(crate::BTHomeError::BufferOverflow));
    }
}
