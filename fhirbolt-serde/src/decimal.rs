use std::marker::PhantomData;

use serde::ser::{Error, Impossible, Serialize, SerializeStruct, Serializer};

pub const TOKEN: &str = "$fhirbolt_serde::private::Decimal";

pub(crate) struct Decimal<'a> {
    pub d: &'a str,
}

impl Serialize for Decimal<'_> {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = tri!(serializer.serialize_struct(TOKEN, 1));
        tri!(s.serialize_field(TOKEN, &self.d));
        s.end()
    }
}

fn invalid_number<E: Error>() -> E {
    E::custom("invalid decimal")
}

pub struct SerializeDecimal<E: Error> {
    _phantom: PhantomData<E>,
    decimal: Option<String>,
}

impl<E: Error> SerializeDecimal<E> {
    pub fn new() -> Self {
        SerializeDecimal {
            _phantom: PhantomData,
            decimal: None,
        }
    }
}

impl<E: Error> SerializeStruct for SerializeDecimal<E> {
    type Ok = String;
    type Error = E;

    #[inline]
    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        if key == TOKEN {
            self.decimal = Some(tri!(value
                .serialize(DecimalStrEmitter::<Self::Error>::new())
                .map_err(Error::custom)));
            Ok(())
        } else {
            Err(Error::custom("expected decimal"))
        }
    }

    fn end(self) -> Result<String, Self::Error> {
        if let Some(value) = self.decimal {
            Ok(value)
        } else {
            Err(Error::custom("expected decimal"))
        }
    }
}

struct DecimalStrEmitter<E: Error> {
    _phantom: PhantomData<E>,
}

impl<E: Error> DecimalStrEmitter<E> {
    pub fn new() -> Self {
        DecimalStrEmitter {
            _phantom: PhantomData,
        }
    }
}

impl<E: Error> Serializer for DecimalStrEmitter<E> {
    type Ok = String;
    type Error = E;

    type SerializeSeq = Impossible<String, Self::Error>;
    type SerializeTuple = Impossible<String, Self::Error>;
    type SerializeTupleStruct = Impossible<String, Self::Error>;
    type SerializeTupleVariant = Impossible<String, Self::Error>;
    type SerializeMap = Impossible<String, Self::Error>;
    type SerializeStruct = Impossible<String, Self::Error>;
    type SerializeStructVariant = Impossible<String, Self::Error>;

    fn serialize_bool(self, _v: bool) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_i8(self, _v: i8) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_i16(self, _v: i16) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_i32(self, _v: i32) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_i64(self, _v: i64) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_u8(self, _v: u8) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_u16(self, _v: u16) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_u32(self, _v: u32) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_u64(self, _v: u64) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_f32(self, _v: f32) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_f64(self, _v: f64) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_char(self, _v: char) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_str(self, value: &str) -> Result<String, Self::Error> {
        Ok(value.to_owned())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_none(self) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_some<T>(self, _value: &T) -> Result<String, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(invalid_number())
    }

    fn serialize_unit(self) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<String, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<String, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(invalid_number())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<String, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(invalid_number())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(invalid_number())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(invalid_number())
    }
}
