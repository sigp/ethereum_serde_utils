use alloy_primitives::Address;
use serde::de::Error;
use serde::{Deserializer, Serializer};

use crate::hex::PrefixedHexVisitor;

pub fn serialize<S>(address: &Address, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut hex_string: String = "0x".to_string();
    hex_string.push_str(&hex::encode(&address));

    serializer.serialize_str(&hex_string)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Address, D::Error>
where
    D: Deserializer<'de>,
{
    let decoded = deserializer.deserialize_str(PrefixedHexVisitor)?;

    if decoded.len() != 20 {
        return Err(D::Error::custom(format!(
            "expected {} bytes for array, got {}",
            20,
            decoded.len()
        )));
    }

    let mut array = [0; 20];
    array.copy_from_slice(&decoded);
    Ok(array.into())
}
