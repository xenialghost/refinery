use serde::{Serialize, Deserialize};
use uuid::Uuid;
use rmp_serde::{encode, decode};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Reading {
    device_id: Uuid,
    reading: f64
}

impl From<&Reading> for Vec<u8> {
    fn from(value: &Reading) -> Self {
        encode::to_vec(value).unwrap()
    }
}

impl TryFrom<&[u8]> for Reading {
    type Error = decode::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        decode::from_slice(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_encoding() {
        let input_reading: Reading = Reading { device_id: Uuid::new_v4(), reading: 12.0 };

        assert_eq!(
            rmp_serde::to_vec(&(input_reading.device_id, input_reading.reading)).unwrap(),
            rmp_serde::to_vec(&input_reading).unwrap()
        );
    }

    #[test]
    fn test_valid_decoding() {
        let input_reading: Reading = Reading { device_id: Uuid::new_v4(), reading: 12.0 };

        let expected_buf: Vec<u8> = rmp_serde::to_vec(&(input_reading.device_id, input_reading.reading)).unwrap();

        assert_eq!(
            input_reading,
            rmp_serde::from_slice(&expected_buf).unwrap()
        );
    }
}