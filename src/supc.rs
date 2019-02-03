#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Supply Controller Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Supply Controller Supply Monitor Mode Register"]
    pub smmr: SMMR,
    #[doc = "0x08 - Supply Controller Mode Register"]
    pub mr: MR,
    #[doc = "0x0c - Supply Controller Wake-up Mode Register"]
    pub wumr: WUMR,
    #[doc = "0x10 - Supply Controller Wake-up Inputs Register"]
    pub wuir: WUIR,
    #[doc = "0x14 - Supply Controller Status Register"]
    pub sr: SR,
}
#[doc = "Supply Controller Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Supply Controller Control Register"]
pub mod cr;
#[doc = "Supply Controller Supply Monitor Mode Register"]
pub struct SMMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Supply Controller Supply Monitor Mode Register"]
pub mod smmr;
#[doc = "Supply Controller Mode Register"]
pub struct MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Supply Controller Mode Register"]
pub mod mr;
#[doc = "Supply Controller Wake-up Mode Register"]
pub struct WUMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Supply Controller Wake-up Mode Register"]
pub mod wumr;
#[doc = "Supply Controller Wake-up Inputs Register"]
pub struct WUIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Supply Controller Wake-up Inputs Register"]
pub mod wuir;
#[doc = "Supply Controller Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Supply Controller Status Register"]
pub mod sr;
