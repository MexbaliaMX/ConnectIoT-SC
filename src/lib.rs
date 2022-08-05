use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::{env, near_bindgen};
use near_sdk::{serde_json, BorshStorageKey};
use std::collections::HashMap;

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    Device,
    Registry,
    Registries,
    DeviceData,
    DeviceMetadata,
    Custom { hash: Vec<u8> },
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Device {
    pub metadata: UnorderedMap<String, String>,
    pub data: UnorderedMap<String, String>,
    pub description: String,
    pub name: String,
}

impl Default for Device {
    fn default() -> Self {
        Self {
            metadata: UnorderedMap::new(StorageKeys::DeviceMetadata),
            data: UnorderedMap::new(StorageKeys::DeviceData),
            description: String::from(""),
            name: String::from(""),
        }
    }
}

impl Device {
    pub fn new(registry_name: String, name: String, description: String) -> Self {
        Self {
            description,
            metadata: UnorderedMap::new(StorageKeys::Custom {
                hash: env::sha256(format!("met_{}_{}", registry_name, name).as_bytes()),
            }),
            data: UnorderedMap::new(StorageKeys::Custom {
                hash: env::sha256(format!("dat_{}_{}", registry_name, name).as_bytes()),
            }),
            name,
        }
    }

    pub fn set_metadata(&mut self, metadata: HashMap<String, String>) {
        for (key, value) in &metadata {
            self.metadata.insert(key, value);
        }
    }

    pub fn get_metadata(&self) -> Vec<(String, String)> {
        self.metadata.to_vec()
    }

    pub fn set_data(&mut self, data: HashMap<String, String>) {
        for (key, value) in &data {
            self.data.insert(key, value);
        }
    }

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

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Registry {
    pub name: String,
    pub devices: LookupMap<String, Device>,
    pub owner_id: String,
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            name: String::from(""),
            devices: LookupMap::new(StorageKeys::Registry),
            owner_id: String::from(""),
        }
    }
}

impl Registry {
    pub fn new(name: String, owner_id: String) -> Self {
        Self {
            owner_id,
            devices: LookupMap::new(StorageKeys::Custom {
                hash: env::sha256(format!("reg{}", name).as_bytes()),
            }),
            name,
        }
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.insert(&device.name, &device);
    }

    pub fn get_device(&self, device_name: String) -> Device {
        self.devices.get(&device_name).unwrap()
    }

