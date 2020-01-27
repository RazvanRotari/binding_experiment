use std::marker::PhantomData;

use container::SeqAccess;
use error::BindError;
use traits::{Deserializable, Deserializer, Visitor};

struct VecVisitor<V: Deserializable> {
    __val: PhantomData<V>,
}

impl<V: Deserializable> VecVisitor<V> {
    fn new() -> Self {
        VecVisitor {
            __val: PhantomData,
        }
    }
}

impl<V: Deserializable> Visitor for VecVisitor<V> {
    type Value = Vec<V>;
    type Error = BindError;

    fn visit_seq<A: SeqAccess>(self, mut seq: A) -> Result<Self::Value, Self::Error> {
        let mut result = Vec::<V>::new();
        while let Some(item) = seq.next_element::<V>()? {
            result.push(item)
        }
        Ok(result)
    }
}

impl<V: Deserializable> Deserializable for Vec<V> {
    fn unmarshal<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
        deserializer.deserialize(VecVisitor::<V>::new())
    }
}