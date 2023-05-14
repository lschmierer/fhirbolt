use std::marker::PhantomData;

use serde::ser::{Error, Impossible, Serialize, Serializer};

pub enum EitherIter<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Iterator for EitherIter<L, R>
where
    L: Iterator,
    R: Iterator<Item = L::Item>,
{
    type Item = L::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EitherIter::Left(i) => i.next(),
            EitherIter::Right(i) => i.next(),
        }
    }
}

fn expected_a_string<E: Error>() -> E {
    Error::custom("expected a string")
}

pub struct StrSerializer<E: Error> {
    _phantom: PhantomData<E>,
}

impl<E: Error> StrSerializer<E> {
    pub fn new() -> Self {
        StrSerializer {
            _phantom: PhantomData,
        }
    }
}

impl<E: Error> Serializer for StrSerializer<E> {
    type Ok = String;
    type Error = E;

    type SerializeSeq = Impossible<String, Self::Error>;
    type SerializeTuple = Impossible<String, Self::Error>;
    type SerializeTupleStruct = Impossible<String, Self::Error>;
    type SerializeTupleVariant = Impossible<String, Self::Error>;
    type SerializeMap = Impossible<String, Self::Error>;
    type SerializeStruct = Impossible<String, Self::Error>;
    type SerializeStructVariant = Impossible<String, Self::Error>;

    #[inline]
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<String, Self::Error> {
        Err(expected_a_string())
    }

    #[inline]
    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<String, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(expected_a_string())
    }

    fn serialize_bool(self, _value: bool) -> Result<String, Self::Error> {
        Err(expected_a_string())
    }

    #[inline]
    fn serialize_i8(self, _value: i8) -> Result<String, Self::Error> {
        Err(expected_a_string())
    }

    #[inline]
    fn serialize_i16(self, _value: i16) -> Result<String, Self::Error> {
        Err(expected_a_string())
    }

    #[inline]
    fn serialize_i32(self, _value: i32) -> Result<String, Self::Error> {
        Err(expected_a_string())
    }

    #[inline]
    fn serialize_i64(self, _value: i64) -> Result<String, Self::Error> {
        Err(expected_a_string())
    }

    #[inline]
    fn serialize_u8(self, _value: u8) -> Result<String, Self::Error> {
        Err(expected_a_string())
    }

    #[inline]
    fn serialize_u16(self, _value: u16) -> Result<String, Self::Error> {
        Err(expected_a_string())
    }

    #[inline]
    fn serialize_u32(self, _value: u32) -> Result<String, Self::Error> {
        Err(expected_a_string())
    }

    #[inline]
    fn serialize_u64(self, _value: u64) -> Result<String, Self::Error> {
        Err(expected_a_string())
    }

    fn serialize_f32(self, _value: f32) -> Result<String, Self::Error> {
        Err(expected_a_string())
    }

    fn serialize_f64(self, _value: f64) -> Result<String, Self::Error> {
        Err(expected_a_string())
    }

    fn serialize_char(self, value: char) -> Result<String, Self::Error> {
        let mut s = String::new();
        s.push(value);
        Ok(s)
    }

    #[inline]
    fn serialize_str(self, value: &str) -> Result<String, Self::Error> {
        Ok(value.to_string())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<String, Self::Error> {
        Err(expected_a_string())
    }

    fn serialize_unit(self) -> Result<String, Self::Error> {
        Err(expected_a_string())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<String, Self::Error> {
        Err(expected_a_string())
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
        Err(expected_a_string())
    }

    fn serialize_none(self) -> Result<String, Self::Error> {
        Err(expected_a_string())
    }

    fn serialize_some<T>(self, _value: &T) -> Result<String, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        Err(expected_a_string())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(expected_a_string())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(expected_a_string())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(expected_a_string())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(expected_a_string())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(expected_a_string())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(expected_a_string())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(expected_a_string())
    }
}
