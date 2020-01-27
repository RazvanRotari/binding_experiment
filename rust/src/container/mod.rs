use error::BindError;
use traits::Deserializable;

pub mod map_iter;
pub mod seq_iter;
pub mod std;

pub trait MapAccess: Sized {
    fn get_keys<K: Deserializable>(&mut self) -> Vec<Result<K, BindError>> {
        unimplemented!()
    }

    fn get_value<V: Deserializable>(&mut self, _: &str) -> Result<Option<V>, BindError> {
        unimplemented!()
    }
}

pub trait SeqAccess: Sized {
    fn next_element<T: Deserializable>(&mut self) -> Result<Option<T>, BindError> {
        unimplemented!()
    }
}
