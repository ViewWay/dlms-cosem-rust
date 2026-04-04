//! Key management for DLMS/COSEM

// no_std support

/// Key management for multiple encryption keys
pub struct KeyManagement {
    keys: Vec<(u8, [u8; 16])>,
}

impl KeyManagement {
    pub fn new() -> Self {
        Self { keys: Vec::new() }
    }

    pub fn set_key(&mut self, id: u8, key: [u8; 16]) {
        if let Some(entry) = self.keys.iter_mut().find(|(k, _)| *k == id) {
            entry.1 = key;
        } else {
            self.keys.push((id, key));
        }
    }

    pub fn get_key(&self, id: u8) -> Option<&[u8; 16]> {
        self.keys.iter().find(|(k, _)| *k == id).map(|(_, v)| v)
    }

    pub fn remove_key(&mut self, id: u8) {
        self.keys.retain(|(k, _)| *k != id);
    }

    pub fn key_count(&self) -> usize {
        self.keys.len()
    }

    pub fn key_ids(&self) -> Vec<u8> {
        self.keys.iter().map(|(k, _)| *k).collect()
    }
}

impl Default for KeyManagement {
    fn default() -> Self {
        Self::new()
    }
}
