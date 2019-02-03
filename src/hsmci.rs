#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Mode Register"]
    pub mr: MR,
    #[doc = "0x08 - Data Timeout Register"]
    pub dtor: DTOR,
    #[doc = "0x0c - SD/SDIO Card Register"]
    pub sdcr: SDCR,
    #[doc = "0x10 - Argument Register"]
    pub argr: ARGR,
    #[doc = "0x14 - Command Register"]
    pub cmdr: CMDR,
    #[doc = "0x18 - Block Register"]
    pub blkr: BLKR,
    #[doc = "0x1c - Completion Signal Timeout Register"]
    pub cstor: CSTOR,
    #[doc = "0x20 - Response Register"]
    pub rspr: [RSPR; 4],
    #[doc = "0x30 - Receive Data Register"]
    pub rdr: RDR,
    #[doc = "0x34 - Transmit Data Register"]
    pub tdr: TDR,
    _reserved11: [u8; 8usize],
    #[doc = "0x40 - Status Register"]
    pub sr: SR,
    #[doc = "0x44 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x48 - Interrupt Disable Register"]
    pub idr: IDR,
    #[doc = "0x4c - Interrupt Mask Register"]
    pub imr: IMR,
    #[doc = "0x50 - DMA Configuration Register"]
    pub dma: DMA,
    #[doc = "0x54 - Configuration Register"]
    pub cfg: CFG,
    _reserved17: [u8; 140usize],
    #[doc = "0xe4 - Write Protection Mode Register"]
    pub wpmr: WPMR,
    #[doc = "0xe8 - Write Protection Status Register"]
    pub wpsr: WPSR,
    _reserved19: [u8; 276usize],
    #[doc = "0x200 - FIFO Memory Aperture0"]
    pub fifo: [FIFO; 256],
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
#[doc = "Data Timeout Register"]
pub struct DTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data Timeout Register"]
pub mod dtor;
#[doc = "SD/SDIO Card Register"]
pub struct SDCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SD/SDIO Card Register"]
pub mod sdcr;
#[doc = "Argument Register"]
pub struct ARGR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Argument Register"]
pub mod argr;
#[doc = "Command Register"]
pub struct CMDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmdr;
#[doc = "Block Register"]
pub struct BLKR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Block Register"]
pub mod blkr;
#[doc = "Completion Signal Timeout Register"]
pub struct CSTOR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Completion Signal Timeout Register"]
pub mod cstor;
#[doc = "Response Register"]
pub struct RSPR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Response Register"]
pub mod rspr;
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
#[doc = "DMA Configuration Register"]
pub struct DMA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Configuration Register"]
pub mod dma;
#[doc = "Configuration Register"]
pub struct CFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration Register"]
pub mod cfg;
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
#[doc = "FIFO Memory Aperture0"]
pub struct FIFO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "FIFO Memory Aperture0"]
pub mod fifo;
