#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Channel Enable Register"]
    pub cher: CHER,
    #[doc = "0x14 - Channel Disable Register"]
    pub chdr: CHDR,
    #[doc = "0x18 - Channel Status Register"]
    pub chsr: CHSR,
    _reserved1: [u8; 4usize],
    #[doc = "0x20 - Conversion Data Register"]
    pub cdr: CDR,
    #[doc = "0x24 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x28 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x2c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x30 - Interrupt Status Register"]
    pub isr: ISR,
    _reserved2: [u8; 96usize],
    #[doc = "0x94 - Analog Current Register"]
    pub acr: ACR,
    _reserved3: [u8; 76usize],
    #[doc = "0xe4 - Write Protect Mode register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protect Status register"]
    pub wpsr: WPSR,
    _reserved4: [u8; 28usize],
    #[doc = "0x108 - Transmit Pointer Register"]
    pub tpr: TPR,
    #[doc = "0x10c - Transmit Counter Register"]
    pub tcr: TCR,
    _reserved5: [u8; 8usize],
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
#[doc = "Mode Register"]
pub struct MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod mr;
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
#[doc = "Conversion Data Register"]
pub struct CDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Conversion Data Register"]
pub mod cdr;
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
#[doc = "Analog Current Register"]
pub struct ACR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Analog Current Register"]
pub mod acr;
#[doc = "Write Protect Mode register"]
pub struct WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Mode register"]
pub mod wpmr;
#[doc = "Write Protect Status register"]
pub struct WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protect Status register"]
pub mod wpsr;
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
