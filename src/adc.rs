#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Channel Sequence Register 1"]
    pub seqr1: SEQR1,
    #[doc = "0x0c - Channel Sequence Register 2"]
    pub seqr2: SEQR2,
    #[doc = "0x10 - Channel Enable Register"]
    pub cher: CHER,
    #[doc = "0x14 - Channel Disable Register"]
    pub chdr: CHDR,
    #[doc = "0x18 - Channel Status Register"]
    pub chsr: CHSR,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - Last Converted Data Register"]
    pub lcdr: LCDR,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x30 - Interrupt Status Register"]
    pub isr: ISR,
    _reserved12: [u8; 8usize],
    #[doc = "0x3c - Overrun Status Register"]
    pub over: OVER,
    #[doc = "0x40 - Extended Mode Register"]
    pub emr: EMR,
    #[doc = "0x44 - Compare Window Register"]
    pub cwr: CWR,
    #[doc = "0x48 - Channel Gain Register"]
    pub cgr: CGR,
    #[doc = "0x4c - Channel Offset Register"]
    pub cor: COR,
    #[doc = "0x50 - Channel Data Register"]
    pub cdr: [CDR; 16],
    _reserved18: [u8; 4usize],
    #[doc = "0x94 - Analog Control Register"]
    pub acr: ACR,
    _reserved19: [u8; 76usize],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: WPSR,
    _reserved21: [u8; 20usize],
    #[doc = "0x100 - Receive Pointer Register"]
    pub rpr: RPR,
    #[doc = "0x104 - Receive Counter Register"]
    pub rcr: RCR,
    _reserved23: [u8; 8usize],
    #[doc = "0x110 - Receive Next Pointer Register"]
    pub rnpr: RNPR,
    #[doc = "0x114 - Receive Next Counter Register"]
    pub rncr: RNCR,
    _reserved25: [u8; 8usize],
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
#[doc = "Mode Register"]
pub struct MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod mr;
#[doc = "Channel Sequence Register 1"]
pub struct SEQR1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Sequence Register 1"]
pub mod seqr1;
#[doc = "Channel Sequence Register 2"]
pub struct SEQR2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Sequence Register 2"]
pub mod seqr2;
#[doc = "Channel Enable Register"]
pub struct CHER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Enable Register"]
pub mod cher;
#[doc = "Channel Disable Register"]
pub struct CHDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Disable Register"]
pub mod chdr;
#[doc = "Channel Status Register"]
pub struct CHSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Status Register"]
pub mod chsr;
#[doc = "Last Converted Data Register"]
pub struct LCDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Last Converted Data Register"]
pub mod lcdr;
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
#[doc = "Interrupt Status Register"]
pub struct ISR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Overrun Status Register"]
pub struct OVER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Overrun Status Register"]
pub mod over;
#[doc = "Extended Mode Register"]
pub struct EMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Extended Mode Register"]
pub mod emr;
#[doc = "Compare Window Register"]
pub struct CWR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Compare Window Register"]
pub mod cwr;
#[doc = "Channel Gain Register"]
pub struct CGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Gain Register"]
pub mod cgr;
#[doc = "Channel Offset Register"]
pub struct COR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Offset Register"]
pub mod cor;
#[doc = "Channel Data Register"]
pub struct CDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Data Register"]
pub mod cdr;
#[doc = "Analog Control Register"]
pub struct ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Control Register"]
pub mod acr;
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
