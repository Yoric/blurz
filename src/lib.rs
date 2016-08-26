extern crate dbus;
extern crate rustc_serialize;

pub use bluetooth_adapter::BluetoothAdapter;
pub use bluetooth_device::BluetoothDevice;
pub use bluetooth_gatt_characteristic::BluetoothGATTCharacteristic;
pub use bluetooth_gatt_descriptor::BluetoothGATTDescriptor;
pub use bluetooth_gatt_service::BluetoothGATTService;
pub use bluetooth_discovery_session::BluetoothDiscoverySession;

/// Representation of a remote device.
pub mod bluetooth_device;
pub mod bluetooth_adapter;
pub mod bluetooth_gatt_characteristic;
pub mod bluetooth_gatt_descriptor;
pub mod bluetooth_gatt_service;
pub mod bluetooth_discovery_session;
mod bluetooth_utils;
