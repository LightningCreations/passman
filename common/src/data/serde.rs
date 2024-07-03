use base64::{prelude::BASE64_STANDARD_NO_PAD, Engine};
use serde::{
    de::{Deserialize, Deserializer, Error, SeqAccess, Unexpected, Visitor},
    ser::{Serialize, Serializer},
};

use super::{Bytes, Version};

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let engine = BASE64_STANDARD_NO_PAD;
            let val = engine.encode(self);
            serializer.serialize_str(&val)
        } else {
            serializer.serialize_bytes(self)
        }
    }
}

impl<'de> Deserialize<'de> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        pub struct BytesVisitor;

        impl<'de> Visitor<'de> for BytesVisitor {
            type Value = Bytes;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a byte sequence")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                // We allow padded or unpadded base64, stirp the padding
                let v = v.trim_end_matches("=");
                let engine = BASE64_STANDARD_NO_PAD;
                engine
                    .decode(v)
                    .map_err(|e| {
                        E::invalid_value(
                            Unexpected::Str(v),
                            &"a base64 encoded string using the standard alphabet",
                        )
                    })
                    .map(Bytes)
            }

            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Bytes::from(v))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Bytes::from(v))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut v = Vec::with_capacity(seq.size_hint().unwrap_or(0));
                while let Some(b) = seq.next_element()? {
                    v.push(b);
                }
                Ok(Bytes::new(v))
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_str(BytesVisitor)
        } else {
            deserializer.deserialize_byte_buf(BytesVisitor)
        }
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let st = self.to_string();
            serializer.serialize_str(&st)
        } else {
            serializer.serialize_u32(self.encoded())
        }
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        pub struct VersionVisitor;

        impl<'de> Visitor<'de> for VersionVisitor {
            type Value = Version;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a version")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                v.parse()
                    .map_err(|_| E::invalid_value(Unexpected::Str(v), &self))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let v = v.try_into().map_err(|_| {
                    E::invalid_value(Unexpected::Unsigned(v), &"An integer that is a valid u32")
                })?;

                Ok(Version::from_encoded(v))
            }
        }

        if deserializer.is_human_readable() {
            deserializer.deserialize_str(VersionVisitor)
        } else {
            deserializer.deserialize_u32(VersionVisitor)
        }
    }
}
