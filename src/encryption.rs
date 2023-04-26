use aes::{cipher::generic_array::GenericArray, Aes128};
use ccm::{aead::Aead, consts::*, Ccm, KeyInit};

use crate::{BTHomeData, BTHomeUnencryptedSerializer, TooManyValuesError};

type Aes128Ccm = Ccm<Aes128, U4, U13>;

pub struct BTHomeEncryptedSerializer {
    cipher: Aes128Ccm,
    mac_address: [u8; 6],
    counter: u32,
}

impl BTHomeEncryptedSerializer {
    pub fn new(encryption_key: [u8; 16], mac_address: [u8; 6], counter_seed: u32) -> BTHomeEncryptedSerializer {
        BTHomeEncryptedSerializer {
                cipher: Aes128Ccm::new(GenericArray::from_slice(&encryption_key)),
                mac_address: mac_address,
                counter: counter_seed,
            }
    }

    pub fn serialize(&mut self, data: BTHomeData) -> Result<Vec<u8>, TooManyValuesError> {
        let mut packet: Vec<u8> = Vec::new();

        // BTHome Device Info (Encrypted v2)
        packet.push(0x41);

        let payload = BTHomeUnencryptedSerializer::get_payload(data);

        // TODO find proper limit
        if payload.len() > 200 {
            return Err(TooManyValuesError)
        }

        let mut nonce: Vec<u8> = Vec::new();
        nonce.extend(self.mac_address);
        nonce.extend([0xD2, 0xFC]);
        // TODO test nonce.extend([0xFC, 0xD2]);
        nonce.push(0x41);
        nonce.extend(self.counter.to_le_bytes());

        let ciphertext_mic = self.cipher
            .encrypt(nonce.as_slice().into(), payload.as_ref())
            .unwrap();
        let (ciphertext, mic) = ciphertext_mic.split_at(ciphertext_mic.len() - 4);

        packet.extend(ciphertext);
        packet.extend(self.counter.to_le_bytes());
        packet.extend(mic);

        self.counter = self.counter.checked_add(1).unwrap_or(0);

        Ok(packet)
    }
}

#[cfg(test)]
mod tests {
    use crate::BTHomeData;

    #[test]
    fn test_encryption() {
        let test_data = BTHomeData::new().co2(428).humidity(20.5).pm2_5(49);
        let mut serializer = super::BTHomeEncryptedSerializer::new([1u8; 16], [2u8; 6], 100);
        let bytes = serializer.serialize(test_data).unwrap();
        assert_eq!(bytes, [65, 74, 108, 47, 218, 138, 216, 242, 135, 141, 100, 0, 0, 0, 249, 120, 189, 74]);
    }
}
