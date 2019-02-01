#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x0c - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x10 - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x14 - Channel Status Register"]
    pub csr: CSR,
    #[doc = "0x18 - Receiver Holding Register"]
    pub rhr: RHR,
    #[doc = "0x1c - Transmitter Holding Register"]
    pub thr: THR,
    #[doc = "0x20 - Baud Rate Generator Register"]
    pub brgr: BRGR,
    #[doc = "0x24 - Receiver Time-out Register"]
    pub rtor: RTOR,
    #[doc = "0x28 - Transmitter Timeguard Register"]
    pub ttgr: TTGR,
    _reserved0: [u8; 20usize],
    #[doc = "0x40 - FI DI Ratio Register"]
    pub fidi: FIDI,
    #[doc = "0x44 - Number of Errors Register"]
    pub ner: NER,
    _reserved1: [u8; 4usize],
    #[doc = "0x4c - IrDA Filter Register"]
    pub if_: IF,
    #[doc = "0x50 - Manchester Encoder Decoder Register"]
    pub man: MAN,
    #[doc = "0x54 - LIN Mode Register"]
    pub linmr: LINMR,
    #[doc = "0x58 - LIN Identifier Register"]
    pub linir: LINIR,
    _reserved2: [u8; 136usize],
    #[doc = "0xe4 - Write Protect Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protect Status Register"]
    pub wpsr: WPSR,
    _reserved3: [u8; 20usize],
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
#[doc = "Mode Register"]
pub struct MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod mr;
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
#[doc = "Channel Status Register"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Status Register"]
pub mod csr;
#[doc = "Receiver Holding Register"]
pub struct RHR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Holding Register"]
pub mod rhr;
#[doc = "Transmitter Holding Register"]
pub struct THR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmitter Holding Register"]
pub mod thr;
#[doc = "Baud Rate Generator Register"]
pub struct BRGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Baud Rate Generator Register"]
pub mod brgr;
#[doc = "Receiver Time-out Register"]
pub struct RTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receiver Time-out Register"]
pub mod rtor;
#[doc = "Transmitter Timeguard Register"]
pub struct TTGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmitter Timeguard Register"]
pub mod ttgr;
#[doc = "FI DI Ratio Register"]
pub struct FIDI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FI DI Ratio Register"]
pub mod fidi;
#[doc = "Number of Errors Register"]
pub struct NER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Number of Errors Register"]
pub mod ner;
#[doc = "IrDA Filter Register"]
pub struct IF {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IrDA Filter Register"]
pub mod if_;
#[doc = "Manchester Encoder Decoder Register"]
pub struct MAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Manchester Encoder Decoder Register"]
pub mod man;
#[doc = "LIN Mode Register"]
pub struct LINMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LIN Mode Register"]
pub mod linmr;
#[doc = "LIN Identifier Register"]
pub struct LINIR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LIN Identifier Register"]
pub mod linir;
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
