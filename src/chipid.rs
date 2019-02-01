#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Chip ID Register"]
    pub cidr: CIDR,
    #[doc = "0x04 - Chip ID Extension Register"]
    pub exid: EXID,
}
#[doc = "Chip ID Register"]
pub struct CIDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip ID Register"]
pub mod cidr;
#[doc = "Chip ID Extension Register"]
pub struct EXID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip ID Extension Register"]
pub mod exid;
