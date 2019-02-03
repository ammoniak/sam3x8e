#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "Control Register"]
    pub cr: CR_UNION,
    #[doc = "Mode Register"]
    pub mr: MR_UNION,
    #[doc = "Interrupt Enable Register"]
    pub ier: IER_UNION,
    #[doc = "Interrupt Disable Register"]
    pub idr: IDR_UNION,
    #[doc = "Interrupt Mask Register"]
    pub imr: IMR_UNION,
    #[doc = "Channel Status Register"]
    pub csr: CSR_UNION,
    #[doc = "0x18 - Receive Holding Register"]
    pub rhr: RHR,
    #[doc = "0x1c - Transmit Holding Register"]
    pub thr: THR,
    #[doc = "0x20 - Baud Rate Generator Register"]
    pub brgr: BRGR,
    #[doc = "0x24 - Receiver Time-out Register"]
    pub rtor: RTOR,
    #[doc = "0x28 - Transmitter Timeguard Register"]
    pub ttgr: TTGR,
    _reserved11: [u8; 20usize],
    #[doc = "0x40 - FI DI Ratio Register"]
    pub fidi: FIDI,
    #[doc = "0x44 - Number of Errors Register"]
    pub ner: NER,
    _reserved13: [u8; 4usize],
    #[doc = "0x4c - IrDA Filter Register"]
    pub if_: IF,
    #[doc = "0x50 - Manchester Configuration Register"]
    pub man: MAN,
    #[doc = "0x54 - LIN Mode Register"]
    pub linmr: LINMR,
    #[doc = "0x58 - LIN Identifier Register"]
    pub linir: LINIR,
    #[doc = "0x5c - LIN Baud Rate Register"]
    pub linbrr: LINBRR,
    _reserved18: [u8; 132usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: WPSR,
    _reserved20: [u8; 20usize],
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
#[repr(C)]
pub union CR_UNION {
    #[doc = "0x00 - Control Register"]
    pub cr_spi_mode: CR_SPI_MODE,
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
}
#[doc = "Mode Register"]
#[repr(C)]
pub union MR_UNION {
    #[doc = "0x04 - Mode Register"]
    pub mr_spi_mode: MR_SPI_MODE,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
}
#[doc = "Interrupt Enable Register"]
#[repr(C)]
pub union IER_UNION {
    #[doc = "0x08 - Interrupt Enable Register"]
    pub ier_lin_mode: IER_LIN_MODE,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub ier_spi_mode: IER_SPI_MODE,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub ier: IER,
}
#[doc = "Interrupt Disable Register"]
#[repr(C)]
pub union IDR_UNION {
    #[doc = "0x0c - Interrupt Disable Register"]
    pub idr_lin_mode: IDR_LIN_MODE,
    #[doc = "0x0c - Interrupt Disable Register"]
    pub idr_spi_mode: IDR_SPI_MODE,
    #[doc = "0x0c - Interrupt Disable Register"]
    pub idr: IDR,
}
#[doc = "Interrupt Mask Register"]
#[repr(C)]
pub union IMR_UNION {
    #[doc = "0x10 - Interrupt Mask Register"]
    pub imr_lin_mode: IMR_LIN_MODE,
    #[doc = "0x10 - Interrupt Mask Register"]
    pub imr_spi_mode: IMR_SPI_MODE,
    #[doc = "0x10 - Interrupt Mask Register"]
    pub imr: IMR,
}
#[doc = "Channel Status Register"]
#[repr(C)]
pub union CSR_UNION {
    #[doc = "0x14 - Channel Status Register"]
    pub csr_lin_mode: CSR_LIN_MODE,
    #[doc = "0x14 - Channel Status Register"]
    pub csr_spi_mode: CSR_SPI_MODE,
    #[doc = "0x14 - Channel Status Register"]
    pub csr: CSR,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Control Register"]
pub struct CR_SPI_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr_spi_mode;
#[doc = "Mode Register"]
pub struct MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod mr;
#[doc = "Mode Register"]
pub struct MR_SPI_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Mode Register"]
pub mod mr_spi_mode;
#[doc = "Interrupt Enable Register"]
pub struct IER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Interrupt Enable Register"]
pub struct IER_SPI_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier_spi_mode;
#[doc = "Interrupt Enable Register"]
pub struct IER_LIN_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ier_lin_mode;
#[doc = "Interrupt Disable Register"]
pub struct IDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "Interrupt Disable Register"]
pub struct IDR_SPI_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod idr_spi_mode;
#[doc = "Interrupt Disable Register"]
pub struct IDR_LIN_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod idr_lin_mode;
#[doc = "Interrupt Mask Register"]
pub struct IMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "Interrupt Mask Register"]
pub struct IMR_SPI_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod imr_spi_mode;
#[doc = "Interrupt Mask Register"]
pub struct IMR_LIN_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod imr_lin_mode;
#[doc = "Channel Status Register"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Status Register"]
pub mod csr;
#[doc = "Channel Status Register"]
pub struct CSR_SPI_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Status Register"]
pub mod csr_spi_mode;
#[doc = "Channel Status Register"]
pub struct CSR_LIN_MODE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Channel Status Register"]
pub mod csr_lin_mode;
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
#[doc = "Manchester Configuration Register"]
pub struct MAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Manchester Configuration Register"]
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
#[doc = "LIN Baud Rate Register"]
pub struct LINBRR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "LIN Baud Rate Register"]
pub mod linbrr;
#[doc = "Write Protection Mode Register"]
pub struct WPMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
#[doc = "Write Protection Status Register"]
pub struct WPSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Write Protection Status Register"]
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
