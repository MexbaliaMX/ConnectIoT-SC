# **ConnectIoT**
<center>

![Arq,use](assets/images/Morado%20Moderno%20Tecnolog%C3%ADa%20y%20Juegos%20Logotipo%20(1).png)

[![Mexbalia](https://img.shields.io/badge/Mexbalia-ConnectIoT%20Smart%20Contract-blue)](https://mexbalia.com/connect-iot/)
[![Mexbalia](https://img.shields.io/badge/-Near%20Protocol-9cf)](https://near.org/)
[![Mexbalia](https://img.shields.io/badge/-near--sdk--rs-orange)](https://www.near-sdk.io/)

</center>

#### Access services **Smart Contract** which allows creating the **connection** between **IoT** devices and the **Near Protocol Blockchain**.
<center>

![Arq,use](assets/images/Screenshot%20from%202022-08-10%2010-16-12.png)

</center>

---
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

---
### Deployment
*You can check the contract ID in the .env file.*
To deploy this smart contract run:
```bash
near dev-deploy ./target/wasm32-unknown-unknown/release/connect_iot.wasm
```


# Trying ConnectIoT

*Before using the smart contract, for a better and easy usage you can set an **envorionment variables** 
for **CONTRACT_ACCOUNT_ID** and **OWNER_ACCOUNT_ID**.*

### Environment variables

```bash
export CONTRACT_ACCOUNT_ID = <ID in .env file>
```
*Your OWNER_ACCOUNT_ID is your testnet account*
```bash
export OWNER_ACCOUNT_ID = <example.testnet> 
```

### Create a registry for a device group.

```bash
near call $CONTRACT_ACCOUNT_ID create_registry '{"registry_name":"REGISTRY_NAME"}' --accountId $OWNER_ACCOUNT_ID
```
### Add a device to an existing registry

```bash
near call $CONTRACT_ACCOUNT_ID add_device_to_registry '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME","description":"DESCRIPTION"}' --accountId $OWNER_ACCOUNT_ID
```
### Set device data 

*It is important to know that **data** is an **UnorderedMap** so you can input as many key/value items in it.*

```bash
near call $CONTRACT_ACCOUNT_ID set_device_data '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME","data":"{\"\":\"\"}"}' --accountId $OWNER_ACCOUNT_ID
```

### Get device data

```bash
near call $CONTRACT_ACCOUNT_ID get_device_data '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME"}' --accountId $OWNER_ACCOUNT_ID
```
### Set device data parameter 

*It is important to know that **param and value** are **Strings**, so you can only input a param & value per function call.*

```bash
near call $CONTRACT_ACCOUNT_ID set_device_data_param '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME","param":"DATA_PARAMETER","value":"VALUE"}' --accountId $OWNER_ACCOUNT_ID
```
### Get device data parameter

```bash
near call $CONTRACT_ACCOUNT_ID get_device_data_param '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME","param":"DATA_PARAMETER"}' --accountId $OWNER_ACCOUNT_ID
```
### Set device metadata 

*It is important to know that **metadata** is an **UnorderedMap** so you can input as many key/value items.*

```bash
near call $CONTRACT_ACCOUNT_ID set_device_metadata '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME","metadata":{"{}":"{}"}}' --accountId $OWNER_ACCOUNT_ID
```
### Get device metadata

```bash
near call $CONTRACT_ACCOUNT_ID get_device_metadata '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME"}' --accountId $OWNER_ACCOUNT_ID
```
### Set device metadata parameter 

*It is important to know that **param and value** are **Strings**, so you can only input a param & value per function call.*

```bash
near call $CONTRACT_ACCOUNT_ID set_device_metadata_param '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME","param":"METADATA_PARAMETER","value":"VALUE"}' --accountId $OWNER_ACCOUNT_ID
```
### Get device metadata parameter

```bash
near call $CONTRACT_ACCOUNT_ID get_device_data_param '{"registry_name":"REGISTRY_NAME","device_name":"DEVICE_NAME","param":"METADATA_PARAMETER"}' --accountId $OWNER_ACCOUNT_ID
```
---
## Unit test

To run tests, run the following command 
```bash
cargo test 
  ```
*Or go to the test code in lib.rs and press *Run test* if using vs code.*

---
## Examples


### Creating a new registry

```bash
near call $CONTRACT_ACCOUNT_ID create_registry '{"registry name":"my_registry"}' --accountId $OWNER_ACCOUNT_ID
```
![NewRegistry,1st function](assets/images/Screenshot%20from%202022-08-09%2016-43-34.png)

*You can check this transaction and its details at the Near Testnet Explorer.*
https://explorer.testnet.near.org/transactions/3tLWruvx3YGHBKuNVZBXspk1PBoryizMuQamqC3Ubq4s

### Adding a new device to registry *"GARDEN 2"*

```bash
near call $CONTRACT_ACCOUNT_ID add_device_to_registry '{"registry_name":"my_registry","device_name":"my_device","description":"Temperature"}' --accountId $OWNER_ACCOUNT_ID
```
![Add New Device, 2nd function](assets/images/Screenshot%20from%202022-08-09%2016-52-59.png)

*You can check this transaction and its details at the Near Testnet Explorer.*
https://explorer.testnet.near.org/transactions/24hztKUH7YW6Xpw8HiT55uqSyqfSzYaszS41A4WzWwso

### Setting device data

```bash
near call $CONTRACT_ACCOUNT_ID set_device_data '{"registry_name":"my_registry","device_name":"my_device","data":"{\"sample1\": \"value1\", \"sample2\": \"value2\", \"sample3\": \"value3\"}"}' --accountId $OWNER_ACCOUNT_ID
```
![Set Device Data,3rd function](assets/images/Screenshot%20from%202022-08-09%2017-00-26.png)

*You can check this transaction and its details at the Near Testnet Explorer.*
https://explorer.testnet.near.org/transactions/EjjP8UNGdQZYv6kKnS8MQ5QNNXPnEWsXdH2BVg5cdSze

### Setting device metadata

```bash
near call $CONTRACT_ACCOUNT_ID set_device_metadata '{"registry_name":"my_registry","device_name":"my_device","metadata":"{\"battery\": \"87%\", \"date\": \"11/12/2019\", \"location\": \"orchard\"}"}' --accountId $OWNER_ACCOUNT_ID
```
![Set Device Metadata, 4th function](assets/images/Screenshot%20from%202022-08-10%2009-49-40.png)

*You can check this transaction and its details at the Near Testnet Explorer.*
https://explorer.testnet.near.org/transactions/AVrMu5s4JtwbT8c2tj8FXCYmyFqsBCi4VhCWebYRACS6

### Getting device data
```bash
near call $CONTRACT_ACCOUNT_ID get_device_data '{"registry_name":"my_registry","device_name":"my_device"}' --accountId $OWNER_ACCOUNT_ID
```
![Get Device Data, 5th function](assets/images/Screenshot%20from%202022-08-10%2012-39-47.png)

*You can check this transaction and its details at the Near Testnet Explorer.*
https://explorer.testnet.near.org/transactions/DJMaTHFopcrGkWQW765Y8eceTbwPo6K58MFKBNyLhhfQ

---
## Credits
<center>

>
  [![Logo Mexbalia](assets/images/Screenshot%20from%202022-08-10%2010-41-59.png)](https://mexbalia.com/)

  [![Logo Near Foundation](assets/images/Screenshot%20from%202022-08-10%2010-38-58.png)](https://near.foundation/)</center>

---
## Support

Reach out via [website](https://mexbalia.com/contact/) or send an email to [info@mexbalia.com](https://google.com)
