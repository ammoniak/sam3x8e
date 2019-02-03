#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Time Register"]
    pub timr: TIMR,
    #[doc = "0x0c - Calendar Register"]
    pub calr: CALR,
    #[doc = "0x10 - Time Alarm Register"]
    pub timalr: TIMALR,
    #[doc = "0x14 - Calendar Alarm Register"]
    pub calalr: CALALR,
    #[doc = "0x18 - Status Register"]
    pub sr: SR,
    #[doc = "0x1c - Status Clear Command Register"]
    pub sccr: SCCR,
    #[doc = "0x20 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x24 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x28 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x2c - Valid Entry Register"]
    pub ver: VER,
    _reserved12: [u8; 180usize],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Mode Register"]
pub struct MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod mr;
#[doc = "Time Register"]
pub struct TIMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Time Register"]
pub mod timr;
#[doc = "Calendar Register"]
pub struct CALR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calendar Register"]
pub mod calr;
#[doc = "Time Alarm Register"]
pub struct TIMALR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Time Alarm Register"]
pub mod timalr;
#[doc = "Calendar Alarm Register"]
pub struct CALALR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Calendar Alarm Register"]
pub mod calalr;
#[doc = "Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod sr;
#[doc = "Status Clear Command Register"]
pub struct SCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Clear Command Register"]
pub mod sccr;
#[doc = "Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Disable Register"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Mask Register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Valid Entry Register"]
pub struct VER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Valid Entry Register"]
pub mod ver;
#[doc = "Write Protect Mode Register"]
pub struct WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