    pub fn exists(&self, device_name: String) -> bool {
        self.devices.get(&device_name).is_some()
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
            registries: LookupMap::new(StorageKeys::Registries),
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /* REGISTRY OPERATIONS */
    pub fn create_registry(&mut self, registry_name: String) -> bool {
        if self.validate_exists_registry(registry_name.clone()) {
            return false;
        }

        let new_registry = Registry::new(registry_name, env::signer_account_id().to_string());
        self.registries.insert(&new_registry.name, &new_registry);
        true
    }

    pub fn delete_registry(&mut self, registry_name: String) -> bool {
        if !self.validate_owner(registry_name.clone()) {
            return false;
        }
        self.registries.remove(&registry_name);
        true
    }

    /*pub fn get_registry(&self, registry_name: String) -> Registry {
        let current_registry = self.registries.get(&registry_name);
        current_registry.unwrap_or_default()
    }*/

    pub fn add_device_to_registry(
        &mut self,
        registry_name: String,
        device_name: String,
        description: String,
    ) -> bool {
        if !self.validate_registry(registry_name.clone()) {
            return false;
        }

        let mut current_registry = self.registries.get(&registry_name).unwrap();
        if self.validate_exists_device(&current_registry, device_name.clone()) {
            return false;
        }

        let new_device = Device::new(registry_name, device_name, description);
        current_registry
            .devices
            .insert(&new_device.name, &new_device);
        true
    }

    pub fn delete_device_from_registry(
        &mut self,
        registry_name: String,
        device_name: String,
    ) -> bool {
        if !self.validate_registry(registry_name.clone()) {
            return false;
        }

        let mut current_registry = self.registries.get(&registry_name).unwrap();
        if !self.validate_exists_device(&current_registry, device_name.clone()) {
            return false;
        }
        current_registry.devices.remove(&device_name);
        self.registries.insert(&registry_name, &current_registry);
        true
    }

    /* DEVICE OPERATIONS */

    // near view get_device_data '{registry_name: "dispositivos", device_id: "foco 1"}'
    pub fn get_device_data(
        &self,
        registry_name: String,
        device_name: String,
    ) -> Vec<(String, String)> {
        if !self.validate_registry(registry_name.clone()) {
            return Vec::new();
        }

        let current_registry = self.registries.get(&registry_name).unwrap();
        if !self.validate_exists_device(&current_registry, device_name.clone()) {
            return Vec::new();
        }
        let current_device = current_registry.devices.get(&device_name).unwrap();
        current_device.get_data()
    }

    pub fn get_device_metadata(
        &self,
        registry_name: String,
        device_name: String,
    ) -> Vec<(String, String)> {
        if !self.validate_registry(registry_name.clone()) {
            return Vec::new();
        }

        let current_registry = self.registries.get(&registry_name).unwrap();
        if !self.validate_exists_device(&current_registry, device_name.clone()) {
            return Vec::new();
        }
        let current_device = current_registry.devices.get(&device_name).unwrap();
        current_device.get_metadata()
    }

    pub fn get_device_data_param(
        &self,
        registry_name: String,
        device_name: String,
        param: String,
    ) -> String {
        if !self.validate_registry(registry_name.clone()) {
            return "Not registry or not allowed".to_string();
        }

        let current_registry = self.registries.get(&registry_name).unwrap();
        if !self.validate_exists_device(&current_registry, device_name.clone()) {
            return "Not device".to_string();
        }
        let current_device = current_registry.devices.get(&device_name).unwrap();
        current_device.get_data_param(param)
    }

    pub fn get_device_metadata_param(
        &self,
        registry_name: String,
        device_name: String,
        param: String,
    ) -> String {
        if !self.validate_registry(registry_name.clone()) {
            return "Not registry or not allowed".to_string();
        }

        let current_registry = self.registries.get(&registry_name).unwrap();
        if !self.validate_exists_device(&current_registry, device_name.clone()) {
            return "Not device".to_string();
        }
        let current_device = current_registry.devices.get(&device_name).unwrap();
        current_device.get_metadata_param(param)
    }

    pub fn set_device_data(&self, registry_name: String, device_name: String, data: String) {
        let mut current_registry = self.registries.get(&registry_name).unwrap();
        let mut current_device = current_registry.devices.get(&device_name).unwrap();
        let aux_map: HashMap<String, String> = serde_json::from_str(&data).unwrap();
        current_device.set_data(aux_map);
        current_registry.add_device(current_device);
    }

    pub fn set_device_metadata(
        &self,
        registry_name: String,
        device_name: String,
        metadata: String,
    ) -> bool {
        if !self.validate_registry(registry_name.clone()) {
            return false;
        }

        let mut current_registry = self.registries.get(&registry_name).unwrap();
        if !self.validate_exists_device(&current_registry, device_name.clone()) {
            return false;
        }
        let mut current_device = current_registry.devices.get(&device_name).unwrap();
        let aux_map: HashMap<String, String> = serde_json::from_str(&metadata).unwrap();
        current_device.set_metadata(aux_map);
        current_registry.add_device(current_device);
        true
    }

    pub fn set_device_data_param(
        &self,
        registry_name: String,
        device_name: String,
        param: String,
        value: String,
    ) -> bool {
        if !self.validate_registry(registry_name.clone()) {
            return false;
        }

        let mut current_registry = self.registries.get(&registry_name).unwrap();
        if !self.validate_exists_device(&current_registry, device_name.clone()) {
            return false;
        }
        let mut current_device = current_registry.devices.get(&device_name).unwrap();
        current_device.set_data_param(param, value);
        current_registry.add_device(current_device);
        true
    }

    pub fn set_device_metadata_param(
        &self,
        registry_name: String,
        device_name: String,
        param: String,
        value: String,
    ) -> bool {
        if !self.validate_registry(registry_name.clone()) {
            return false;
        }

        let mut current_registry = self.registries.get(&registry_name).unwrap();
        if !self.validate_exists_device(&current_registry, device_name.clone()) {
            return false;
        }
        let mut current_device = current_registry.devices.get(&device_name).unwrap();
        current_device.set_metadata_param(param, value);
        current_registry.add_device(current_device);
        true
    }

    #[private]
    fn validate_owner(&self, registry_name: String) -> bool {
        let registry = self.registries.get(&registry_name).unwrap();
        registry.owner_id == env::signer_account_id().to_string()
    }

    #[private]
    fn validate_exists_registry(&self, registry_name: String) -> bool {
        self.registries.get(&registry_name).is_some()
    }

    #[private]
    fn validate_registry(&self, registry_name: String) -> bool {
        self.validate_exists_registry(registry_name.clone())
            && self.validate_owner(registry_name.clone())
    }

    #[private]
    fn validate_exists_device(&self, registry: &Registry, device_name: String) -> bool {
        registry.devices.get(&device_name).is_some()
    }
}

/*-------------------UNIT TESTS -----------------------*/

/*
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;

    use near_sdk::{testing_env, VMContext};

    pub fn get_context(input: Vec<u32>, is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .current_account_id(near_sdk::AccountId::new_unchecked(
                "estebancumo4.testnet".to_string(),
            ))
            .signer_account_id("bob_near".parse().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    //#[result_serializer(borsh)]
    fn create_registry_test() {
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = Contract {};
        contract.create_registry();
        println!("Registro {:?} creado con éxito", contract.registries);
    }
    // ... Write test here
}
*/
