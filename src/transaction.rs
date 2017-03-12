use std::collections::BTreeMap;
use std::cmp::Ord;
use error::Result;

pub trait ReadTransaction<K, V>
    where K: Ord
{
    fn get(&self, key: K) -> Option<&V>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub trait WriteTransaction<K, V>: ReadTransaction<K, V>
    where K: Ord
{
    fn update(&mut self, key: K, value: V) -> Result<Option<V>>;
    fn remove(&mut self, key: K) -> Result<Option<V>>;
    fn remove_all(&mut self) -> Result<()>;
}

#[derive(Debug)]
pub struct Transaction<K: Ord, V> {
    pub store: Box<BTreeMap<K, V>>,
}

impl<K, V> Transaction<K, V>
    where K: Ord
{
    pub fn new(store: Box<BTreeMap<K, V>>) -> Transaction<K, V> {
        Transaction { store: store }
    }

    fn commit() -> Result<()> {
        unimplemented!()
    }

    fn rollback() -> Result<()> {
        unimplemented!()
    }
}

impl<K, V> ReadTransaction<K, V> for Transaction<K, V>
    where K: Ord
{
    fn get(&self, key: K) -> Option<&V> {
        self.store.get(&key)
    }

    fn len(&self) -> usize {
        self.store.len()
    }

    fn is_empty(&self) -> bool {
        self.store.is_empty()
    }
}

impl<K, V> WriteTransaction<K, V> for Transaction<K, V>
    where K: Ord
{
    fn update(&mut self, key: K, value: V) -> Result<Option<V>> {
        Ok(self.store.as_mut().insert(key, value))
    }

    fn remove(&mut self, key: K) -> Result<Option<V>> {
        Ok(self.store.as_mut().remove(&key))
    }

    fn remove_all(&mut self) -> Result<()> {
        Ok(self.store.as_mut().clear())
    }
}
