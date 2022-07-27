use std::collections::HashMap;

use nanoid::nanoid;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
//use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen};
use near_sdk::{serde_json, BorshStorageKey};

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
        let id = nanoid!().clone();
        Self {
            id: id.to_string(),
            metadata: UnorderedMap::new(StorageKeys::Custom {
                hash: env::sha256(format!("met{}", id).as_bytes()),
            }),
            data: UnorderedMap::new(StorageKeys::Custom {
                hash: env::sha256(format!("dat{}", id).as_bytes()),
            }),
            description: String::from(""),
            name: String::from(""),
        }
    }
}

impl Device {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            ..Default::default()
        }
    }

    /*
     * metadata: UnorderedMap<String, String>
     * { metadata: "algo" }
     * { metadata: "{ownerId: "asdasdas", version:"v 1.0", baterry: 80}" }
     */
    pub fn set_metadata(&mut self, metadata: HashMap<String, String>) {
        for (key, value) in &metadata {
            self.metadata.insert(key, value);
        }
    }

    /*
     * return UnorderedMap<String, String>
     */
    pub fn get_metadata(&self) -> Vec<(String, String)> {
        self.metadata.to_vec()
    }

    /*
     * data: UnorderedMap<String, String>
     */
    pub fn set_data(&mut self, data: HashMap<String, String>) {
        for (key, value) in &data {
            self.data.insert(key, value);
        }
    }

    /*
     * return UnorderedMap<String, String>
     */
    pub fn get_data(&self) -> Vec<(String, String)> {
        self.data.to_vec()
    }

    pub fn get_metadata_param(&self, param: String) -> String {
        self.metadata.get(&param).unwrap()
    }

    pub fn set_metadata_param(&mut self, param: String, value: String) {
        self.metadata.insert(&param, &value);
    }

    pub fn get_data_param(&self, param: String) -> String {
        self.data.get(&param).unwrap()
    }

    pub fn set_data_param(&mut self, param: String, value: String) {
        self.data.insert(&param, &value);
    }
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
        let id = nanoid!().clone();
        Self {
            id: id.to_string(),
            devices: LookupMap::new(StorageKeys::Custom {
                hash: env::sha256(format!("reg{}", id).as_bytes()),
            }),
            owner_id: String::from(""),
        }
    }
}

impl Registry {
    pub fn new(owner_id: String) -> Self {
        Self {
            owner_id,
            ..Default::default()
        }
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.insert(&device.id, &device);
    }

    pub fn get_device(&self, device_id: String) -> Device {
        self.devices.get(&device_id).unwrap()
    }

    pub fn exists(&self, device_id: String) -> bool {
        self.devices.get(&device_id).is_some()
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub registries: LookupMap<String, Registry>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            registries: LookupMap::new(b"registries".to_vec()),
        }
    }
}

impl Contract {
    /* REGISTRY OPERATIONS */
    pub fn create_registry(&mut self) -> String {
        let new_registry = Registry::new(env::signer_account_id());
        self.registries.insert(&new_registry.id, &new_registry);
        new_registry.id
    }

    pub fn get_registry(&self, registry_id: String) -> Registry {
        let current_registry = self.registries.get(&registry_id);
        current_registry.unwrap_or_default()
    }

    pub fn add_device_to_registry(
        &mut self,
        registry_id: String,
        name: String,
        description: String,
    ) {
        let new_device = Device::new(name, description);
        let mut current_registry = self.registries.get(&registry_id).unwrap();
        current_registry.devices.insert(&new_device.id, &new_device);
    }

    /* DEVICE OPERATIONS */

    // near view get_device_data '{registry_id: "dispositivos", device_id: "foco 1"}'
    pub fn get_device_data(&self, registry_id: String, device_id: String) -> Vec<(String, String)> {
        let current_registry = self.registries.get(&registry_id).unwrap();
        let current_device = current_registry.devices.get(&device_id).unwrap();
        current_device.get_data()
    }

    pub fn get_device_metadata(
        &self,
        registry_id: String,
        device_id: String,
    ) -> Vec<(String, String)> {
        let current_registry = self.registries.get(&registry_id).unwrap();
        let current_device = current_registry.devices.get(&device_id).unwrap();
        current_device.get_metadata()
    }

    pub fn get_device_data_param(
        &self,
        registry_id: String,
        device_id: String,
        param: String,
    ) -> String {
        let current_registry = self.registries.get(&registry_id).unwrap();
        let current_device = current_registry.devices.get(&device_id).unwrap();
        current_device.get_data_param(param)
    }

    pub fn get_device_metadata_param(
        &self,
        registry_id: String,
        device_id: String,
        param: String,
    ) -> String {
        let current_registry = self.registries.get(&registry_id).unwrap();
        let current_device = current_registry.devices.get(&device_id).unwrap();
        current_device.get_metadata_param(param)
    }

    pub fn set_device_data(&self, registry_id: String, device_id: String, data: String) {
        let current_registry = self.registries.get(&registry_id).unwrap();
        let mut current_device = current_registry.devices.get(&device_id).unwrap();
        let aux_map: HashMap<String, String> = serde_json::from_str(&data).unwrap();
        current_device.set_data(aux_map);
    }

    pub fn set_device_metadata(&self, registry_id: String, device_id: String, metadata: String) {
        let current_registry = self.registries.get(&registry_id).unwrap();
        let mut current_device = current_registry.devices.get(&device_id).unwrap();
        let aux_map: HashMap<String, String> = serde_json::from_str(&metadata).unwrap();
        current_device.set_metadata(aux_map);
    }

    pub fn set_device_data_param(
        &self,
        registry_id: String,
        device_id: String,
        param: String,
        value: String,
    ) {
        let current_registry = self.registries.get(&registry_id).unwrap();
        let mut current_device = current_registry.devices.get(&device_id).unwrap();
        current_device.set_data_param(param, value)
    }

    pub fn set_device_metadata_param(
        &self,
        registry_id: String,
        device_id: String,
        param: String,
        value: String,
    ) {
        let current_registry = self.registries.get(&registry_id).unwrap();
        let mut current_device = current_registry.devices.get(&device_id).unwrap();
        current_device.set_metadata_param(param, value)
    }
}
