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
cargo run
```


## Trying ConnectIoT

Before using the smart contract, for a better and easy usage you can set an envorionment variable 
for CONTRACT_ACCOUNT_ID.

### Environment variable

```bash
export CONTRACT_ACCOUNT_ID = <ID in .env file>
```


Create a registry for a device group.

```bash
near call $CONTACT_ACCOUNT_ID create_registry {'"registry name":"Garden"'} --acoountId OWNER_ACCOUNT_ID
```
Add a device to an existing registry

```bash
near call $CONTACT_ACCOUNT_ID add_device_to_registry {'"registry name":"Garden","device_name":"Temperature 1","description":"Thermometer in zone 1"'} --acoountId OWNER_ACCOUNT_ID
```




### Unit test

To run tests, run the following command

```bash
cargo test 
  ```