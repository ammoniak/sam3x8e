#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Mode Register"]
    pub mr: MR,
    #[doc = "0x04 - Alarm Register"]
    pub ar: AR,
    #[doc = "0x08 - Value Register"]
    pub vr: VR,
    #[doc = "0x0c - Status Register"]
    pub sr: SR,
}
#[doc = "Mode Register"]
pub struct MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod mr;
#[doc = "Alarm Register"]
pub struct AR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alarm Register"]
pub mod ar;
#[doc = "Value Register"]
pub struct VR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Value Register"]
pub mod vr;
#[doc = "Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod sr;
