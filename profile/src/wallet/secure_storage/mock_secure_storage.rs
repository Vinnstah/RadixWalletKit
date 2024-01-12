use crate::prelude::*;

#[derive(Debug)]
pub struct MockSecureStorage {}
impl MockSecureStorage {
    pub fn new() -> Arc<Self> {
        Arc::new(MockSecureStorage {})
    }
}
impl SecureStorage for MockSecureStorage {
    fn load_data(&self, _key: SecureStorageKey) -> Result<Option<Vec<u8>>> {
        panic!("You have not installed any secure storage yet.")
    }

    fn save_data(&self, _key: SecureStorageKey, _value: Vec<u8>) -> Result<()> {
        panic!("You have not installed any secure storage yet.")
    }
}
