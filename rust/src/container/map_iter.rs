#[allow(unused_imports)]
use std::collections::{btree_map, BTreeMap};
#[allow(unused_imports)]
use std::collections::{hash_map, HashMap};

use container::MapAccess;
use error::BindError;
use traits::Deserializable;
use types::Value;

#[cfg(not(feature = "sorted"))]
pub type Map<K, V> = HashMap<K, V>;
#[cfg(feature = "sorted")]
pub type Map<K, V> = BTreeMap<K, V>;

impl MapAccess for Map<String, Value> {
    fn get_keys<K: Deserializable>(&mut self) -> Vec<Result<K, BindError>> {
        self.keys().into_iter()
            // copy occurs
            .map(|s| K::unmarshal(s.clone()))
            .collect()
    }

    /*
        When retrieving values it drains the current map
    */
    fn get_value<V: Deserializable>(&mut self, key: &str) -> Result<Option<V>, BindError> {
        let deserializer: Option<&Value> = self.get(key);
        match deserializer {
            Some(value) => V::unmarshal(value)
                .map(Option::Some),
            None => Ok(None)
        }
    }
}


impl MapAccess for &Map<String, Value> {
    fn get_keys<K: Deserializable>(&mut self) -> Vec<Result<K, BindError>> {
        self.keys().into_iter()
            // copy occurs
            .map(|s| K::unmarshal(s.clone()))
            .collect()
    }

    /*
        When retrieving values it drains the current map
    */
    fn get_value<V: Deserializable>(&mut self, key: &str) -> Result<Option<V>, BindError> {
        let deserializer: Option<&Value> = self.get(key);
        match deserializer {
            Some(value) => V::unmarshal(value)
                .map(Option::Some),
            None => Ok(None)
        }
    }
}