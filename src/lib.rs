use nanoid::nanoid;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::BorshStorageKey;
use near_sdk::{env, near_bindgen};

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Custom { hash: Vec<u8> },
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Device {
    pub id: String,
    pub metadata: UnorderedMap<String, String>,
    pub data: UnorderedMap<String, String>,
    pub description: String,
    pub name: String,
}

impl Default for Device {
    fn default() -> Self {
        let new_id = nanoid!();
        Self {
            id: new_id,
            metadata: UnorderedMap::new(StorageKeys::Custom {
                hash: env::sha256(format!("met{}", new_id).as_bytes()),
            }),
            data: UnorderedMap::new(StorageKeys::Custom {
                hash: env::sha256(format!("dat{}", new_id).as_bytes()),
            }),
            description: String::from(""),
            name: String::from(""),
        }
    }
}

impl Device {
    pub fn set_metadata(&self, metadata: UnorderedMap<String, String>) {}
    pub fn get_metadata(&self) -> UnorderedMap<String, String> {}
    pub fn set_data(&self, data: UnorderedMap<String, String>) {}
    pub fn get_data(&self) -> UnorderedMap<String, String> {}
    pub fn get_metadata_param(&self) -> String {}
    pub fn set_metadata_param(&self, param: String) {}
    pub fn get_data_param(&self) -> String {}
    pub fn set_data_param(&self, param: String) {}
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Registry {
    pub id: String,
    pub devices: LookupMap<String, Device>,
    pub owner_id: String,
}

impl Default for Registry {
    fn default() -> Self {
        let new_id = nanoid!();
        Self {
            id: new_id,
            devices: LookupMap::new(StorageKeys::Custom {
                hash: env::sha256(format!("reg{}", new_id).as_bytes()),
            }),
            owner_id: String::from(""),
        }
    }
}

impl Registry {
    pub fn add_device(&self, device: Device) {}
    pub fn get_device(&self) -> Device {}
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub registries: LookupMap<String, Registry>,
}

impl Default for Contract {
    fn default() -> Self {
        let new_id = nanoid!();
        Self {
            registries: LookupMap::new(b"registries".to_vec()),
        }
    }
}

impl Contract {
    pub fn create_registry(&self, registry_id: String) {}
    pub fn get_registry(&self) -> Registry {}
}
