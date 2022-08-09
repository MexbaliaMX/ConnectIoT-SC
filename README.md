# **ConnectIoT**

[![Mexbalia](https://img.shields.io/badge/Mexbalia-ConnectIoT%20Smart%20Contract-blue)](https://mexbalia.com/connect-iot/)
[![Mexbalia](https://img.shields.io/badge/-Near%20Protocol-9cf)](https://near.org/)
[![Mexbalia](https://img.shields.io/badge/-near--sdk--rs-orange)](https://www.near-sdk.io/)

#### Access services **Smart Contract** which allows creating the **connection** between **IoT** devices and the **Near Protocol Blockchain**.


### Features

- Create a registry for a device group
- Add device to registry
- Set/Get device data
- Set/Get device metadata
- Set/Get device data parmeters
- Set/Get device metadata parmeters


### Prerequesites

1. Install rustup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  
```

*Visit https://www.rust-lang.org/tools/install for more info.*

2. Install node.js >=12 (https://nodejs.org)
   
3. Install Near-CLI

```bash
npm i near-cli
 ```
4. Crate a Near testnet account in  (https://wallet.testnet.near.org/)
5. Account access

```bash
near login
  ```
6. Clone the project

```bash
git clone https://github.com/EbanCuMo/Connect-IoT.git
```
7. Go to the project directory

```bash
cd Connect-IoT
```
8. Compile release version
```bash
cargo build --target wasm32-unknown-unknown --release
```


### Deployment
*This is optional, the smart contract is already deployed
you can check the contract ID in the .env file.*

To deploy this smart contract run
```bash
near dev-deploy ./target/wasm32-unknown-unknown/release/connect_iot.wasm
```


# Trying ConnectIoT

*Before using the smart contract, for a better and easy usage you can set an **envorionment variable** 
for **CONTRACT_ACCOUNT_ID**.*

### Environment variable

```bash
export CONTRACT_ACCOUNT_ID = <ID in .env file>
```


### Create a registry for a device group.

```bash
near call $CONTRACT_ACCOUNT_ID create_registry '{"registry name":"REGISTRY_NAME"}' --acoountId OWNER_ACCOUNT_ID
```
### Add a device to an existing registry

```bash
near call $CONTRACT_ACCOUNT_ID add_device_to_registry '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME","description":"DESCRIPTION"}' --acoountId OWNER_ACCOUNT_ID
```
### Set device data 

*It is important to know that **data** is an **UnorderedMap** so you can input as many key/value items in it without worrying about order.*

```bash
near call $CONTRACT_ACCOUNT_ID set_device_data '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME","data":{"{}":"{}"}}' --acoountId OWNER_ACCOUNT_ID
```

### Get device data

```bash
near view $CONTRACT_ACCOUNT_ID get_device_data '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME"}' --acoountId OWNER_ACCOUNT_ID
```
### Set device data parmeter 

*It is important to know that **param and value** are **Strings**, so you can only input a param & value per function call.*

```bash
near call $CONTRACT_ACCOUNT_ID set_device_data_param '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME","param":"DATA_PARAMETER","value":"VALUE"}' --acoountId OWNER_ACCOUNT_ID
```
### Get device data parameter

```bash
near view $CONTRACT_ACCOUNT_ID get_device_data_param '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME","param":"DATA_PARAMETER"}' --acoountId OWNER_ACCOUNT_ID
```
### Set device metadata 

*It is important to know that **metadata** is an **UnorderedMap** so you can input as many key/value items in it without worrying about order.*

```bash
near call $CONTRACT_ACCOUNT_ID set_device_metadata '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME","metadata":{"{}":"{}"}}' --acoountId OWNER_ACCOUNT_ID
```
### Get device metadata

```bash
near view $CONTRACT_ACCOUNT_ID get_device_metadata '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME"}' --acoountId OWNER_ACCOUNT_ID
```
### Set device metadata parmeter 

*It is important to know that **param and value** are **Strings**, so you can only input a param & value per function call.*

```bash
near call $CONTRACT_ACCOUNT_ID set_device_metadata_param '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME","param":"METADATA_PARAMETER","value":"VALUE"}' --acoountId OWNER_ACCOUNT_ID
```
### Get device metadata parameter

```bash
near view $CONTRACT_ACCOUNT_ID get_device_data_param '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME","param":"METADATA_PARAMETER"}' --acoountId OWNER_ACCOUNT_ID
```

### Unit test

To run tests, run the following command 
```bash
cargo test 
  ```
*Or go to the test code in lib.rs and press *Run test* if using vs code.*

## Examples

### Creating a new registry

```bash
near call $CONTRACT_ACCOUNT_ID create_registry '{"registry name":"GARDEN"}' --acoountId OWNER_ACCOUNT_ID
```
![NewRegistry,1st function](assets/images/Screenshot%20from%202022-08-09%2013-52-16.png)