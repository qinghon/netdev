mod db;
pub mod device;
#[cfg(feature = "gateway")]
pub mod gateway;
pub mod interface;
pub mod mac;
mod sys;

pub use device::NetworkDevice;
#[cfg(feature = "gateway")]
pub use gateway::get_default_gateway;
#[cfg(feature = "gateway")]
pub use interface::get_default_interface;
pub use interface::get_interfaces;
pub use interface::Interface;
pub use ipnet;
