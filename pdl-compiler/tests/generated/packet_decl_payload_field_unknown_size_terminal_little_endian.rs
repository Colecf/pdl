#![rustfmt::skip]
/// @generated rust packets from test.
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::{TryFrom, TryInto};
use std::cell::Cell;
use std::fmt;
use pdl_runtime::{Error, Packet};
type Result<T> = std::result::Result<T, Error>;
/// Private prevents users from creating arbitrary scalar values
/// in situations where the value needs to be validated.
/// Users can freely deref the value, but only the backend
/// may create it.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Private<T>(T);
impl<T> std::ops::Deref for Private<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FooDataChild {
    Payload(Bytes),
    None,
}
impl FooDataChild {
    fn get_total_size(&self) -> usize {
        match self {
            FooDataChild::Payload(bytes) => bytes.len(),
            FooDataChild::None => 0,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FooChild {
    Payload(Bytes),
    None,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FooData {
    a: u32,
    child: FooDataChild,
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Foo {
    #[cfg_attr(feature = "serde", serde(flatten))]
    foo: FooData,
}
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FooBuilder {
    pub a: u32,
    pub payload: Option<Bytes>,
}
impl FooData {
    fn conforms(bytes: &[u8]) -> bool {
        bytes.len() >= 3
    }
    fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        if bytes.get().remaining() < 3 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 3,
                got: bytes.get().remaining(),
            });
        }
        let payload = &bytes.get()[..bytes.get().len() - 3];
        bytes.get_mut().advance(payload.len());
        if bytes.get().remaining() < 3 {
            return Err(Error::InvalidLengthError {
                obj: "Foo".to_string(),
                wanted: 3,
                got: bytes.get().remaining(),
            });
        }
        let a = bytes.get_mut().get_uint_le(3) as u32;
        let child = match () {
            _ if !payload.is_empty() => {
                FooDataChild::Payload(Bytes::copy_from_slice(payload))
            }
            _ => FooDataChild::None,
        };
        Ok(Self { a, child })
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        match &self.child {
            FooDataChild::Payload(payload) => buffer.put_slice(payload),
            FooDataChild::None => {}
        }
        if self.a > 0xff_ffff {
            panic!("Invalid value for {}::{}: {} > {}", "Foo", "a", self.a, 0xff_ffff);
        }
        buffer.put_uint_le(self.a as u64, 3);
    }
    fn get_total_size(&self) -> usize {
        self.get_size()
    }
    fn get_size(&self) -> usize {
        3 + self.child.get_total_size()
    }
}
impl Packet for Foo {
    fn to_bytes(self) -> Bytes {
        let mut buffer = BytesMut::with_capacity(self.foo.get_size());
        self.foo.write_to(&mut buffer);
        buffer.freeze()
    }
    fn to_vec(self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }
}
impl From<Foo> for Bytes {
    fn from(packet: Foo) -> Self {
        packet.to_bytes()
    }
}
impl From<Foo> for Vec<u8> {
    fn from(packet: Foo) -> Self {
        packet.to_vec()
    }
}
impl Foo {
    pub fn parse(bytes: &[u8]) -> Result<Self> {
        let mut cell = Cell::new(bytes);
        let packet = Self::parse_inner(&mut cell)?;
        Ok(packet)
    }
    fn parse_inner(mut bytes: &mut Cell<&[u8]>) -> Result<Self> {
        let data = FooData::parse_inner(&mut bytes)?;
        Self::new(data)
    }
    pub fn specialize(&self) -> FooChild {
        match &self.foo.child {
            FooDataChild::Payload(payload) => FooChild::Payload(payload.clone()),
            FooDataChild::None => FooChild::None,
        }
    }
    fn new(foo: FooData) -> Result<Self> {
        Ok(Self { foo })
    }
    pub fn get_a(&self) -> u32 {
        self.foo.a
    }
    pub fn get_payload(&self) -> &[u8] {
        match &self.foo.child {
            FooDataChild::Payload(bytes) => &bytes,
            FooDataChild::None => &[],
        }
    }
    fn write_to(&self, buffer: &mut BytesMut) {
        self.foo.write_to(buffer)
    }
    pub fn get_size(&self) -> usize {
        self.foo.get_size()
    }
}
impl FooBuilder {
    pub fn build(self) -> Foo {
        let foo = FooData {
            a: self.a,
            child: match self.payload {
                None => FooDataChild::None,
                Some(bytes) => FooDataChild::Payload(bytes),
            },
        };
        Foo::new(foo).unwrap()
    }
}
impl From<FooBuilder> for Foo {
    fn from(builder: FooBuilder) -> Foo {
        builder.build().into()
    }
}
