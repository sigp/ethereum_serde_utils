use ethereum_types::U256;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S>(num: &U256, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    num.to_string().serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    U256::from_dec_str(&s).map_err(|e| de::Error::custom(format!("Invalid U256 string: {e}")))
}

#[cfg(test)]
mod test {
    use ethereum_types::U256;
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(transparent)]
    struct Wrapper {
        #[serde(with = "super")]
        val: U256,
    }

    #[test]
    fn encoding() {
        assert_eq!(
            &serde_json::to_string(&Wrapper { val: 0.into() }).unwrap(),
            "\"0\""
        );
        assert_eq!(
            &serde_json::to_string(&Wrapper { val: 1.into() }).unwrap(),
            "\"1\""
        );
        assert_eq!(
            &serde_json::to_string(&Wrapper { val: 256.into() }).unwrap(),
            "\"256\""
        );
        assert_eq!(
            &serde_json::to_string(&Wrapper { val: 65.into() }).unwrap(),
            "\"65\""
        );
        assert_eq!(
            &serde_json::to_string(&Wrapper { val: 1024.into() }).unwrap(),
            "\"1024\""
        );
        assert_eq!(
            &serde_json::to_string(&Wrapper {
                val: U256::max_value() - 1
            })
            .unwrap(),
            "\"115792089237316195423570985008687907853269984665640564039457584007913129639934\""
        );
        assert_eq!(
            &serde_json::to_string(&Wrapper {
                val: U256::max_value()
            })
            .unwrap(),
            "\"115792089237316195423570985008687907853269984665640564039457584007913129639935\""
        );
    }

    #[test]
    fn decoding() {
        assert_eq!(
            serde_json::from_str::<Wrapper>("\"0\"").unwrap(),
            Wrapper { val: 0.into() },
        );
        assert_eq!(
            serde_json::from_str::<Wrapper>("\"65\"").unwrap(),
            Wrapper { val: 65.into() },
        );
        assert_eq!(
            serde_json::from_str::<Wrapper>("\"1024\"").unwrap(),
            Wrapper { val: 1024.into() },
        );
        assert_eq!(
            serde_json::from_str::<Wrapper>(
                "\"115792089237316195423570985008687907853269984665640564039457584007913129639934\""
            )
            .unwrap(),
            Wrapper {
                val: U256::max_value() - 1
            },
        );
        assert_eq!(
            serde_json::from_str::<Wrapper>(
                "\"115792089237316195423570985008687907853269984665640564039457584007913129639935\""
            )
            .unwrap(),
            Wrapper {
                val: U256::max_value()
            },
        );
        serde_json::from_str::<Wrapper>("\"0x0\"").unwrap_err();
        serde_json::from_str::<Wrapper>("\"0x0400\"").unwrap_err();
        serde_json::from_str::<Wrapper>("\"-1\"").unwrap_err();
        serde_json::from_str::<Wrapper>("\"ff\"").unwrap_err();
    }
}
