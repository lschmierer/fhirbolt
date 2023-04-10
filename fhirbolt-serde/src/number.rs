use std::{error, fmt, result};

use serde::ser::{self, Impossible, Serialize};

pub const NUMBER_TOKEN: &str = "$serde_json::private::Number";

#[derive(Debug)]
pub struct Error(String);
type Result<T> = result::Result<T, Error>;

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error(msg.to_string())
    }
}

pub struct NumberValueEmitter;

impl serde::ser::Serializer for NumberValueEmitter {
    type Ok = String;
    type Error = Error;

    type SerializeSeq = Impossible<String, Self::Error>;
    type SerializeTuple = Impossible<String, Self::Error>;
    type SerializeTupleStruct = Impossible<String, Self::Error>;
    type SerializeTupleVariant = Impossible<String, Self::Error>;
    type SerializeMap = Impossible<String, Self::Error>;
    type SerializeStruct = Impossible<String, Self::Error>;
    type SerializeStructVariant = Impossible<String, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<String> {
        unimplemented!()
    }

    fn serialize_i8(self, _v: i8) -> Result<String> {
        unimplemented!()
    }

    fn serialize_i16(self, _v: i16) -> Result<String> {
        unimplemented!()
    }

    fn serialize_i32(self, _v: i32) -> Result<String> {
        unimplemented!()
    }

    fn serialize_i64(self, _v: i64) -> Result<String> {
        unimplemented!()
    }

    fn serialize_u8(self, _v: u8) -> Result<String> {
        unimplemented!()
    }

    fn serialize_u16(self, _v: u16) -> Result<String> {
        unimplemented!()
    }

    fn serialize_u32(self, _v: u32) -> Result<String> {
        unimplemented!()
    }

    fn serialize_u64(self, _v: u64) -> Result<String> {
        unimplemented!()
    }

    fn serialize_f32(self, _v: f32) -> Result<String> {
        unimplemented!()
    }

    fn serialize_f64(self, _v: f64) -> Result<String> {
        unimplemented!()
    }

    fn serialize_char(self, _v: char) -> Result<String> {
        unimplemented!()
    }

    fn serialize_str(self, value: &str) -> Result<String> {
        Ok(value.to_owned())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<String> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<String> {
        unimplemented!()
    }

    fn serialize_some<T>(self, _value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<String> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<String> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<String> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<String>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        unimplemented!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        unimplemented!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}
