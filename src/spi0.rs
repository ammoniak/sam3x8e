#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Receive Data Register"]
    pub rdr: RDR,
    #[doc = "0x0c - Transmit Data Register"]
    pub tdr: TDR,
    #[doc = "0x10 - Status Register"]
    pub sr: SR,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x18 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x1c - Interrupt Mask Register"]
    pub imr: IMR,
    _reserved0: [u8; 16usize],
    #[doc = "0x30 - Chip Select Register"]
    pub csr: [CSR; 4],
    _reserved1: [u8; 164usize],
    #[doc = "0xe4 - Write Protection Control Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: WPSR,
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
#[doc = "Receive Data Register"]
pub struct RDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Data Register"]
pub mod rdr;
#[doc = "Transmit Data Register"]
pub struct TDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Data Register"]
pub mod tdr;
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
#[doc = "Chip Select Register"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Chip Select Register"]
pub mod csr;
#[doc = "Write Protection Control Register"]
pub struct WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Control Register"]
pub mod wpmr;
#[doc = "Write Protection Status Register"]
pub struct WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Status Register"]
pub mod wpsr;
