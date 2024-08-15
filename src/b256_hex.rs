use alloy_primitives::B256;
use serde::de::Error;
use serde::{Deserializer, Serializer};

use crate::hex::PrefixedHexVisitor;

pub fn serialize<S>(hash: &B256, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut hex_string: String = "0x".to_string();
    hex_string.push_str(&hex::encode(&hash));

    serializer.serialize_str(&hex_string)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<B256, D::Error>
where
    D: Deserializer<'de>,
{
    let decoded = deserializer.deserialize_str(PrefixedHexVisitor)?;

    if decoded.len() != 32 {
        return Err(D::Error::custom(format!(
            "expected {} bytes for array, got {}",
            32,
            decoded.len()
        )));
    }

    let mut array = [0; 32];
    array.copy_from_slice(&decoded);
    Ok(array.into())
}
