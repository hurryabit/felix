#![allow(dead_code)]
use std::{borrow::Borrow, collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct Env<K, V> {
    data: HashMap<K, Vec<V>>,
}

impl<K, V> Env<K, V> {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl<K: Eq + Hash, V> Env<K, V> {
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.data.get(key).and_then(|values| values.last())
    }

    pub fn push(&mut self, key: K, value: V) {
        self.data.entry(key).or_insert_with(Vec::new).push(value);
    }

    pub fn pop<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        if let Some((key, mut values)) = self.data.remove_entry(key) {
            let value = values.pop();
            if !values.is_empty() {
                self.data.insert(key, values);
            }
            value
        } else {
            None
        }
    }
}

impl<K, V> Default for Env<K, V> {
    fn default() -> Self {
        Self {
            data: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_simple() {
        let mut env: Env<String, usize> = Env::new();
        assert_eq!(env.get("x"), None);

        env.push(String::from("x"), 1);
        assert_eq!(env.get("x"), Some(&1));

        assert_eq!(env.pop("x"), Some(1));
        assert_eq!(env.get("x"), None);

        assert_eq!(env.pop("x"), None);
    }

    #[test]
    fn env_shadowing() {
        let mut env: Env<String, usize> = Env::new();
        assert_eq!(env.get("x"), None);

        env.push(String::from("x"), 1);
        assert_eq!(env.get("x"), Some(&1));

        env.push(String::from("x"), 2);
        assert_eq!(env.get("x"), Some(&2));

        assert_eq!(env.pop("x"), Some(2));
        assert_eq!(env.get("x"), Some(&1));

        assert_eq!(env.pop("x"), Some(1));
        assert_eq!(env.get("x"), None);

        assert_eq!(env.pop("x"), None);
    }
}
