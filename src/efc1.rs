#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EEFC Flash Mode Register"]
    pub fmr: FMR,
    #[doc = "0x04 - EEFC Flash Command Register"]
    pub fcr: FCR,
    #[doc = "0x08 - EEFC Flash Status Register"]
    pub fsr: FSR,
    #[doc = "0x0c - EEFC Flash Result Register"]
    pub frr: FRR,
}
#[doc = "EEFC Flash Mode Register"]
pub struct FMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEFC Flash Mode Register"]
pub mod fmr;
#[doc = "EEFC Flash Command Register"]
pub struct FCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEFC Flash Command Register"]
pub mod fcr;
#[doc = "EEFC Flash Status Register"]
pub struct FSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEFC Flash Status Register"]
pub mod fsr;
#[doc = "EEFC Flash Result Register"]
pub struct FRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EEFC Flash Result Register"]
pub mod frr;
