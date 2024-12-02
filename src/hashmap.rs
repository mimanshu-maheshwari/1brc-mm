pub struct HashMap<V> {
    data: Vec<Option<(String, V)>>,
    capacity: usize,
}

impl<V: Clone> HashMap<V> {
    pub fn with_capacity(n: usize) -> Self {
        Self {
            data: vec![None; n],
            capacity: n,
        }
    }

    fn hash_(&self, key: &[u8]) -> usize {
        (((key.iter().map(|&v| v as usize).sum::<usize>() + 7) * 5) + 31) % self.capacity
    }

    fn hash(&self, key: &str) -> usize {
        (((key.bytes().map(|v| v as usize).sum::<usize>() + 7) * 5) + 31) % self.capacity
    }

    pub fn insert(&mut self, key: String, value: V) {
        let hash = self.hash(&key);
        if !self.contains_key(&key) {
            self.data[hash] = Some((key, value));
        } else {
            println!(
                "Conflict with keys exists({}) input({})",
                self.data[hash].as_ref().unwrap().0,
                key
            );
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        let hash = self.hash(key);
        self.data[hash].is_some()
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        let hash = self.hash(key);
        if let Some(val) = self.data.get(hash) {
            if val.is_none() {
                return None;
            }
            Some(&val.as_ref().unwrap().1)
        } else {
            None
        }
    }

    pub fn get_mut_(&mut self, key: &[u8]) -> Option<&mut V> {
        let hash = self.hash_(key);
        if let Some(val) = self.data.get_mut(hash) {
            if val.is_none() {
                return None;
            }
            Some(&mut val.as_mut().unwrap().1)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        let hash = self.hash(key);
        if let Some(val) = self.data.get_mut(hash) {
            Some(&mut val.as_mut().unwrap().1)
        } else {
            None
        }
    }
}

pub struct HashMapIter<'a, V> {
    data: &'a [Option<(String, V)>],
    index: usize,
}

impl<V> HashMap<V> {
    pub fn iter(&self) -> HashMapIter<V> {
        HashMapIter {
            data: &self.data,
            index: 0,
        }
    }
}

impl<'a, V> Iterator for HashMapIter<'a, V> {
    type Item = (&'a String, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.data.len() {
            if let Some((key, value)) = &self.data[self.index] {
                self.index += 1; // Move to the next element
                return Some((key, value));
            }
            self.index += 1; // Skip None and move to the next element
        }
        None // No more elements
    }
}
