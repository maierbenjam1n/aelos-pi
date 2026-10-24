use bluer::{
    adv::{Advertisement, Type::Peripheral},
    gatt::local::{
        Application, Characteristic, CharacteristicNotify, CharacteristicNotifyMethod,
        CharacteristicRead, CharacteristicWrite, CharacteristicWriteMethod, Service,
    },
};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const SERVICE_UUID: bluer::Uuid = bluer::Uuid::from_u128(0x0000_0000_0000_0000_0000_0000_0000_0001);
const STATUS_UUID: bluer::Uuid = bluer::Uuid::from_u128(0x0000_0000_0000_0000_0000_0000_0000_0002);
const COMMAND_UUID: bluer::Uuid = bluer::Uuid::from_u128(0x0000_0000_0000_0000_0000_0000_0000_0003);

#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;

    adapter.set_powered(true).await?;

    let advertisement = Advertisement {
        advertisement_type: Peripheral,
        service_uuids: [SERVICE_UUID].iter().collect(),
        local_name: Some(String::from("aelos-pi")),
        discoverable: Some(true),
        ..Default::default()
    };

    Ok(())
}
