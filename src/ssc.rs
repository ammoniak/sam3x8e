#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Clock Mode Register"]
    pub cmr: CMR,
    _reserved2: [u8; 8usize],
    #[doc = "0x10 - Receive Clock Mode Register"]
    pub rcmr: RCMR,
    #[doc = "0x14 - Receive Frame Mode Register"]
    pub rfmr: RFMR,
    #[doc = "0x18 - Transmit Clock Mode Register"]
    pub tcmr: TCMR,
    #[doc = "0x1c - Transmit Frame Mode Register"]
    pub tfmr: TFMR,
    #[doc = "0x20 - Receive Holding Register"]
    pub rhr: RHR,
    #[doc = "0x24 - Transmit Holding Register"]
    pub thr: THR,
    _reserved8: [u8; 8usize],
    #[doc = "0x30 - Receive Sync. Holding Register"]
    pub rshr: RSHR,
    #[doc = "0x34 - Transmit Sync. Holding Register"]
    pub tshr: TSHR,
    #[doc = "0x38 - Receive Compare 0 Register"]
    pub rc0r: RC0R,
    #[doc = "0x3c - Receive Compare 1 Register"]
    pub rc1r: RC1R,
    #[doc = "0x40 - Status Register"]
    pub sr: SR,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x48 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x4c - Interrupt Mask Register"]
    pub imr: IMR,
    _reserved16: [u8; 148usize],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: WPSR,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Clock Mode Register"]
pub struct CMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Mode Register"]
pub mod cmr;
#[doc = "Receive Clock Mode Register"]
pub struct RCMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Clock Mode Register"]
pub mod rcmr;
#[doc = "Receive Frame Mode Register"]
pub struct RFMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Frame Mode Register"]
pub mod rfmr;
#[doc = "Transmit Clock Mode Register"]
pub struct TCMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Clock Mode Register"]
pub mod tcmr;
#[doc = "Transmit Frame Mode Register"]
pub struct TFMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Frame Mode Register"]
pub mod tfmr;
#[doc = "Receive Holding Register"]
pub struct RHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Holding Register"]
pub mod rhr;
#[doc = "Transmit Holding Register"]
pub struct THR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "Receive Sync. Holding Register"]
pub struct RSHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Sync. Holding Register"]
pub mod rshr;
#[doc = "Transmit Sync. Holding Register"]
pub struct TSHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Sync. Holding Register"]
pub mod tshr;
#[doc = "Receive Compare 0 Register"]
pub struct RC0R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Compare 0 Register"]
pub mod rc0r;
#[doc = "Receive Compare 1 Register"]
pub struct RC1R {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Compare 1 Register"]
pub mod rc1r;
#[doc = "Status Register"]
pub struct SR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Status Register"]
pub mod sr;
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
#[doc = "Write Protect Mode Register"]
pub struct WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Mode Register"]
pub mod wpmr;
#[doc = "Write Protect Status Register"]
pub struct WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Status Register"]
pub mod wpsr;
