use aes::{cipher::generic_array::GenericArray, Aes128};
use ccm::{consts::*, AeadInPlace, Ccm, KeyInit};
use tinyvec::{ArrayVec, SliceVec};

use crate::*;

type Aes128Ccm = Ccm<Aes128, U4, U13>;

pub struct CryptoEngine {
    cipher: Aes128Ccm,
    mac_address: [u8; 6],
    counter: u32,
}

impl CryptoEngine {
    pub fn new(encryption_key: [u8; 16], mac_address: [u8; 6], counter_seed: u32) -> CryptoEngine {
        CryptoEngine {
            cipher: Aes128Ccm::new(GenericArray::from_slice(&encryption_key)),
            mac_address: mac_address,
            counter: counter_seed,
        }
    }
}

impl BTHomeData {
    pub fn to_encrypted_slice(
        self,
        crypto_engine: &mut CryptoEngine,
        buffer: &mut [u8],
    ) -> Result<usize> {
        // BTHome Device Info (Encrypted v2)
        buffer[0] = 0x41;

        let payload_size = add_payload(self, &mut buffer[1..])?;

        let mut nonce = ArrayVec::<[u8; 13]>::new();
        nonce.extend(crypto_engine.mac_address);
        nonce.extend([0xD2, 0xFC]);
        nonce.push(0x41);
        nonce.extend(crypto_engine.counter.to_le_bytes());

        let mic = crypto_engine
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

        buffer.extend(crypto_engine.counter.to_le_bytes());
        buffer.extend(mic);

        crypto_engine.counter = crypto_engine.counter.checked_add(1).unwrap_or(0);

        Ok(buffer.len())
    }

    #[cfg(feature = "std")]
    pub fn to_encrypted_vec(self, crypto_engine: &mut CryptoEngine) -> Result<std::vec::Vec<u8>> {
        let mut buffer = [0u8; 256];
        let size = self.to_encrypted_slice(crypto_engine, &mut buffer)?;
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
        let mut crypto_engine = CryptoEngine::new([1u8; 16], [2u8; 6], 100);
        let mut buffer = [0u8; 256];
        let size = TEST_DATA
            .to_encrypted_slice(&mut crypto_engine, &mut buffer)
            .unwrap();
        assert_eq!(buffer[0..size], TEST_BYTES);
    }

    #[test]
    #[cfg(feature = "std")]
    fn serialize_std() {
        let mut crypto_engine = CryptoEngine::new([1u8; 16], [2u8; 6], 100);
        let bytes = TEST_DATA.to_encrypted_vec(&mut crypto_engine).unwrap();
        assert_eq!(bytes, TEST_BYTES);
    }

    #[test]
    fn encryption_overhead_buffer_overflow() {
        let mut crypto_engine = CryptoEngine::new([1u8; 16], [2u8; 6], 100);
        let mut buffer = [0u8; 14];
        let res = TEST_DATA.to_encrypted_slice(&mut crypto_engine, &mut buffer);
        assert_eq!(res, Err(crate::BTHomeError::BufferOverflow));
    }
}
