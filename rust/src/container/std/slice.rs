use std::marker::PhantomData;

use container::SeqAccess;
use error::BindError;
use traits::{Deserializable, Deserializer, Visitor};

struct SliceVisitor<V: Deserializable> {
    __val: PhantomData<V>,
}

impl<V: Deserializable> SliceVisitor<V> {
    fn new() -> Self {
        SliceVisitor {
            __val: PhantomData,
        }
    }
}

impl<V: Deserializable> Visitor for SliceVisitor<V> {
    type Value = Box<[V]>;
    type Error = BindError;

    fn visit_seq<A: SeqAccess>(self, mut seq: A) -> Result<Self::Value, Self::Error> {
        let mut result = Vec::<V>::new();
        while let Ok(Some(item)) = seq.next_element::<V>() {
            result.push(item)
        }
        Ok(result.into_boxed_slice())
    }
}

impl<V: Deserializable> Deserializable for Box<[V]> {
    fn unmarshal<D: Deserializer>(deserializer: D) -> Result<Self, BindError> {
        deserializer.deserialize(SliceVisitor::<V>::new())
    }
}
