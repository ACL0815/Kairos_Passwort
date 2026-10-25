//! Opt-in live test: no pairing, entry creation, unlock, or configuration changes.
#![cfg(windows)]

use kairos_keepassxc::{
    Association, AssociationStore, Client, DpapiStore, Error, NamedPipeTransport,
};
use std::time::Duration;

struct ReadOnlyStore(Option<DpapiStore>);

impl AssociationStore for ReadOnlyStore {
    fn load(&mut self) -> Result<Option<Association>, Error> {
        match &mut self.0 {
            Some(store) => store.load(),
            None => Ok(None),
        }
    }
    fn save(&mut self, _: &Association) -> Result<(), Error> {
        Err(Error::Storage)
    }
}

#[test]
#[ignore = "Requires running KeePassXC with an unlocked disposable test DB and browser integration already enabled; see README"]
fn real_keepassxc_handshake_hash_and_optional_existing_association() {
    let path = std::env::var_os("KAIROS_KEEPASSXC_TEST_ASSOCIATION_FILE");
    let has_association = path.is_some();
    let store = ReadOnlyStore(path.map(DpapiStore::new));
    let transport = NamedPipeTransport::connect(Duration::from_secs(5)).unwrap();
    let mut client = Client::connect(transport, store).unwrap();
    let hash = client.get_databasehash().unwrap();
    assert!(!hash.is_empty());
    if has_association {
        assert_eq!(client.test_associate().unwrap(), hash);
    }
}
