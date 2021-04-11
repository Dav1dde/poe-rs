use serde::de::{self, MapAccess, SeqAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;

pub(crate) fn empty_array_is_map<'de, D, K, V>(deserializer: D) -> Result<HashMap<K, V>, D::Error>
where
    D: Deserializer<'de>,
    K: Deserialize<'de> + Eq + std::hash::Hash,
    V: Deserialize<'de>,
{
    struct ArrMapVisitor<K, V>(PhantomData<fn() -> (K, V)>);

    impl<'de, K, V> Visitor<'de> for ArrMapVisitor<K, V>
    where
        K: Deserialize<'de> + Eq + std::hash::Hash,
        V: Deserialize<'de>,
    {
        type Value = HashMap<K, V>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("expected a map or empty array")
        }

        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            if let Ok(None) = seq.next_element::<K>() {
                Ok(HashMap::new())
            } else {
                Err(de::Error::invalid_type(Unexpected::Seq, &self))
            }
        }

        fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(ArrMapVisitor(PhantomData))
}

pub(crate) fn string_or_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    struct NumVisitor;

    impl<'de> Visitor<'de> for NumVisitor {
        type Value = u32;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("expected a number")
        }

        fn visit_str<E: de::Error>(self, value: &str) -> Result<u32, E> {
            value.parse().map_err(de::Error::custom)
        }

        fn visit_u64<E: de::Error>(self, number: u64) -> Result<u32, E> {
            Ok(number as u32)
        }
    }

    deserializer.deserialize_any(NumVisitor)
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Deserialize)]
    struct EmptyArr {
        #[serde(deserialize_with = "empty_array_is_map")]
        foo: HashMap<String, u32>,
    }

    #[test]
    fn empty_array_is_map_empty_array() {
        let a = serde_json::from_str::<EmptyArr>("{\"foo\": []}").unwrap();
        assert_eq!(HashMap::new(), a.foo);
    }

    #[test]
    fn empty_array_is_map_array_with_element() {
        let a = serde_json::from_str::<EmptyArr>("{\"foo\": [\"item\"]}");
        assert!(a.is_err());
    }

    #[test]
    fn empty_array_is_map_empty_map() {
        let a = serde_json::from_str::<EmptyArr>("{\"foo\": {}}").unwrap();
        assert_eq!(HashMap::new(), a.foo);
    }

    #[test]
    fn empty_array_is_map_map_with_elements() {
        let a = serde_json::from_str::<EmptyArr>("{\"foo\": {\"1\": 1, \"2\": 22}}").unwrap();
        let mut r = HashMap::new();
        r.insert("1".to_string(), 1);
        r.insert("2".to_string(), 22);
        assert_eq!(r, a.foo);
    }

    #[derive(Deserialize)]
    struct StringU32 {
        #[serde(deserialize_with = "string_or_u32")]
        foo: u32,
    }

    #[test]
    fn string_or_u32_string() {
        let a = serde_json::from_str::<StringU32>("{\"foo\": \"12\"}").unwrap();
        assert_eq!(12, a.foo);
    }

    #[test]
    fn string_or_u32_u32() {
        let a = serde_json::from_str::<StringU32>("{\"foo\": 12}").unwrap();
        assert_eq!(12, a.foo);
    }
}
