use std::slice::Iter;
use std::vec::IntoIter;

use container::SeqAccess;
use error::BindError;
use traits::Deserializable;
use types::Value;

impl SeqAccess for Iter<'_, Value> {
    fn next_element<V: Deserializable>(&mut self) -> Result<Option<V>, BindError> {
        match self.next() {
            Some(value) => V::unmarshal(value)
                .map(Option::Some),
            None => Ok(None)
        }
    }
}

impl SeqAccess for IntoIter<Value> {
    fn next_element<V: Deserializable>(&mut self) -> Result<Option<V>, BindError> {
        match self.next() {
            Some(value) => V::unmarshal(value)
                .map(Option::Some),
            None => Ok(None)
        }
    }
}