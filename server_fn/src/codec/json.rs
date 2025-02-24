use crate::{Decodes, Encodes};
use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};

struct Json;

impl<T> Encodes<T> for Json
where
    T: Serialize,
{
    type Error = serde_json::Error;
    const CONTENT_TYPE: &'static str = "application/json";

    fn encode(output: T) -> Result<Bytes, Self::Error> {
        serde_json::to_vec(&output).map(Bytes::from)
    }
}

impl<T> Decodes<T> for Json
where
    T: DeserializeOwned,
{
    type Error = serde_json::Error;

    fn decode(bytes: Bytes) -> Result<T, Self::Error> {
        serde_json::from_slice(&bytes)
    }
}
