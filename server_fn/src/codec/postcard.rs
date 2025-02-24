use crate::{Decodes, Encodes};
use bytes::Bytes;
use serde::{de::DeserializeOwned, Serialize};

/// A codec for Postcard.
pub struct Postcard;

impl<T> Encodes<T> for Postcard
where
    T: Serialize,
{
    type Error = postcard::Error;
    const CONTENT_TYPE: &'static str = "application/x-postcard";

    fn encode(value: T) -> Result<Bytes, Self::Error> {
        postcard::to_allocvec(&value).map(|bytes| Bytes::from(bytes))
    }
}

impl<T> Decodes<T> for Postcard
where
    T: DeserializeOwned,
{
    type Error = postcard::Error;

    fn decode(bytes: Bytes) -> Result<T, Self::Error> {
        postcard::from_bytes(&bytes)
    }
}
