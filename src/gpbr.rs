#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose Backup Register"]
    pub gpbr: [GPBR; 8],
}
#[doc = "General Purpose Backup Register"]
pub struct GPBR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "General Purpose Backup Register"]
pub mod gpbr;
