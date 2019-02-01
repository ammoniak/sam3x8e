#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Master Mode Register"]
    pub mmr: MMR,
    #[doc = "0x08 - Slave Mode Register"]
    pub smr: SMR,
    #[doc = "0x0c - Internal Address Register"]
    pub iadr: IADR,
    #[doc = "0x10 - Clock Waveform Generator Register"]
    pub cwgr: CWGR,
    _reserved0: [u8; 12usize],
    #[doc = "0x20 - Status Register"]
    pub sr: SR,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x30 - Receive Holding Register"]
    pub rhr: RHR,
    #[doc = "0x34 - Transmit Holding Register"]
    pub thr: THR,
    _reserved1: [u8; 200usize],
    #[doc = "0x100 - Receive Pointer Register"]
    pub rpr: RPR,
    #[doc = "0x104 - Receive Counter Register"]
    pub rcr: RCR,
    #[doc = "0x108 - Transmit Pointer Register"]
    pub tpr: TPR,
    #[doc = "0x10c - Transmit Counter Register"]
    pub tcr: TCR,
    #[doc = "0x110 - Receive Next Pointer Register"]
    pub rnpr: RNPR,
    #[doc = "0x114 - Receive Next Counter Register"]
    pub rncr: RNCR,
    #[doc = "0x118 - Transmit Next Pointer Register"]
    pub tnpr: TNPR,
    #[doc = "0x11c - Transmit Next Counter Register"]
    pub tncr: TNCR,
    #[doc = "0x120 - Transfer Control Register"]
    pub ptcr: PTCR,
    #[doc = "0x124 - Transfer Status Register"]
    pub ptsr: PTSR,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Master Mode Register"]
pub struct MMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Master Mode Register"]
pub mod mmr;
#[doc = "Slave Mode Register"]
pub struct SMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slave Mode Register"]
pub mod smr;
#[doc = "Internal Address Register"]
pub struct IADR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Internal Address Register"]
pub mod iadr;
#[doc = "Clock Waveform Generator Register"]
pub struct CWGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Waveform Generator Register"]
pub mod cwgr;
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
#[doc = "Receive Pointer Register"]
pub struct RPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Pointer Register"]
pub mod rpr;
#[doc = "Receive Counter Register"]
pub struct RCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Counter Register"]
pub mod rcr;
#[doc = "Transmit Pointer Register"]
pub struct TPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Pointer Register"]
pub mod tpr;
#[doc = "Transmit Counter Register"]
pub struct TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Counter Register"]
pub mod tcr;
#[doc = "Receive Next Pointer Register"]
pub struct RNPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Next Pointer Register"]
pub mod rnpr;
#[doc = "Receive Next Counter Register"]
pub struct RNCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Next Counter Register"]
pub mod rncr;
#[doc = "Transmit Next Pointer Register"]
pub struct TNPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Next Pointer Register"]
pub mod tnpr;
#[doc = "Transmit Next Counter Register"]
pub struct TNCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Next Counter Register"]
pub mod tncr;
#[doc = "Transfer Control Register"]
pub struct PTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Control Register"]
pub mod ptcr;
#[doc = "Transfer Status Register"]
pub struct PTSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transfer Status Register"]
pub mod ptsr;
